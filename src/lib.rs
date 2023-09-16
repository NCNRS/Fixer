use serde::{Deserialize, Serialize};

/// Messages that get serialized by a service and passed along
/// by the Fixer.
#[derive(Debug, Serialize, Deserialize)]
pub struct FixerMsg {
    pub kind: MsgType,
}

/// The kind of Message that is being sent. 
/// We often use `match` on this type and then
/// process the msg based on it's type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MsgType {
    QuickHack
}