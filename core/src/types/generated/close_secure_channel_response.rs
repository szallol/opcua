// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

/// Closes a secure channel.
#[derive(Debug, Clone, PartialEq)]
pub struct CloseSecureChannelResponse {
    pub response_header: ResponseHeader,
}

impl MessageInfo for CloseSecureChannelResponse {
    fn object_id(&self) -> ObjectId {
        ObjectId::CloseSecureChannelResponse_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<CloseSecureChannelResponse> for CloseSecureChannelResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let response_header = ResponseHeader::decode(stream)?;
        Ok(CloseSecureChannelResponse {
            response_header: response_header,
        })
    }
}
