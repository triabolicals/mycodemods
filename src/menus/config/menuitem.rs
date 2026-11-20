use super::*;

use engage::{
    GameVariableManager,
    Mess,
    app::{BasicMenuItem_Attribute, BasicMenu_Result, Force, Force_Type, GameUserData, GodData, GrowMode, Pad, Random_2, SoftwareKeyboard, SoftwareKeyboard_Preset, UnitPool},
    nn::hid::NpadButton,
    system::Action_1
};
use engage::app::{Gender, PersonData, Unit};
use unity::{Il2CppString, OptionalMethod};
use crate::utils::to_opt;
pub use ConfigMenuItemKind::*;

#[unity::inject(namespace="MyCodeMods", name="MyCodeModConfigItem", parent=ConfigBasicMenuItem)]
pub struct MyCodeModConfigItem { pub kind: i32 }

impl MyCodeModConfigItem {
    pub fn new(kind: ConfigMenuItemKind) -> MyCodeModConfigItem {
        let item = Self::instantiate().unwrap();
        let command = kind.max_value() == 0;
        IConfigBasicMenuItemMethods::ctor(item);
        item.set_m_is_command_icon(command);
        item.set_m_is_arrow(false);
        item.set_kind(kind as i32);
        let key = kind.get_key();
        if !GameVariableManager::is_exist(kind.get_key()) {
            match kind {
                Growth => {
                    let v = if GameUserData::get_instance().get_grow_mode().value == 0 { 0 } else { 1 };
                    GameVariableManager::make_entry_norewind(kind.get_key(), v);
                }
                LueurName => {
                    if let Some(unit) = ConfigMenuItemKind::get_lueur() {
                        let name = unit.get_name();
                        GameVariableManager::make_entry_str(key, name);
                        item.set_m_command_text(name);
                    }
                }
                LueurGender => {
                    if let Some(unit) = ConfigMenuItemKind::get_lueur() {
                        let name = unit.get_name();
                        GameVariableManager::make_entry_str(key, name);
                        item.set_m_command_text(name);
                    }
                }
                _ => { GameVariableManager::make_entry_norewind(kind.get_key(), 0); }
            }
        }
        item.set_m_command_text(kind.get_command_text());
        item.set_m_help_text(kind.get_help_text());
        item.set_m_title_text(kind.get_title());
        item
    }
    pub fn update_command_help_text(self) {
        let kind = self.get_menu_kind();
        self.set_m_command_text(kind.get_command_text());
        self.set_m_help_text(kind.get_help_text());
        self.set_m_title_text(kind.get_title());
        self.update_text();
    }
    pub fn get_menu_kind(self) -> ConfigMenuItemKind {
        ConfigMenuItemKind::from(self.kind())
    }
}
#[unity::injected_methods]
impl MyCodeModConfigItem {
    #[override_virtual(name="ACall")]
    pub fn a_call(self) -> BasicMenu_Result { self.get_menu_kind().a_call(self) }

    #[override_virtual(name="BuildAttribute")]
    pub fn build_attribute(self) -> BasicMenuItem_Attribute { self.get_menu_kind().build_attribute() }

    #[override_virtual(name="OnSelect")]
    pub fn on_selected(self) {
        unsafe { ConfigBasicMenuItem::on_select(self) };
        let is_command = self.get_menu_kind().max_value() == 0;
        self.set_m_is_arrow(!is_command);
        self.set_m_is_command_icon(is_command);
        self.update_command_help_text();
    }
    #[override_virtual(name="CustomCall")]
    pub fn custom_call(self) -> BasicMenu_Result {
        let kind = self.get_menu_kind();
        if Pad::is_trigger(NpadButton::left()) == Pad::is_trigger(NpadButton::right()) {
            return BasicMenu_Result::pass();
        }
        if kind.is_flag() {
            if kind.toggle_flag() {
                self.update_command_help_text();
                BasicMenu_Result::se_cursor()
            }
            else { BasicMenu_Result::pass() }
        }
        else {
            if kind.increment_value() {
                self.update_command_help_text();
                BasicMenu_Result::se_cursor()
            }
            else { BasicMenu_Result::pass() }
        }
    }
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq)]
pub enum ConfigMenuItemKind {
    LevelDisplay,
    InternalLevel,
    AI,
    Cutscene,
    GiftDLC,
    GiftPatch,
    GiftHero,
    BGM,
    AutoFisher,
    DLCGod,
    DLCEvil,
    Support,
    Achievements,
    DeathList,
    Engrave,
    BattleAction,
    ExpSequence,
    // Var
    Exp,
    Reseed,
    Growth,
    Talk,
    LueurName,
    LueurGender,
    Menu,
    Invalid,
}
impl From<i32> for ConfigMenuItemKind {
    fn from(value: i32) -> Self {
        match value {
            0 => LevelDisplay,
            1 => InternalLevel,
            2 => AI,
            3 => Cutscene,
            4 => GiftDLC,
            5 => GiftPatch,
            6 => GiftHero,
            7 => BGM,
            8 => AutoFisher,
            9 => DLCGod,
            10 => DLCEvil,
            11 => Support,
            12 => Achievements,
            13 => DeathList,
            14 => Engrave,
            15 => BattleAction,
            16 => ExpSequence,
            // Var
            17 => Exp,
            18 => Reseed,
            19 => Growth,
            20 => Talk,
            21 => LueurName,
            22 => LueurGender,
            23 => Menu,
            _ => Invalid,
        }
    }
}
impl ConfigMenuItemKind {
    pub fn get_key(self) -> &'static str {
        if self.is_flag() { "G_MCM_Flags" }
        else {
            match self {
                Growth => "G_GROWTH_TYPE",
                Exp => "G_EXP_TYPE",
                Talk => "G_SkipTalk",
                _ => "G_MCM_Flags",
            }
        }
    }
    pub fn max_value(self) -> i32 {
        match self {
            Growth => 3,
            Exp => 2,
            Talk => 2,
            DLCEvil|DLCGod|Menu|Reseed => 0,
            LueurGender => 2,
            _ => 1,
        }
    }
    pub fn get_lueur() -> Option<Unit> {
        to_opt(UnitPool::get_from_person(PersonData::get("PID_リュール".into()), false))
    }
    pub fn get_command_text(self) -> Il2CppString {
        let v = self.get_value();
        match self {
            DLCGod|DLCEvil => { "Clear".into() }
            AutoFisher => { if v != 0 { "Max Bond".into() } else { "Default".into() } }
            Talk => {
                if v == 0 { Mess::get("MID_CONFIG_TUTORIAL_OFF") }
                else if v == 1 { "Map".into() }
                else { "All".into() }
            }
            LueurGender => {
                let gender = Self::get_lueur().map(|l| l.m_edit().m_gender().value).unwrap_or(0);
                match gender {
                    1 => "Male",
                    2 => "Female",
                    _ => "???"
                }.into()
            }
            Exp => {
                match v  {
                    1 => Mess::get("MID_SYS_None"),
                    2 => Mess::get("MID_SYS_MAX"),
                    _ => "Default".into()
                }
            }
            Growth => {
                match v {
                    0 => Mess::get("MID_SYS_Grow_Random"),
                    1 => Mess::get("MID_SYS_Grow_Fixed"),
                    2 => Mess::get("MID_SYS_None"),
                    3 => Mess::get("MID_Hub_MuscleExercises_Perfect"),
                    _ => "???".into()
                }
            }
            Reseed => { "Reseed".into() }
            LueurName => if let Some(lueur) = Self::get_lueur() { lueur.get_name() } else { Mess::get("MPID_Lueur") },
            Menu => "View".into(),
            _ => {
                if v != 0 { Mess::get("MID_CONFIG_TUTORIAL_ON") } else { Mess::get("MID_CONFIG_TUTORIAL_OFF") }
            }
        }
    }
    pub fn a_call(self, menu_item: MyCodeModConfigItem) -> BasicMenu_Result {
        match self {
            Menu => {
                SettingMenu::create_bind(menu_item.m_menu());
                BasicMenu_Result::close_decide()
            }
            DLCGod => {
                for x in 1..7 {
                    GameVariableManager::set_bool(format!("G_Cleared_G00{}", x), true);
                    GameVariableManager::set_number(format!("G_GmapSpot_G00{}", x), 3);
                }
                let emblems = ["エーデルガルト", "チキ", "ヘクトル", "ヴェロニカ", "セネリオ", "カミラ", "クロム"];
                if GameVariableManager::get_number("G_Emblem_Mode") == 0 {
                    for x in emblems {
                        let gid = format!("GID_{}", x);
                        engage::app::GodPool::create(GodData::get(gid.into()));
                    }
                }
                else {
                    for x in emblems {
                        let key = format!("G_R_GID_{}", x);
                        let gid =
                            if GameVariableManager::is_exist(key.as_str()) { GameVariableManager::get_string(key.as_str()) }
                            else { format!("GID_{}", x).into() };
                        engage::app::GodPool::create(GodData::get(gid.into()));
                    }
                }
                menu_item.m_menu().rebuild_instant(false);
                BasicMenu_Result::se_decide2()
            }
            DLCEvil => {
                for x in 1..7 {
                    GameVariableManager::set_bool(format!("G_Cleared_E00{}", x), true);
                    GameVariableManager::set_number(format!("G_GmapSpot_E00{}", x), 3);
                }
                menu_item.m_menu().rebuild_instant(false);
                BasicMenu_Result::se_decide2()
            }
            Reseed => {
                let rng = Random_2::get_system();
                Force::get(Force_Type::player()).iter()
                    .chain(Force::get(Force_Type::absent()).iter())
                    .for_each(|u|{ u.set_grow_seed(rng.get_value() as u32); });
                BasicMenu_Result::se_decide()
            }
            LueurName => {
                if let Some(unit) = to_opt(UnitPool::get_from_person_2("PID_リュール", false)){
                    let name = unit.get_name();
                    let header = engage::app::Mess::get("MID_GAMESTART_PLAYER_NAME_INPUT");
                    let action = Action_1::new(menu_item.into(), set_alear_name_method_info().into());
                    SoftwareKeyboard::create_bind(menu_item.m_menu(), 12, name, header, "", SoftwareKeyboard_Preset::default(), action);
                }
                BasicMenu_Result::se_cursor()
            }
            _ => { BasicMenu_Result::pass() }
        }
    }
    pub fn get_title(self) -> Il2CppString {
        match self {
            LevelDisplay => "Level Display",
            InternalLevel => "Internal Level",
            AI => "Enemy AI Data",
            Cutscene => "Skip Cutscenes / Movies ",
            GiftDLC => "Skip DLC Gift Items",
            GiftPatch => "Skip Update Gift Items",
            GiftHero => "Skip Fire Emblem Heroes Items",
            BGM => "Single Map BGM",
            AutoFisher => "Autofisher Bond Setting",
            DLCGod => "Clear Divine Paralogues",
            DLCEvil => "Clear Fell Xenologue",
            Support => "Autocomplete Supports",
            Exp => "Exp Mode",
            Reseed => "Reseed Growth Seeds",
            Growth => "Growth Mode",
            Talk => "Talk Dialogue",
            LueurName => "Player Name",
            LueurGender => "Player Gender",
            Achievements => "Autocollect Achievements",
            Menu => "MyCodeMods",
            DeathList => "Unit Death List",
            Engrave => "Engrave Data",
            BattleAction => "Skip Map Animations",
            ExpSequence => "Skip Exp Gain",
            Invalid => "???"
        }.into()
    }
    pub fn get_help_text(self) -> Il2CppString {
        match self {
            LevelDisplay => "Display unit's total level in Unit Status.".into(),
            InternalLevel => "Displays internal level in Unit Info/Unit Status.".into(),
            AI => "Displays AI data in the Unit's name help box.".into(),
            Cutscene => "Skips movies / in-game cutscenes".into(),
            GiftPatch => "Disable gift items from patch updates.".into(),
            GiftDLC => "Disable gift items from DLC updates.".into(),
            GiftHero => "Disable Fire Emblem Heroes gift items.".into(),
            BGM => "Player phase BGM will be used for all phases.".into(),
            AutoFisher => "Enable the maximum bond fragment when auto fishing.".into(),
            DLCGod => "Automatically complete Divine Paralogue chapters.".into(),
            DLCEvil => "Automatically complete Fell Xenologue chapters.".into(),
            Reseed => "Reseed all unit's growth seed. Cannot be reversed.".into(),
            DeathList => "Displays list of dead units in loading screens".into(),
            Engrave => "Displays engraving stats and previous engraved weapon.".into(),
            BattleAction => "Skip certain map animations.".into(),
            ExpSequence => "Skip experience gain in battle.".into(),
            Exp => {
                match self.get_value() {
                    1 => "Units gain no experience.".into(),
                    2 => "Units gain experience required to level up.".into(),
                    _ => "Units gain experience normally.".into(),
                }
            }
            Support => "Auto complete support conversations.".into(),
            Talk => {
                match self.get_value() {
                    1 => "Map dialogue is skipped.",
                    2 => "All dialogue is skipped.",
                    _ => "No dialogue is skipped.",
                }.into()
            }
            Growth => {
                let v = self.get_value();
                match v {
                    1 => "Units will gain stats by 'Fixed' mode.".into(),
                    2 => "Units will not gain stats on level up.".into(),
                    3 => "Units will have perfect level ups.".into(),
                    _ => "Units will gain stats by 'Random' mode.".into(),
                }
            }
            LueurName => format!("Change {}'s assigned name.", Mess::get("MPID_Lueur")).into(),
            LueurGender => format!("Change {}'s gender.", Mess::get("MPID_Lueur")).into(),
            Menu => "Open up MyCodeMods Settings".into(),
            Achievements => "Automatically collect bond fragments from achievements.".into(),
            _ => "???".into(),
        }
    }
    pub fn is_bool(self) -> bool { self.get_value() != 0 }
    pub fn is_flag(self) -> bool { self.max_value() == 1 }
    pub fn get_flag_value(self) -> bool {
        if self.is_flag() {
            let i = self as i32;
            let v = GameVariableManager::get_number(self.get_key()) & (1 << i) != 0;
            // println!("Flag #{}: {} [{}]", i, self.get_title(), v);
            v
        }
        else { false }
    }
    pub fn get_value(self) -> i32 {
        if self.is_flag() { self.get_flag_value() as i32 }
        else { GameVariableManager::get_number(self.get_key()) }
    }
    pub fn set_value(self, value: i32) {
        match self {
            LueurGender => {
                if let Some(lueur) = Self::get_lueur(){
                    let value = if lueur.m_edit().m_gender().value == 1 { 2 } else { 1 };
                    lueur.m_edit().set_m_gender(Gender{value});
                }
            }
            _ => {
                if !self.is_flag() { GameVariableManager::set_number(self.get_key(), value); }
            }
        }
    }
    pub fn toggle_flag(self) -> bool {
        if self.is_flag() {
            let i = self as i32;
            let f = 1 << i;
            let v = GameVariableManager::get_number(self.get_key());
            if v & f != 0 { GameVariableManager::set_number(self.get_key(), v & !f); }
            else { GameVariableManager::set_number(self.get_key(), v | f); }
        }
        self.is_flag()
    }
    pub fn increment_value(self) -> bool {
        let v = self.get_value();
        let left = Pad::is_trigger(NpadButton::left());
        let right = Pad::is_trigger(NpadButton::right());
        if left == right { false }
        else if self.is_flag() { self.toggle_flag() }
        else if self.max_value() == 0 { false }
        else if left {
            if v == 0 { self.set_value(self.max_value()); } else { self.set_value(v - 1); }
            true
        }
        else if right {
            if v >= self.max_value() { self.set_value(0); } else { self.set_value(v + 1); }
            true
        }
        else { false }
    }
    pub fn build_attribute(self) -> BasicMenuItem_Attribute {
        match self {
            DLCEvil => {
                if GameVariableManager::get_bool("G_Cleared_E006") || !GameVariableManager::get_bool("G_拠点_神竜導入イベント再生済み")  {
                    return BasicMenuItem_Attribute::hide();
                }
            }
            DLCGod=> {
                if !GameVariableManager::get_bool("G_拠点_神竜導入イベント再生済み") { return BasicMenuItem_Attribute::hide(); }
                for i in 1..7 {
                    if !GameVariableManager::get_bool(format!("G_Cleared_G00{}", i)) {
                        return BasicMenuItem_Attribute::enable();
                    }
                }
                return BasicMenuItem_Attribute::hide();
            }
            Reseed => {
                let user = GameUserData::get_instance();
                if user.get_difficulty(false).value == 2 && user.get_grow_mode() == GrowMode::random() {
                    return BasicMenuItem_Attribute::hide();
                }
            }
            GiftDLC|GiftPatch => {
                return if !crate::gifts::obtained_gifts(self == GiftDLC) { BasicMenuItem_Attribute::enable() }
                else { BasicMenuItem_Attribute::hide() }
            }
            GiftHero => {
                return
                    if !crate::gifts::has_hero_dlc() { BasicMenuItem_Attribute::hide() }
                    else if GameVariableManager::get_number("G_拠点_コンテンツ報酬受け取り済み") == 0 { BasicMenuItem_Attribute::enable() }
                    else { BasicMenuItem_Attribute::hide() }
            }
            _ => {}
        }
        BasicMenuItem_Attribute::enable()
    }
}

#[unity::callback]
pub fn set_alear_name(item: MyCodeModConfigItem, this: Il2CppString, _method_info: OptionalMethod) {
    if let Some(lueur) = ConfigMenuItemKind::get_lueur() {
        lueur.m_edit().set_m_name(this);
        item.update_command_help_text();
    }
}