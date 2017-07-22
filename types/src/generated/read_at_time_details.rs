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

#[derive(Debug, Clone, PartialEq)]
pub struct ReadAtTimeDetails {
    pub req_times: Option<Vec<DateTime>>,
    pub use_simple_bounds: Boolean,
}

impl BinaryEncoder<ReadAtTimeDetails> for ReadAtTimeDetails {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += byte_len_array(&self.req_times);
        size += self.use_simple_bounds.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += write_array(stream, &self.req_times)?;
        size += self.use_simple_bounds.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let req_times: Option<Vec<DateTime>> = read_array(stream)?;
        let use_simple_bounds = Boolean::decode(stream)?;
        Ok(ReadAtTimeDetails {
            req_times,
            use_simple_bounds,
        })
    }
}
