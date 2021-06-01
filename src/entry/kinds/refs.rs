use super::*;

/// For use in classes page content only.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryRefClassFeature<'a> {
    pub class_feature: &'a str,
}

impl<'a> EntryRefClassFeature<'a> {
    pub fn new(class_feature_ref: &'a str) -> Self {
        Self {
            class_feature: class_feature_ref,
        }
    }
}

/// For use in classes page content only.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryRefSubclassFeature<'a> {
    pub subclass_feature: &'a str,
}

impl<'a> EntryRefSubclassFeature<'a> {
    pub fn new(subclass_feature_ref: &'a str) -> Self {
        Self {
            subclass_feature: subclass_feature_ref,
        }
    }
}

/// For use in classes page content only.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryRefOptionalFeature<'a> {
    #[serde(rename = "optionalfeature")]
    pub optional_feature: &'a str,
    pub name: Option<&'a str>,
}

impl<'a> EntryRefOptionalFeature<'a> {
    pub fn new(optional_feature_ref: &'a str, name: Option<&'a str>) -> Self {
        Self {
            optional_feature: optional_feature_ref,
            name,
        }
    }
}

impl<'a> From<EntryRefClassFeature<'a>> for EntryKind<'a> {
    fn from(value: EntryRefClassFeature<'a>) -> Self {
        EntryKind::RefClassFeature(value)
    }
}

impl<'a> From<EntryRefSubclassFeature<'a>> for EntryKind<'a> {
    fn from(value: EntryRefSubclassFeature<'a>) -> Self {
        EntryKind::RefSubclassFeature(value)
    }
}

impl<'a> From<EntryRefOptionalFeature<'a>> for EntryKind<'a> {
    fn from(value: EntryRefOptionalFeature<'a>) -> Self {
        EntryKind::RefOptionalFeature(value)
    }
}
