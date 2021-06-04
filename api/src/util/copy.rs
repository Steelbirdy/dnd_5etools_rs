use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopyBlock<'a> {
    pub name: &'a str,
    pub source: &'a str,
    /// Used in deity data
    pub pantheon: Option<&'a str>,
    pub _mod: Option<HashMap<&'a str, CopyBlockMod<'a>>>,
    pub _trait: Option<CopyBlockTrait<'a>>,
    pub _preserve: Option<HashMap<&'a str, bool>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "mode")]
pub enum CopyModifier<'a> {
    ReplaceTxt {
        replace: &'a str,
        with: &'a str,
        flags: Option<&'a str>,
    },
    AppendStr {
        str: &'a str,
        joiner: Option<&'a str>,
    },
    PrependArr {
        /// [Value::Array|String|Object]
        items: Value,
    },
    AppendArr {
        /// [Value::Array|String|Object]
        items: Value,
    },
    AppendIfNotExistsArr {
        /// [Value::Array|String|Object]
        items: Value,
    },
    ReplaceArr {
        replace: CopyModifierReplaceKind<'a>,
        /// [Value::Array|Object]
        items: Value,
    },
    ReplaceOrAppendArr {
        replace: CopyModifierReplaceKind<'a>,
        /// [Value::Array|Object]
        items: Value,
    },
    InsertArr {
        index: usize,
        items: Value,
    },
    RemoveArr {
        /// [Value::Array|String]
        names: Option<Value>,
        /// [Value::Array|String]
        items: Option<Value>,
        force: Option<bool>,
    },
    CalculateProp {
        prop: &'a str,
        formula: &'a str,
    },
    ReplaceSpells {
        /// [Value::Object]
        spells: Option<Value>,
        /// [Value::Object]
        daily: Option<Value>,
    },
    AddSpells {
        /// [Value::Object]
        spells: Option<Value>,
        will: Option<Vec<Value>>,
        /// [Value::Object]
        daily: Option<Value>,
    },
    AddSkills {
        /// [Value::Object]
        skills: Option<Value>,
    },
}

#[skip_serializing_none]
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CopyModifierReplaceKind<'a> {
    String(&'a str),
    Index {
        index: usize,
    },
    Regex {
        regex: &'a str,
        flags: Option<&'a str>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CopyBlockMod<'a> {
    String(&'a str),
    Single(CopyModifier<'a>),
    Multiple(Vec<CopyModifier<'a>>),
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopyBlockTrait<'a> {
    pub name: &'a str,
    pub source: &'a str,
}
