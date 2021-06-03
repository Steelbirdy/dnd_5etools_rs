use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[allow(dead_code)]
pub type StartingEquipment<'a> = HashMap<&'a str, Vec<StartingEquipmentItem<'a>>>;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StartingEquipmentItem<'a> {
    /// An item UID, e.g. "longsword|phb"
    String(&'a str),
    Item {
        /// An item UID, e.g. "longsword|phb"
        item: &'a str,
        quantity: Option<u64>,
        /// The display name this item should be given
        display_name: Option<&'a str>,
        /// The value (in copper pieces) that this piece of equipment contains in coins.
        contains_value: Option<u64>,
    },
    EquipmentType {
        equipment_type: EquipmentType,
        quantity: Option<u64>,
    },
    Value {
        /// A value in copper pieces
        value: u64,
    },
    Special {
        /// The name of a non-statted item, such as "wooden stake"
        special: &'a str,
        quantity: Option<u64>,
        /// The worth (in copper pieces) of this piece of equipment.
        worth_value: Option<u64>,
        /// The value (in copper pieces) that this piece of equipment contains in coins.
        contains_value: Option<u64>,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EquipmentType {
    WeaponSimple,
    WeaponSimpleMelee,
    WeaponMartial,
    WeaponMartialMelee,
    InstrumentMusical,
    ArmorLight,
    ArmorMedium,
    ArmorHeavy,
    WeaponMelee,
    WeaponRanged,
    FocusSpellcasting,
    SetGaming,
    ToolArtisan,
}
