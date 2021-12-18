use crate::block::block_object::{OptionBlockObject, TextBlockObject};
use serde::{Deserialize, Serialize};

use std::fmt::Debug;

#[typetag::serde(tag = "type")]
pub trait Block: Debug {}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct BlockAction {
    pub action_id: Option<String>,
    pub block_id: Option<String>,
    pub text: TextBlockObject,
    pub value: Option<String>,
    pub actions_ts: Option<String>,
    pub selected_option: OptionBlockObject,
    pub selected_options: Vec<OptionBlockObject>,
    pub selected_user: Option<String>,
    pub selected_users: Vec<String>,
    pub selected_channel: Option<String>,
    pub selected_channels: Vec<String>,
    pub selected_conversation: Option<String>,
    pub selected_conversations: Vec<String>,
    pub selected_date: Option<String>,
    pub selected_time: Option<String>,
    pub initial_option: OptionBlockObject,
    pub initial_user: Option<String>,
    pub initial_channel: Option<String>,
    pub initial_conversation: Option<String>,
    pub initial_date: Option<String>,
    pub initial_time: Option<String>,
}

#[typetag::serde(name = "actions")]
impl Block for BlockAction {}
