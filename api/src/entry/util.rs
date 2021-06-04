use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum MediaHref<'a> {
    Internal { path: &'a str },
    External { url: &'a str },
}
