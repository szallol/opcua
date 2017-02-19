use std;
use std::io::{Cursor};

use comms::*;
use types::*;
use debug::*;

pub struct Chunker {}

impl Chunker {
    fn chunk_message_type(message: &SupportedMessage) -> ChunkMessageType {
        match *message {
            SupportedMessage::OpenSecureChannelRequest(_) | SupportedMessage::OpenSecureChannelResponse(_) => ChunkMessageType::OpenSecureChannel,
            SupportedMessage::CloseSecureChannelRequest(_) | SupportedMessage::CloseSecureChannelResponse(_) => ChunkMessageType::CloseSecureChannel,
            _ => ChunkMessageType::Message
        }
    }

    /// Encodes a message using the supplied sequence number and secure channel info and emits the corresponding chunks
    pub fn encode(sequence_number: UInt32, request_id: UInt32, secure_channel_info: &SecureChannelInfo, supported_message: &SupportedMessage) -> std::result::Result<Vec<Chunk>, &'static StatusCode> {
        // TODO multiple chunks

        // External values
        let secure_channel_id = secure_channel_info.secure_channel_id;

        debug!("Creating a chunk for secure channel id {}, sequence id {}", secure_channel_id, sequence_number);

        let message_type = Chunker::chunk_message_type(supported_message);

        let is_first_chunk = true;
        let is_last_chunk = true;
        let chunk_type = if is_last_chunk { ChunkType::Final } else { ChunkType::Intermediate };

        // security header depends on message type
        let security_header = if message_type == ChunkMessageType::OpenSecureChannel {
            SecurityHeader::Asymmetric(AsymmetricSecurityHeader::none())
        } else {
            SecurityHeader::Symmetric(SymmetricSecurityHeader {
                token_id: secure_channel_info.token_id,
            })
        };

        let sequence_header = SequenceHeader {
            sequence_number: sequence_number,
            request_id: request_id,
        };

        let node_id = supported_message.node_id();

        // Calculate the chunk body size
        let mut chunk_body_size = 0;
        chunk_body_size += security_header.byte_len();
        chunk_body_size += sequence_header.byte_len();
        if is_first_chunk {
            // Write a node id
            chunk_body_size += node_id.byte_len();
        }
        chunk_body_size += supported_message.byte_len();
        // TODO encrypted message size
        // chunk_body_size += 1; // padding size byte when padding
        // TODO signature size

        let message_size = (CHUNK_HEADER_SIZE + chunk_body_size) as u32;

        debug!("Creating a chunk with a size of {}", message_size);

        let chunk_header = ChunkHeader {
            message_type: message_type,
            chunk_type: chunk_type,
            message_size: message_size,
            secure_channel_id: secure_channel_id,
            is_valid: true,
        };

        let mut stream = Cursor::new(vec![0u8; chunk_body_size]);

        // write security header
        let _ = security_header.encode(&mut stream);
        // write sequence header
        let _ = sequence_header.encode(&mut stream);
        // Write a node id for the first chunk
        if is_first_chunk {
            debug!("Encoding node id {:?}", node_id);
            let _ = node_id.encode(&mut stream);
        } else {}
        // write message
        let _ = supported_message.encode(&mut stream);
        // write padding byte (0 since there is no padding bytes)
        // write_u8(&mut stream, 0u8);

        // TODO write padding
        // TODO encrypt
        // TODO calculate signature
        // TODO write signature

        // Now the chunk is made and can be added to the result
        let chunk = Chunk {
            chunk_header: chunk_header,
            chunk_body: stream.into_inner(),
        };

        Ok(vec![chunk])
    }

    /// Decodes a series of chunks to create a message
    pub fn decode(chunks: &Vec<Chunk>, secure_channel_info: &SecureChannelInfo, expected_node_id: Option<NodeId>) -> std::result::Result<SupportedMessage, &'static StatusCode> {
        if chunks.len() != 1 {
            // TODO more than one chunk is not supported yet
            // TODO decoding multiple chunks means validating their headers, decrypting them to a buffer and stitching them together
            error!("Only one chunk is supported");
            return Err(&BAD_UNEXPECTED_ERROR);
        }

        let chunk = &chunks[0];

        let is_first_chunk = true;
        let chunk_info = chunk.chunk_info(is_first_chunk, secure_channel_info)?;
        debug!("Chunker::decode chunk_info = {:?}", chunk_info);

        let body_start = chunk_info.body_offset;
        let body_end = body_start + chunk_info.body_length;
        let chunk_body = &chunk.chunk_body[body_start..body_end];
        debug!("chunk_message_body:");
        debug_buffer(chunk_body);

        // First chunk has an extension object prefix.
        //
        // The extension object prefix is just the node id. A point the spec rather unhelpfully doesn't
        // elaborate on. Probably because people enjoy debugging why the stream pos is out by 1 byte
        // for hours.

        let object_id = if let Some(ref node_id) = chunk_info.node_id {
            let valid_node_id = if node_id.namespace != 0 || !node_id.is_numeric() {
                // Must be ns 0 and numeric
                false
            } else if expected_node_id.is_some() {
                expected_node_id.unwrap() == *node_id
            } else {
                true
            };
            if !valid_node_id {
                error!("The node id read from the stream was not accepted in this context {:?}", node_id);
                return Err(&BAD_UNEXPECTED_ERROR);
            }
            let object_id = node_id.as_object_id();
            if object_id.is_err() {
                error!("The node was not an object id");
                return Err(&BAD_UNEXPECTED_ERROR);
            }
            let object_id = object_id.unwrap();
            debug!("Decoded node id / object id of {:?}", object_id);
            object_id
        } else {
            return Err(&BAD_TCP_MESSAGE_TYPE_INVALID);
        };

        // Now the payload. The node id of the prefix allows us to recognize it.

        let mut chunk_body_stream = &mut Cursor::new(chunk_body);

        let decoded_message = SupportedMessage::decode_by_object_id(&mut chunk_body_stream, object_id);
        if decoded_message.is_err() {
            debug!("Can't decode message {:?}", object_id);
            return Err(&BAD_SERVICE_UNSUPPORTED)
        }
        let decoded_message = decoded_message.unwrap();
        if let SupportedMessage::Invalid(_) = decoded_message {
            debug!("Message {:?} is unsupported", object_id);
            return Err(&BAD_SERVICE_UNSUPPORTED);
        }

        debug!("Returning decoded msg {:?}", decoded_message);
        return Ok(decoded_message)
    }
}