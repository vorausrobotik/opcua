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
pub struct DataChangeFilter {
    pub trigger: DataChangeTrigger,
    pub deadband_type: UInt32,
    pub deadband_value: Double,
}

impl BinaryEncoder<DataChangeFilter> for DataChangeFilter {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.trigger.byte_len();
        size += self.deadband_type.byte_len();
        size += self.deadband_value.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.trigger.encode(stream)?;
        size += self.deadband_type.encode(stream)?;
        size += self.deadband_value.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let trigger = DataChangeTrigger::decode(stream)?;
        let deadband_type = UInt32::decode(stream)?;
        let deadband_value = Double::decode(stream)?;
        Ok(DataChangeFilter {
            trigger,
            deadband_type,
            deadband_value,
        })
    }
}
