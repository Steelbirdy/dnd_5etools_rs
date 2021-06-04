#![allow(dead_code)]

use super::*;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source<'a> {
    pub source: &'a str,
    pub page: Option<i16>,
}

#[allow(dead_code)]
pub type AdditionalSources<'a> = Vec<Source<'a>>;

#[allow(dead_code)]
pub type OtherSources<'a> = Vec<Source<'a>>;
