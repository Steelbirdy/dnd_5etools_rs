use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Speed<'a> {
    #[serde(borrow)]
    Custom(CustomSpeed<'a>),
    Simple(i64),
    #[serde(rename = "varies")]
    Varies,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomSpeed<'a> {
    #[serde(flatten, borrow)]
    pub speeds: HashMap<SpeedKind, SpeedVal<'a>>,
    pub can_hover: Option<bool>,
    pub choose: Option<CustomSpeedChoose<'a>>,
    pub alternate: HashMap<SpeedKind, Vec<SpeedVal<'a>>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct CustomSpeedChoose<'a> {
    pub from: Vec<SpeedKind>,
    pub amount: i64,
    pub note: Option<&'a str>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SpeedVal<'a> {
    Conditional { number: i64, condition: &'a str },
    Number(i64),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SpeedKind {
    Walk,
    Burrow,
    Climb,
    Fly,
    Swim,
}
