use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum OptionalFeatureType {
    #[serde(rename = "EI")]
    EldritchInvocation,
    #[serde(rename = "ED")]
    ElementalDiscipline,
    FightingStyle(FightingStyle),
    Maneuver(Maneuver),
    #[serde(rename = "MM")]
    Metamagic,
    #[serde(rename = "OTH")]
    Other,
    #[serde(rename = "AS")]
    ArcaneShot,
    #[serde(rename = "PB")]
    PactBoon,
    #[serde(rename = "AI")]
    ArtificerInfusion,
    ShipUpgrade(ShipUpgrade),
    InfernalWarMachineUpgrade(InfernalWarMachineUpgrade),
    #[serde(rename = "OR")]
    OnomancyResonant,
    #[serde(rename = "RN")]
    RuneKnightRune,
    #[serde(rename = "AF")]
    AlchemicalFormula,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum FightingStyle {
    #[serde(rename = "FS:B")]
    Bard,
    #[serde(rename = "FS:F")]
    Fighter,
    #[serde(rename = "FS:P")]
    Paladin,
    #[serde(rename = "FS:R")]
    Ranger,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum InfernalWarMachineUpgrade {
    #[serde(rename = "IWM:W")]
    Weapon,
    #[serde(rename = "IWM:A")]
    Armor,
    #[serde(rename = "IWM:G")]
    Gadget,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Maneuver {
    #[serde(rename = "MV")]
    Generic,
    #[serde(rename = "MV:B")]
    BattleMaster,
    #[serde(rename = "MV:C2-UA")]
    UACavalierV2,
    #[serde(rename = "AS:V1-UA")]
    UAArcaneShotV1,
    #[serde(rename = "AS:V2-UA")]
    UAArcaneShotV2,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum ShipUpgrade {
    #[serde(rename = "SHP:H")]
    Hull,
    #[serde(rename = "SHP:M")]
    Movement,
    #[serde(rename = "SHP:W")]
    Weapon,
    #[serde(rename = "SHP:F")]
    Figurehead,
    #[serde(rename = "SHP:O")]
    Miscellaneous,
}
