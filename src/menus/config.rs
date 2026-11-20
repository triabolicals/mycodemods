use super::*;
use engage::{
    app::{
        Pad,
        GameUserData,
        GrowMode,
        IGameUserDataMethods,
        ISingletonClass_1Methods,
        Force,
        Force_Type,
        GodData,
        IRandom_2Methods,
        IStructData_1Methods,
        IUnitMethods,
        Random_2,
        softwarekeyboard::*,
        basicmenuitem::*,
        proc::*,
        procvoidmethod::*,
        procinst::*,
        procboolmethod::*,
        procdesc::*,
        IUnit,
        IUnitEdit,
        ResourceManager_2,
        UnitPool,
        GameMessage,
        IBasicMenuContentMethods,
        IBitField32,
        IJobDataMethods,
        IPersonDataMethods
    },
    GameVariableManager,
    Mess,
    nn::hid::NpadButton,
    ForceExt,
    root::configbasicmenuitem::*,
    prelude::List_1,
    root::{ConfigMenuContent, ConfigSequence, IConfigMenuContentMethods, IConfigRoot},
    system::Action_1,
    unity_engine::IGameObjectMethods,
    ProcVoidMethodExt,
    system::collections::generic::IList_1Methods
};
use unity::{injected_methods, Array, Cast, FromIlInstance, Il2CppString, OptionalMethod};
use crate::utils::to_opt;
use menuitem::ConfigMenuItemKind::*;
use crate::menus::config::menuitem::MyCodeModConfigItem;

pub mod menuitem;

#[unity::inject(namespace="MyCodeMods", name="SettingMenu", parent=BasicMenu)]
pub struct SettingMenu {}

impl SettingMenu {
    pub fn create_bind(proc: impl Into<ProcInst>) {
        let config_sequence = ConfigSequence::new();
        let descs = [
            Proc::call_2(ProcVoidMethod::new(config_sequence.into(),create_setting_menu_method_info().into())),
            Proc::call_2(ProcVoidMethod::from_fn(config_sequence, restore_previous_menu).unwrap()),
            Proc::end(),
        ];
        let descs_array: Array<ProcDesc> = Array::from_slice(&descs).unwrap();
        config_sequence.create_bind(proc, descs_array, "MyCodeModSettingMenu");
    }
}
extern "C" fn restore_previous_menu(this: ConfigSequence, _method: OptionalMethod) {
    if let Some(menu) = this.get_super().try_cast::<BasicMenu>() {
        menu.set_active(true);
        let content = menu.m_menu_content();
        let suspend = IBasicMenu::m_suspend(menu) - 1;
        if suspend < 1 {
            IBasicMenu::set_m_suspend(menu, 0);
            if !content.is_null() { content.un_suspend(); }
            menu.on_resume();
        }
        else { IBasicMenu::set_m_suspend(menu, suspend); }
        engage::BasicMenuExt::open_anime_all(menu);
        engage::TitleBar::open_header(Mess::get("MID_MENU_CONFIG"), Mess::get("MID_MENU_CONFIG_HELP"), "KHID_環境設定_項目選択");
    }
}
#[injected_methods]
impl SettingMenu {
    #[override_virtual(name="OnDispose")]
    fn on_dispose(self) {
        if let Some(v) = menuitem::ConfigMenuItemKind::get_lueur() {
            let flag = v.get_job().get_flag().m_value();
            let gender = v.m_edit().m_gender().value;
            if (flag & 4 != 0 && gender == 1) || (flag & 16 != 0 && gender == 2) {
                let message = format!("{} will reclass to their default class\ndue to gender.", v.get_name());
                GameMessage::create_key_wait(self, message);
                v.set_job(v.get_person().get_job());
            }
        }
        crate::proc::map::patch_imm_bgm();
    }
}
#[unity::callback]
fn create_setting_menu(config: ConfigSequence, _: OptionalMethod) {
    let root = engage::app::GameUI::get_root();
    let go = ResourceManager_2::instantiate("UI/Common/Config/Prefabs/ConfigRoot", root);
    let config_root = go.get_component::<engage::root::ConfigRoot>();
    if config_root.is_null() { return; }
    let menu_content = config_root.m_config_menu_content_object().get_component::<ConfigMenuContent>();
    let list: List_1<BasicMenuItem> = List_1::new();
    let menu = SettingMenu::instantiate().unwrap();
    [
        BattleAction, LevelDisplay, InternalLevel, AI, Cutscene, BGM, AutoFisher, Support, Achievements, Talk,
        Exp, Reseed, Growth, LueurName, GiftDLC, GiftPatch, GiftHero, DLCGod, DLCEvil,
    ].into_iter().for_each(|i|{
        let s = MyCodeModConfigItem::new(i);
        list.add(s.into());
    });

    IBasicMenuMethods::ctor(menu, list, menu_content);
    let descs = IBasicMenuMethods::create_default_desc(menu);
    IProcInstMethods::create_bind(menu, config, descs, "MyCodeModsSettingMenu");
    let row_count = IConfigMenuContentMethods::get_menu_item_content_max(menu_content);
    menu.set_show_row_num(row_count - 1);
}
