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

/// Registers one or more nodes for repeated use within a session.
#[derive(Debug, Clone, PartialEq)]
pub struct RegisterNodesRequest {
    pub request_header: RequestHeader,
    pub nodes_to_register: Option<Vec<NodeId>>,
}

impl MessageInfo for RegisterNodesRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::RegisterNodesRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<RegisterNodesRequest> for RegisterNodesRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += byte_len_array(&self.nodes_to_register);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += write_array(stream, &self.nodes_to_register)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream)?;
        let nodes_to_register: Option<Vec<NodeId>> = read_array(stream)?;
        Ok(RegisterNodesRequest {
            request_header,
            nodes_to_register,
        })
    }
}
