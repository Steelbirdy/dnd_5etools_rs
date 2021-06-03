use super::*;

#[allow(dead_code)]
pub type WeaponProficiencies<'a> = Vec<WeaponProficiency<'a>>;

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeaponProficiency<'a> {
    #[serde(flatten, with = "super::bitflags_as_map")]
    pub weapons: BitFlags<Weapon>,
    pub any: Option<u8>,
    #[serde(borrow)]
    pub choose: Option<WeaponProficiencyChoose<'a>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeaponProficiencyChoose<'a> {
    /// A filter string, e.g. \"type=martial weapon|miscellaneous=mundane\"
    pub from: &'a str,
    pub count: u8,
}

#[bitflags]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Weapon {
    Simple,
    Martial,
    Firearms,
    #[serde(rename = "battleaxe|phb")]
    Battleaxe,
    #[serde(rename = "club|phb")]
    Club,
    #[serde(rename = "dagger|phb")]
    Dagger,
    #[serde(rename = "flail|phb")]
    Flail,
    #[serde(rename = "glaive|phb")]
    Glaive,
    #[serde(rename = "greataxe|phb")]
    Greataxe,
    #[serde(rename = "greatclub|phb")]
    Greatclub,
    #[serde(rename = "greatsword|phb")]
    Greatsword,
    #[serde(rename = "halberd|phb")]
    Halberd,
    #[serde(rename = "handaxe|phb")]
    Handaxe,
    #[serde(rename = "javalin|phb")]
    Javelin,
    #[serde(rename = "lance|phb")]
    Lance,
    #[serde(rename = "light hammer")]
    LightHammer,
    #[serde(rename = "longsword|phb")]
    Longsword,
    #[serde(rename = "mace|phb")]
    Mace,
    #[serde(rename = "maul|phb")]
    Maul,
    #[serde(rename = "morningstar|phb")]
    Morningstar,
    #[serde(rename = "pike|phb")]
    Pike,
    #[serde(rename = "quarterstaff|phb")]
    Quarterstaff,
    #[serde(rename = "rapier|phb")]
    Rapier,
    #[serde(rename = "scimitar|phb")]
    Scimitar,
    #[serde(rename = "shortsword|phb")]
    Shortsword,
    #[serde(rename = "sickle|phb")]
    Sickle,
    #[serde(rename = "spear|phb")]
    Spear,
    #[serde(rename = "staff|phb")]
    Staff,
    #[serde(rename = "trident|phb")]
    Trident,
    #[serde(rename = "war pick|phb")]
    WarPick,
    #[serde(rename = "warhammer|phb")]
    Warhammer,
    #[serde(rename = "whip|phb")]
    Whip,
    #[serde(rename = "blowgun|phb")]
    Blowgun,
    #[serde(rename = "dart|phb")]
    Dart,
    #[serde(rename = "hand crossbow|phb")]
    HandCrossbow,
    #[serde(rename = "heavy crossbow|phb")]
    HeavyCrossbow,
    #[serde(rename = "light crossbow|phb")]
    LightCrossbow,
    #[serde(rename = "longbow|phb")]
    Longbow,
    #[serde(rename = "net|phb")]
    Net,
    #[serde(rename = "shortbow|phb")]
    Shortbow,
    #[serde(rename = "sling|phb")]
    Sling,
    #[serde(rename = "double-bladed scimitar|erlw")]
    DoubleBladedScimitar,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn serde_weapon_proficiencies() {
        let json = r#"[
  {
    "simple": true,
    "longsword|phb": true,
    "choose": {
      "from": "type=martial weapon|source=phb",
      "count": 2
    }
  },
  {
    "net|phb": true,
    "double-bladed scimitar|erlw": true
  },
  {
    "any": 1,
    "choose": {
      "from": "type=simple weapon|miscellaneous=mundane",
      "count": 1
    }
  }
]"#;

        let object = vec![
            WeaponProficiency {
                weapons: Weapon::Simple | Weapon::Longsword,
                any: None,
                choose: Some(WeaponProficiencyChoose {
                    from: "type=martial weapon|source=phb",
                    count: 2,
                }),
            },
            WeaponProficiency {
                weapons: Weapon::Net | Weapon::DoubleBladedScimitar,
                any: None,
                choose: None,
            },
            WeaponProficiency {
                weapons: BitFlags::empty(),
                any: Some(1),
                choose: Some(WeaponProficiencyChoose {
                    from: "type=simple weapon|miscellaneous=mundane",
                    count: 1,
                }),
            },
        ];

        check_serde(json, object);
    }
}
