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
use generated::AddReferencesItem;

/// Adds one or more references to the server address space.
#[derive(Debug, Clone, PartialEq)]
pub struct AddReferencesRequest {
    pub request_header: RequestHeader,
    pub references_to_add: Option<Vec<AddReferencesItem>>,
}

impl MessageInfo for AddReferencesRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::AddReferencesRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<AddReferencesRequest> for AddReferencesRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += byte_len_array(&self.references_to_add);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += write_array(stream, &self.references_to_add)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream)?;
        let references_to_add: Option<Vec<AddReferencesItem>> = read_array(stream)?;
        Ok(AddReferencesRequest {
            request_header,
            references_to_add,
        })
    }
}
