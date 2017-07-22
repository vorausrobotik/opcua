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

/// A request to add a node to the server address space.
#[derive(Debug, Clone, PartialEq)]
pub struct AddNodesItem {
    pub parent_node_id: ExpandedNodeId,
    pub reference_type_id: NodeId,
    pub requested_new_node_id: ExpandedNodeId,
    pub browse_name: QualifiedName,
    pub node_class: NodeClass,
    pub node_attributes: ExtensionObject,
    pub type_definition: ExpandedNodeId,
}

impl MessageInfo for AddNodesItem {
    fn object_id(&self) -> ObjectId {
        ObjectId::AddNodesItem_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<AddNodesItem> for AddNodesItem {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.parent_node_id.byte_len();
        size += self.reference_type_id.byte_len();
        size += self.requested_new_node_id.byte_len();
        size += self.browse_name.byte_len();
        size += self.node_class.byte_len();
        size += self.node_attributes.byte_len();
        size += self.type_definition.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.parent_node_id.encode(stream)?;
        size += self.reference_type_id.encode(stream)?;
        size += self.requested_new_node_id.encode(stream)?;
        size += self.browse_name.encode(stream)?;
        size += self.node_class.encode(stream)?;
        size += self.node_attributes.encode(stream)?;
        size += self.type_definition.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let parent_node_id = ExpandedNodeId::decode(stream)?;
        let reference_type_id = NodeId::decode(stream)?;
        let requested_new_node_id = ExpandedNodeId::decode(stream)?;
        let browse_name = QualifiedName::decode(stream)?;
        let node_class = NodeClass::decode(stream)?;
        let node_attributes = ExtensionObject::decode(stream)?;
        let type_definition = ExpandedNodeId::decode(stream)?;
        Ok(AddNodesItem {
            parent_node_id,
            reference_type_id,
            requested_new_node_id,
            browse_name,
            node_class,
            node_attributes,
            type_definition,
        })
    }
}
