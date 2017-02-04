// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

/// The result of a browse operation.
#[derive(Debug, Clone, PartialEq)]
pub struct BrowseResult {
    pub status_code: StatusCode,
    pub continuation_point: ByteString,
    pub references: Option<Vec<ReferenceDescription>>,
}

impl MessageInfo for BrowseResult {
    fn object_id(&self) -> ObjectId {
        ObjectId::BrowseResult_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<BrowseResult> for BrowseResult {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.status_code.byte_len();
        size += self.continuation_point.byte_len();
        size += byte_len_array(&self.references);
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.status_code.encode(stream)?;
        size += self.continuation_point.encode(stream)?;
        size += write_array(stream, &self.references)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let status_code = StatusCode::decode(stream)?;
        let continuation_point = ByteString::decode(stream)?;
        let references: Option<Vec<ReferenceDescription>> = read_array(stream)?;
        Ok(BrowseResult {
            status_code: status_code,
            continuation_point: continuation_point,
            references: references,
        })
    }
}
