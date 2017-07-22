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
pub struct EUInformation {
    pub namespace_uri: UAString,
    pub unit_id: Int32,
    pub display_name: LocalizedText,
    pub description: LocalizedText,
}

impl MessageInfo for EUInformation {
    fn object_id(&self) -> ObjectId {
        ObjectId::EUInformation_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<EUInformation> for EUInformation {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.namespace_uri.byte_len();
        size += self.unit_id.byte_len();
        size += self.display_name.byte_len();
        size += self.description.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.namespace_uri.encode(stream)?;
        size += self.unit_id.encode(stream)?;
        size += self.display_name.encode(stream)?;
        size += self.description.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let namespace_uri = UAString::decode(stream)?;
        let unit_id = Int32::decode(stream)?;
        let display_name = LocalizedText::decode(stream)?;
        let description = LocalizedText::decode(stream)?;
        Ok(EUInformation {
            namespace_uri,
            unit_id,
            display_name,
            description,
        })
    }
}
