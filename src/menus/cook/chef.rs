use engage::{app::{
    unit::*, force::*, unitinfo::*, procinst::*, gameui::*,
    shopunitselectmenuitem::*, shopunitselectmenu::*,
    shopunitselectroot::*, shopunitselectmenucontent::*,
    IStructData_1Methods,
    ShopUnitSelectStatus, ResourceManager_2, BackgroundManager,
    Proc, ProcBoolMethod, ProcVoidMethod, ProcDesc,
    BasicMenuContent, BasicMenuItem, BasicMenu_Result, CookData, HubUtil,
    IBasicMenuMethods
}, ProcBoolMethodExt, ProcVoidMethodExt, TitleBar, system::IDelegate, ForceExt, prelude::List_1, system::Action, system::collections::generic::IList_1Methods, unity_engine::IGameObjectMethods, GameVariableManager};
use unity::{Array, Cast, FromIlInstance, Il2CppString, IlNull, OptionalMethod};
use crate::menus::hub::{HubQuickMenu, HubQuickMenuKind};

#[unity::inject(namespace="MyCodeMods", name="UnitChefProc", parent=ProcInst)]
pub struct UnitChefProc { pub shop_unit_select_root: ShopUnitSelectRoot, }
impl UnitChefProc {
    pub fn create_descs(self) -> Array<ProcDesc> {
        let descs = [
            Proc::call_2(ProcVoidMethod::from_fn(self, Self::load_shop_ui).unwrap()),
            Proc::wait_while_true_2(ProcBoolMethod::from_fn(self, Self::is_loading).unwrap()),
            Proc::call_2(ProcVoidMethod::from_fn(self, Self::create_menu_bind).unwrap()),
            Proc::call_2(ProcVoidMethod::from_fn(self, Self::unload).unwrap()),
            Proc::end()
        ];
        Array::from_slice(&descs).unwrap()
    }
    extern "C" fn load_shop_ui(_: UnitChefProc, _: OptionalMethod) { ShopUnitSelectRoot::load_prefab_async() }
    extern "C" fn is_loading(_: UnitChefProc, _: OptionalMethod) -> bool { ShopUnitSelectRoot::is_loading_prefab() }
    extern "C" fn unload(proc : UnitChefProc, _: OptionalMethod) {
        if !proc.shop_unit_select_root().is_null() {
            TitleBar::close_header();
            if BackgroundManager::is_bind() { BackgroundManager::unbind(); }
            ShopUnitSelectRoot::destroy(proc.shop_unit_select_root());
        }
        if let Some(hub_menu) = proc.get_super().try_cast::<HubQuickMenu>(){ 
            hub_menu.update_menu_item(HubQuickMenuKind::Cafe, false);    
            IBasicMenuMethods::open_anime(hub_menu);
        }
        ShopUnitSelectRoot::unload_prefab()
    }
    extern "C" fn create_menu_bind(proc_inst: UnitChefProc, _: OptionalMethod) {
        let canvas = GameUI::get_canvas(GameUI_Priority::default());
        let transform = canvas.get_transform();
        let x = ResourceManager_2::instantiate_2("UI/Hub/Shop/Prefabs/ShopUnitSelectRoot", transform);
        if !x.is_null() {
            let root = x.get_component::<ShopUnitSelectRoot>();
            if !root.is_null() {
                proc_inst.set_shop_unit_select_root(root);
                TitleBar::open_header("Chef Selection", "Select a unit to cook.", Il2CppString::null());
                if !BackgroundManager::is_bind() { BackgroundManager::bind_2(); }
                create_menu(root, proc_inst);
            }
        }
    }
    pub fn is_available() -> bool {
        Force::get(Force_Type::absent()).iter()
            .filter(|unit| !unit.is_hero() && !CookData::get(unit.get_pid()).is_null())
            .count() > 2
    }
}
#[unity::inject(namespace="MyCodeMods", name="UnitChefSelectionMenuItem", parent=ShopUnitSelectMenuItem)]
pub struct UnitChefSelectionMenuItem {}

#[unity::injected_methods]
impl UnitChefSelectionMenuItem {
    #[override_virtual(name="ACall")]
    pub fn a_call(self) -> BasicMenu_Result {
        HubUtil::set_current_cooking_pid(self.get_unit().get_pid());
        GameVariableManager::set_number("G_Taste", -1);
        BasicMenu_Result::close_decide()
    }
    #[override_virtual(name="BCall")]
    pub fn b_call(self) -> BasicMenu_Result {
        if !self.m_decide_event_handler().is_null() {
            if let Some(root) = self.m_decide_event_handler().m_target().try_cast::<ShopUnitSelectRoot>() {
                unsafe { close_status(root.m_shop_unit_select_status(), None); }
            }
        }
        UnitInfo::set_unit(UnitInfo_Side::left(), Unit::null(), false, true, true, Action::null());
        BasicMenu_Result::close_cancel()
    }
    #[override_virtual(name="GetName")]
    pub fn get_name(self) -> Il2CppString { "Recruit Units".into() }
}

fn create_menu(this: ShopUnitSelectRoot, proc: impl Into<ProcInst>) {
    let game_object = this.m_shop_unit_select_menu_object();
    let menu_item_list = List_1::<UnitChefSelectionMenuItem>::new();
    let menu_content = game_object.get_component::<ShopUnitSelectMenuContent>();
    let max = IShopUnitSelectMenuContentMethods::get_menu_item_content_max(menu_content);
    let select = ShopUnitSelectMenu_SelectEventHandler::new(this.into(), ShopUnitSelectRoot::on_select_menu_item_method_info().into());
    let menu = ShopUnitSelectMenu::instantiate().unwrap();
    let decided = ShopUnitSelectMenu_DecideEventHandler::new(this.into(), ShopUnitSelectRoot::on_decide_menu_item_method_info().into());
    Force::get(Force_Type::absent()).iter().for_each(|unit| {
        if !unit.is_hero() && !CookData::get(unit.get_pid()).is_null(){
            let item = UnitChefSelectionMenuItem::instantiate().unwrap();
            IShopUnitSelectMenuItemMethods::ctor(item, unit, decided, select);
            menu_item_list.add(item);
        }
    });
    unsafe { IBasicMenuMethods::ctor(menu, menu_item_list.cast::<List_1<BasicMenuItem>>(), menu_content.cast::<BasicMenuContent>()); }
    menu.set_m_show_god_ring_icon(false);
    menu.set_m_game_object(game_object);
    let descs = menu.create_default_desc();
    menu.create_bind(proc, descs, Il2CppString::null());
    menu.set_show_row_num(max - 1);
    let status = this.m_unit_status_object().get_component::<ShopUnitSelectStatus>();
    this.set_m_shop_unit_select_status(status);
    unsafe { open_status(status, None); }
    UnitInfo::push_mode_model_only();
    UnitInfo::set_visible(true);
}
#[skyline::from_offset(0x247a540)] fn open_status(this: ShopUnitSelectStatus, optional_method: OptionalMethod);
#[skyline::from_offset(0x247ab20)] fn close_status(this: ShopUnitSelectStatus, optional_method: OptionalMethod);
