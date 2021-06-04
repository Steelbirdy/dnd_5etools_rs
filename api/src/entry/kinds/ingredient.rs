use super::*;
use serde_json::Number;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryIngredient<'a> {
    #[serde(borrow, flatten)]
    pub base: EntryBaseData<'a>,
    pub entry: Box<Entry<'a>>,
    #[serde(flatten)]
    pub amounts: HashMap<&'a str, Number>,
}

impl<'a> From<EntryIngredient<'a>> for EntryKind<'a> {
    fn from(value: EntryIngredient<'a>) -> Self {
        EntryKind::Ingredient(value)
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn serde_entry_entries() {
        let json = r#"{
  "type": "ingredient",
  "entry": "{=amount1} pound thick-cut bacon",
  "amount1": 1
}"#;

        let object: Entry = EntryIngredient {
            base: Default::default(),
            entry: Box::new("{=amount1} pound thick-cut bacon".into()),
            amounts: {
                let mut map = HashMap::new();
                map.insert("amount1", Number::from(1u8));
                map
            },
        }
        .into();

        check_serde(json, object);
    }
}
