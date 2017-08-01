// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
#[allow(unused_imports)]
use data_types::*;
#[allow(unused_imports)]
use data_value::*;
#[allow(unused_imports)]
use attribute::*;
#[allow(unused_imports)]
use date_time::*;
#[allow(unused_imports)]
use node_id::*;
#[allow(unused_imports)]
use service_types::*;
#[allow(unused_imports)]
use variant::*;
#[allow(unused_imports)]
use generated::node_ids::*;
#[allow(unused_imports)]
use generated::status_codes::StatusCode;
#[allow(unused_imports)]
use generated::status_codes::StatusCode::*;
use generated::ViewDescription;
use generated::NodeTypeDescription;
use generated::ContentFilter;

#[derive(Debug, Clone, PartialEq)]
pub struct QueryFirstRequest {
    pub request_header: RequestHeader,
    pub view: ViewDescription,
    pub node_types: Option<Vec<NodeTypeDescription>>,
    pub filter: ContentFilter,
    pub max_data_sets_to_return: UInt32,
    pub max_references_to_return: UInt32,
}

impl MessageInfo for QueryFirstRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::QueryFirstRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<QueryFirstRequest> for QueryFirstRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.view.byte_len();
        size += byte_len_array(&self.node_types);
        size += self.filter.byte_len();
        size += self.max_data_sets_to_return.byte_len();
        size += self.max_references_to_return.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.view.encode(stream)?;
        size += write_array(stream, &self.node_types)?;
        size += self.filter.encode(stream)?;
        size += self.max_data_sets_to_return.encode(stream)?;
        size += self.max_references_to_return.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream)?;
        let view = ViewDescription::decode(stream)?;
        let node_types: Option<Vec<NodeTypeDescription>> = read_array(stream)?;
        let filter = ContentFilter::decode(stream)?;
        let max_data_sets_to_return = UInt32::decode(stream)?;
        let max_references_to_return = UInt32::decode(stream)?;
        Ok(QueryFirstRequest {
            request_header,
            view,
            node_types,
            filter,
            max_data_sets_to_return,
            max_references_to_return,
        })
    }
}