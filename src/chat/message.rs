use serde::{Deserialize, Serialize};

use crate::attachments::attachment::Attachment;

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    pub text: Option<String>,
    pub username: Option<String>,
    pub bot_id: Option<String>,
    pub attachments: Option<Vec<Attachment>>,
    pub r#type: Option<String>,
    pub subtype: Option<String>,
    pub ts: Option<String>,
}
