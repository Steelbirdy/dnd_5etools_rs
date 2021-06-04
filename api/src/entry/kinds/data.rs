use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryDataCreature<'a> {
    #[serde(borrow, flatten)]
    pub base: EntryBaseData<'a>,
    pub data_creature: DataCreature<'a>,
}

impl<'a> From<DataCreature<'a>> for EntryDataCreature<'a> {
    fn from(data_creature: DataCreature<'a>) -> Self {
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
    pub base: EntryBaseData<'a>,
    pub data_spell: DataSpell<'a>,
}

impl<'a> From<DataSpell<'a>> for EntryDataSpell<'a> {
    fn from(data_spell: DataSpell<'a>) -> Self {
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
    pub base: EntryBaseData<'a>,
    pub data_trap_hazard: EntryDataTrapHazardKind<'a>,
}

impl<'a> From<DataTrap<'a>> for EntryDataTrapHazard<'a> {
    fn from(data_trap: DataTrap<'a>) -> Self {
        Self {
            base: Default::default(),
            data_trap_hazard: EntryDataTrapHazardKind::Trap(data_trap),
        }
    }
}

impl<'a> From<DataHazard<'a>> for EntryDataTrapHazard<'a> {
    fn from(data_hazard: DataHazard<'a>) -> Self {
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
    pub base: EntryBaseData<'a>,
    pub data_object: DataObject<'a>,
}

impl<'a> From<DataObject<'a>> for EntryDataObject<'a> {
    fn from(data_object: DataObject<'a>) -> Self {
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
    pub base: EntryBaseData<'a>,
    pub data_item: DataItem<'a>,
}

impl<'a> From<DataItem<'a>> for EntryDataItem<'a> {
    fn from(data_item: DataItem<'a>) -> Self {
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
    Trap(DataTrap<'a>),
    Hazard(DataHazard<'a>),
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
pub struct DataCreature<'a>(&'a str);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSpell<'a>(&'a str);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataTrap<'a>(&'a str);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataHazard<'a>(&'a str);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataObject<'a>(&'a str);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataItem<'a>(&'a str);
