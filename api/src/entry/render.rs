use super::*;
use super::kinds::all::*;
use super::error::Error;

pub type Result<T> = std::result::Result<T, error::Error>;

pub trait EntryRenderer {
    fn render(&self, entry: Entry) -> Result<String> {
        match entry {
            Entry::Entry(kind) => self.render_entry_kind(kind),
            Entry::Integer(n) => self.render_entry_int(n),
            Entry::String(str) => self.render_entry_string(str),
        }
    }

    fn render_entry_kind(&self, kind: EntryKind) -> Result<String> {
        use EntryKind::*;
        match kind {
            Section(value) => self.render_section(value),
            Entries(value) => self.render_entries(value),
            Homebrew(value) => self.render_homebrew(value),
            Quote(value) => self.render_quote(value),
            Inline(value) => self.render_inline(value),
            InlineBlock(value) => self.render_inline_block(value),
            Options(value) => self.render_options(value),
            Table(value) => self.render_table(value),
            TableGroup(value) => self.render_table_group(value),
            TableRow(value) => self.render_table_row(value),
            TableCell(value) => self.render_table_cell(value),
            List(value) => self.render_list(value),
            Bonus(value) => self.render_bonus(value),
            BonusSpeed(value) => self.render_bonus_speed(value),
            Dice(value) => self.render_dice(value),
            AbilityDc(value) => self.render_ability_dc(value),
            AbilityAttackMod(value) => self.render_ability_attack_mod(value),
            AbilityGeneric(value) => self.render_ability_generic(value),
            Link(value) => self.render_link(value),
            OptFeature(value) => self.render_opt_feature(value),
            Inset(value) => self.render_inset(value),
            InsetReadaloud(value) => self.render_inset_readaloud(value),
            Variant(value) => self.render_variant(value),
            VariantInner(value) => self.render_variant_inner(value),
            VariantSub(value) => self.render_variant_sub(value),
            Item(value) => self.render_item(value),
            ItemSub(value) => self.render_item_sub(value),
            ItemSpell(value) => self.render_item_spell(value),
            Image(value) => self.render_image(value),
            Gallery(value) => self.render_gallery(value),
            Actions(value) => self.render_actions(value),
            Attack(value) => self.render_attack(value),
            Flowchart(value) => self.render_flowchart(value),
            FlowBlock(value) => self.render_flow_block(value),
            Ingredient(value) => self.render_ingredient(value),
            DataCreature(value) => self.render_data_creature(value),
            DataSpell(value) => self.render_data_spell(value),
            DataTrapHazard(value) => self.render_data_trap_hazard(value),
            DataObject(value) => self.render_data_object(value),
            DataItem(value) => self.render_data_item(value),
            RefClassFeature(value) => self.render_ref_class_feature(value),
            RefSubclassFeature(value) => self.render_ref_subclass_feature(value),
            RefOptionalFeature(value) => self.render_ref_optional_feature(value),
            Hr(value) => self.render_hr(value),
            Spellcasting(value) => self.render_spellcasting(value),
        }
    }

    fn render_entry_int(&self, n: i64) -> Result<String>;

    fn render_entry_string(&self, string: &str) -> Result<String>;

    fn render_section(&self, entry: EntrySection) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_entries(&self, entry: EntryEntries) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_homebrew(&self, entry: EntryHomebrew) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_quote(&self, entry: EntryQuote) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_inline(&self, entry: EntryInline) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_inline_block(&self, entry: EntryInlineBlock) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_options(&self, entry: EntryOptions) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_table(&self, entry: EntryTable) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_table_group(&self, entry: EntryTableGroup) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_table_row(&self, entry: EntryTableRow) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_table_cell(&self, entry: EntryTableCell) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_list(&self, entry: EntryList) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_bonus(&self, entry: EntryBonus) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_bonus_speed(&self, entry: EntryBonusSpeed) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_dice(&self, entry: EntryDice) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_ability_dc(&self, entry: EntryAbilityDc) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_ability_attack_mod(&self, entry: EntryAbilityAttackMod) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_ability_generic(&self, entry: EntryAbilityGeneric) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_link(&self, entry: EntryLink) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_opt_feature(&self, entry: EntryOptFeature) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_inset(&self, entry: EntryInset) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_inset_readaloud(&self, entry: EntryInsetReadaloud) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_variant(&self, entry: EntryVariant) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_variant_inner(&self, entry: EntryVariantInner) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_variant_sub(&self, entry: EntryVariantSub) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_item(&self, entry: EntryItem) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_item_sub(&self, entry: EntryItemSub) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_item_spell(&self, entry: EntryItemSpell) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_image(&self, entry: EntryImage) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_gallery(&self, entry: EntryGallery) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_actions(&self, entry: EntryActions) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_attack(&self, entry: EntryAttack) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_flowchart(&self, entry: EntryFlowchart) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_flow_block(&self, entry: EntryFlowBlock) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_ingredient(&self, entry: EntryIngredient) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_data_creature(&self, entry: EntryDataCreature) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_data_spell(&self, entry: EntryDataSpell) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_data_trap_hazard(&self, entry: EntryDataTrapHazard) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_data_object(&self, entry: EntryDataObject) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_data_item(&self, entry: EntryDataItem) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_ref_class_feature(&self, entry: EntryRefClassFeature) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_ref_subclass_feature(&self, entry: EntryRefSubclassFeature) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_ref_optional_feature(&self, entry: EntryRefOptionalFeature) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_hr(&self, entry: EntryHr) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
    fn render_spellcasting(&self, entry: EntrySpellcasting) -> Result<String> {
        let _ = entry;
        Err(Error::NotImplemented)
    }
}