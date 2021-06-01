use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryDataCreature<'a> {
    #[serde(borrow, flatten)]
    pub base: EntryBase<'a>,
    pub data_creature: Creature<'a>,
}

impl<'a> From<Creature<'a>> for EntryDataCreature<'a> {
    fn from(data_creature: Creature<'a>) -> Self {
        Self {
            base: Default::default(),
            data_creature,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryDataSpell<'a> {
    #[serde(borrow, flatten)]
    pub base: EntryBase<'a>,
    pub data_spell: Spell<'a>,
}

impl<'a> From<Spell<'a>> for EntryDataSpell<'a> {
    fn from(data_spell: Spell<'a>) -> Self {
        Self {
            base: Default::default(),
            data_spell,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryDataTrapHazard<'a> {
    #[serde(borrow, flatten)]
    pub base: EntryBase<'a>,
    pub data_trap_hazard: EntryDataTrapHazardKind<'a>,
}

impl<'a> From<Trap<'a>> for EntryDataTrapHazard<'a> {
    fn from(data_trap: Trap<'a>) -> Self {
        Self {
            base: Default::default(),
            data_trap_hazard: EntryDataTrapHazardKind::Trap(data_trap),
        }
    }
}

impl<'a> From<Hazard<'a>> for EntryDataTrapHazard<'a> {
    fn from(data_hazard: Hazard<'a>) -> Self {
        Self {
            base: Default::default(),
            data_trap_hazard: EntryDataTrapHazardKind::Hazard(data_hazard),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryDataObject<'a> {
    #[serde(borrow, flatten)]
    pub base: EntryBase<'a>,
    pub data_object: Object<'a>,
}

impl<'a> From<Object<'a>> for EntryDataObject<'a> {
    fn from(data_object: Object<'a>) -> Self {
        Self {
            base: Default::default(),
            data_object,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryDataItem<'a> {
    #[serde(borrow, flatten)]
    pub base: EntryBase<'a>,
    pub data_item: Item<'a>,
}

impl<'a> From<Item<'a>> for EntryDataItem<'a> {
    fn from(data_item: Item<'a>) -> Self {
        Self {
            base: Default::default(),
            data_item,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EntryDataTrapHazardKind<'a> {
    #[serde(borrow)]
    Trap(Trap<'a>),
    Hazard(Hazard<'a>),
}

impl<'a> From<EntryDataCreature<'a>> for EntryKind<'a> {
    fn from(value: EntryDataCreature<'a>) -> Self {
        EntryKind::DataCreature(value)
    }
}

impl<'a> From<EntryDataSpell<'a>> for EntryKind<'a> {
    fn from(value: EntryDataSpell<'a>) -> Self {
        EntryKind::DataSpell(value)
    }
}

impl<'a> From<EntryDataTrapHazard<'a>> for EntryKind<'a> {
    fn from(value: EntryDataTrapHazard<'a>) -> Self {
        EntryKind::DataTrapHazard(value)
    }
}

impl<'a> From<EntryDataObject<'a>> for EntryKind<'a> {
    fn from(value: EntryDataObject<'a>) -> Self {
        EntryKind::DataObject(value)
    }
}

impl<'a> From<EntryDataItem<'a>> for EntryKind<'a> {
    fn from(value: EntryDataItem<'a>) -> Self {
        EntryKind::DataItem(value)
    }
}

// TODO: Temporary structs. Write tests later
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Creature<'a>(&'a str);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spell<'a>(&'a str);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trap<'a>(&'a str);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hazard<'a>(&'a str);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Object<'a>(&'a str);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item<'a>(&'a str);
