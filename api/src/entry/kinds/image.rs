use super::*;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryImage<'a> {
    #[serde(flatten)]
    pub base: EntryBaseData<'a>,
    pub href: MediaHref<'a>,
    /// A thumbnail image used in rare cases, e.g. when loading a wall of maps to choose from in the DM Screen.
    pub href_thumbnail: Option<MediaHref<'a>>,
    pub title: Option<&'a str>,
    /// For accessibility purposes
    pub alt_text: Option<&'a str>,
    pub image_type: Option<EntryImageType>,
    pub map_regions: Option<Vec<EntryImageMapRegion<'a>>>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    /// Specify the max desired display width of the images,
    /// as opposed to \"width\" which should only be used for the _real_ width. Assumes pixels by default.
    pub max_width: Option<i64>,
    /// As per \"maxWidth\"
    pub max_height: Option<i64>,
    pub max_width_units: Option<&'a str>,
    pub max_height_units: Option<&'a str>,
    pub style: Option<EntryImageStyle>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryGallery<'a> {
    #[serde(flatten, borrow)]
    pub base: EntryBaseData<'a>,
    pub images: Vec<EntryImage<'a>>,
}

impl<'a> From<EntryImage<'a>> for EntryKind<'a> {
    fn from(value: EntryImage<'a>) -> Self {
        EntryKind::Image(value)
    }
}

impl<'a> From<EntryGallery<'a>> for EntryKind<'a> {
    fn from(value: EntryGallery<'a>) -> Self {
        EntryKind::Gallery(value)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryImageType {
    Map,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryImageMapRegion<'a> {
    pub area: Option<&'a str>,
    pub points: HashMap<i64, i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EntryImageStyle {
    ComicSpeakerLeft,
    ComicSpeakerRight,
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn serde_entry_image() {
        let json = r#"{
  "type": "image",
  "href": {
    "type": "internal",
    "path": "blank.png"
  },
  "title": "Optional Title"
}"#;

        let object: Entry = EntryImage {
            base: Default::default(),
            href: MediaHref::Internal { path: "blank.png" },
            href_thumbnail: None,
            title: Some("Optional Title"),
            alt_text: None,
            image_type: None,
            map_regions: None,
            width: None,
            height: None,
            max_width: None,
            max_height: None,
            max_width_units: None,
            max_height_units: None,
            style: None,
        }
        .into();

        check_serde(json, object);
    }
}
