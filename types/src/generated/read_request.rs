// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use {BinaryEncoder, EncodingResult};
#[allow(unused_imports)]
use basic_types::*;
#[allow(unused_imports)]
use data_types::*;
#[allow(unused_imports)]
use data_value::*;
#[allow(unused_imports)]
use helpers::*;
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
use generated::ReadValueId;

#[derive(Debug, Clone, PartialEq)]
pub struct ReadRequest {
    pub request_header: RequestHeader,
    pub max_age: Double,
    pub timestamps_to_return: TimestampsToReturn,
    pub nodes_to_read: Option<Vec<ReadValueId>>,
}

impl MessageInfo for ReadRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::ReadRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ReadRequest> for ReadRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.max_age.byte_len();
        size += self.timestamps_to_return.byte_len();
        size += byte_len_array(&self.nodes_to_read);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.max_age.encode(stream)?;
        size += self.timestamps_to_return.encode(stream)?;
        size += write_array(stream, &self.nodes_to_read)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream)?;
        let max_age = Double::decode(stream)?;
        let timestamps_to_return = TimestampsToReturn::decode(stream)?;
        let nodes_to_read: Option<Vec<ReadValueId>> = read_array(stream)?;
        Ok(ReadRequest {
            request_header,
            max_age,
            timestamps_to_return,
            nodes_to_read,
        })
    }
}
