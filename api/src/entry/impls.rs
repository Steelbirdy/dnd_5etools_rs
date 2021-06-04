use super::EntryBase;
use super::kinds::*;
use serde_json::Value;

macro_rules! impl_entry_base {
    [$( $kind:ident, )+] => {
        $(
        impl<'a> EntryBase<'a> for $kind<'a> {
            fn name(&self) -> Option<&'a str> {
                self.base.name
            }

            fn source(&self) -> Option<&'a str> {
                self.base.source
            }

            fn data(&self) -> Option<&Value> {
                self.base.data.as_ref()
            }

            fn page(&self) -> Option<i64> {
                self.base.page
            }

            fn id(&self) -> Option<&'a str> {
                self.base.id
            }
        }
        )+
    };
}

impl_entry_base![
    EntryAbilityDc,
    EntryAbilityAttackMod,
    EntryAbilityGeneric,
    EntryActions,
    EntryAttack,
    EntryBonus,
    EntryBonusSpeed,
    EntryDataCreature,
    EntryDataSpell,
    EntryDataTrapHazard,
    EntryDataObject,
    EntryDataItem,
    EntryDice,
    EntryEntries,
    EntryFlowchart,
    EntryFlowBlock,
    EntryHomebrew,
    EntryImage,
    EntryGallery,
    EntryIngredient,
    EntryInline,
    EntryInlineBlock,
    EntryInset,
    EntryInsetReadaloud,
    EntryItem,
    EntryItemSub,
    EntryItemSpell,
    EntryLink,
    EntryList,
    EntryOptFeature,
    EntryOptions,
    EntryQuote,
    EntrySection,
    EntrySpellcasting,
    EntryTable,
    EntryTableGroup,
    EntryTableRow,
    EntryTableCell,
    EntryVariant,
    EntryVariantInner,
    EntryVariantSub,
];
