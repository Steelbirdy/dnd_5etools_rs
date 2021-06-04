mod error;
mod impls;
pub mod kinds;
mod render;
mod util;

pub use error::{Error, Result};
pub use render::RenderEntry;
pub use util::*;

use kinds::*;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryBaseData<'a> {
    pub name: Option<&'a str>,
    pub source: Option<&'a str>,
    /// A generic object for storing special data for external use-cases.
    /// Keys prefixed with \"rd-\" should be added as \"data-\" HTML attributes when rendering to HTML.
    pub data: Option<Value>,
    /// Technically the API allows for negative page numbers. This should be handled during rendering.
    pub page: Option<i64>,
    pub id: Option<&'a str>,
}

pub trait EntryBase<'a> {
    fn name(&self) -> Option<&'a str>;
    fn source(&self) -> Option<&'a str>;
    fn data(&self) -> Option<&Value>;
    fn page(&self) -> Option<i64>;
    fn id(&self) -> Option<&'a str>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Entry<'a> {
    Entry(EntryKind<'a>),
    String(&'a str),
    // Not really sure why this is a thing, but it is in the schema...
    Integer(i64),
}

impl<'a> Entry<'a> {
    pub fn from_json(s: &'a str) -> Result<Self> {
        serde_json::from_str(s)
            .map_err(Error::from)
    }
}

pub type Entries<'a> = Vec<Entry<'a>>;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum EntryKind<'a> {
    #[serde(borrow)]
    Section(section::EntrySection<'a>),
    Entries(entries::EntryEntries<'a>),
    Homebrew(homebrew::EntryHomebrew<'a>),
    Quote(quote::EntryQuote<'a>),
    Inline(inline::EntryInline<'a>),
    InlineBlock(inline::EntryInlineBlock<'a>),
    /// Used to specify how many of the listed options can be chosen as e.g. permanent character features.
    /// Leave blank for transient choices.
    Options(options::EntryOptions<'a>),
    Table(table::EntryTable<'a>),
    TableGroup(table::EntryTableGroup<'a>),
    #[serde(rename = "row")]
    TableRow(table::EntryTableRow<'a>),
    #[serde(rename = "cell")]
    TableCell(table::EntryTableCell<'a>),
    List(list::EntryList<'a>),
    Bonus(bonus::EntryBonus<'a>),
    BonusSpeed(bonus::EntryBonusSpeed<'a>),
    Dice(dice::EntryDice<'a>),
    AbilityDc(ability::EntryAbilityDc<'a>),
    AbilityAttackMod(ability::EntryAbilityAttackMod<'a>),
    AbilityGeneric(ability::EntryAbilityGeneric<'a>),
    Link(link::EntryLink<'a>),
    #[serde(rename = "optfeature")]
    OptFeature(opt_feature::EntryOptFeature<'a>),
    Inset(inset::EntryInset<'a>),
    InsetReadaloud(inset::EntryInsetReadaloud<'a>),
    Variant(variant::EntryVariant<'a>),
    VariantInner(variant::EntryVariantInner<'a>),
    VariantSub(variant::EntryVariantSub<'a>),
    Item(item::EntryItem<'a>),
    ItemSub(item::EntryItemSub<'a>),
    ItemSpell(item::EntryItemSpell<'a>),
    Image(image::EntryImage<'a>),
    Gallery(image::EntryGallery<'a>),
    Actions(actions::EntryActions<'a>),
    Attack(attack::EntryAttack<'a>),
    Flowchart(flow::EntryFlowchart<'a>),
    FlowBlock(flow::EntryFlowBlock<'a>),
    Ingredient(ingredient::EntryIngredient<'a>),
    DataCreature(data::EntryDataCreature<'a>),
    DataSpell(data::EntryDataSpell<'a>),
    DataTrapHazard(data::EntryDataTrapHazard<'a>),
    DataObject(data::EntryDataObject<'a>),
    DataItem(data::EntryDataItem<'a>),
    RefClassFeature(refs::EntryRefClassFeature<'a>),
    RefSubclassFeature(refs::EntryRefSubclassFeature<'a>),
    #[serde(rename = "refOptionalfeature")]
    RefOptionalFeature(refs::EntryRefOptionalFeature<'a>),
    Hr(hr::EntryHr),
    Spellcasting(spellcasting::EntrySpellcasting<'a>),
}

impl<'a, T> From<T> for Entry<'a>
where
    EntryKind<'a>: From<T>,
{
    fn from(value: T) -> Self {
        let kind = EntryKind::from(value);
        Self::Entry(kind)
    }
}

impl From<i64> for Entry<'_> {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl<'a> From<&'a str> for Entry<'a> {
    fn from(value: &'a str) -> Self {
        Self::String(value)
    }
}
