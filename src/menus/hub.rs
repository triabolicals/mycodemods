pub(crate) mod mascot;

use engage::{
    BasicMenuExt,
    GameVariableManager,
    Mess,
    app::{
        IUnitMethods,
        BasicMenuSelect,
        GmapMenuContent,
        HubFacilityData,
        HubMenuContent,
        ISingletonClass_1Methods,
        IStructData_1Methods,
        ITitleBarMethods,
        SortieTopMenuContent,
        hubfacilitydata::*,
        hubmenu::*,
        basicmenu::*,
        basicmenuitem::*,
        basicmenuselect::*,
        ISingletonProcInst_1Methods,
        HubVariable_Mascot,
        IBasicMenuContentMethods,
        IBasicMenuItemContentMethods,
        IBitField32Methods,
        RefreshVariable,
        WellSequence,
        WellSequence_UseFlags,
        IHubMiniMapMethods,
        gmapsequence::*,
    },
    prelude::List_1,
    system::collections::generic::IList_1Methods,
    List_1Ext,
    combat::Kaneko,
    unity_engine::{IComponentMethods, IGameObjectMethods}
};
use engage::app::{GameParam, HubUtil, IGameParamMethods, IGmapMapInfoContentMethods, IRandom_2Methods, IStructBase, Random_2, UnitPool};
use unity::{Cast, FromIlInstance, Il2CppString};
use crate::menus::cook::chef::UnitChefProc;
use crate::utils::to_opt;
use crate::proc::*;

#[unity::inject(namespace="MyCodeMods", name="HubQuickMenu", parent=BasicMenu)]
pub struct HubQuickMenu {
    pub mode: i32,
    pub select: BasicMenuSelect,
}
pub fn hub_menu_edit(this: HubMenu) {
    let item_list = this.m_full_menu_item_list();
    let m004 = GameVariableManager::get_bool("G_Cleared_M004");
    if m004 {
        if let Some(pos) = item_list.iter().position(|i| i.try_cast::<HubMenu_MapInfoItem>().is_some()){
            item_list.remove_at(pos as i32);
            item_list.insert(pos as i32 - 1, HubQuickMenuItem::new(HubQuickMenuKind::HubFacilities).into());
            item_list.insert(pos as i32 - 1, HubQuickMenuItem::new(HubQuickMenuKind::HubMarket).into());
        }
    }
    item_list.insert(0, HubQuickMenuItem::new(HubQuickMenuKind::Collector).into());
    this.set_show_row_num(item_list.count());
    
}
impl HubQuickMenu {
    pub fn enable_map(enable: bool) {
        if let Some(mini_map) = to_opt(HubSequence::get_instance()).and_then(|h| to_opt(h.get_mini_map())){
            if enable { mini_map.show_system_menu(); }
            else { mini_map.hide_system_menu(); }
        }
        else if let Some(map_info) = to_opt(GmapSequence::get_instance()).and_then(|h| to_opt(h.m_map_info())){
            if enable { map_info.open_map_details() }
            else { map_info.close(); }
        }
    }
    pub fn create_bind(this: ProcInst, market: bool) {
        let sequence = GameUserData::get_instance().get_sequence().value;
        let menu = HubQuickMenu::instantiate().unwrap();
        let select = BasicMenuSelect::new();
        menu.set_select(select);
        match sequence {
            2 => { IBasicMenuMethods::ctor(menu, Self::create_menu_items(3), SortieTopMenuContent::create()); }
            6 => { IBasicMenuMethods::ctor(menu, Self::create_menu_items(2), GmapMenuContent::create()); }
            _ => {
                let content = HubMenuContent::create();
                if let Some(time) = to_opt(Kaneko::find_in_children(content.get_game_object().get_transform(), "Time")) {
                    time.get_game_object().set_active(false);
                }
                IBasicMenuMethods::ctor(menu, Self::create_menu_items(market as i32), content);
                Self::enable_map(false);
            }
        }
        menu.bind_parent_menu();
        menu.set_anchor_type(BasicMenu_AnchorType::center(), 0.0, 0.0);
        menu.create_bind(this, menu.create_default_desc(), "QuickMenu");
        if let Some(menu) = this.try_cast::<BasicMenu>() { IBasicMenuMethods::close_anime(menu); }
    }
    pub fn create_muscle_bind(self) {
        let menu = HubQuickMenu::instantiate().unwrap();
        IBasicMenuMethods::ctor(menu, Self::create_menu_items(4), HubMenuContent::create());
        menu.bind_parent_menu();
        menu.create_bind(self, menu.create_default_desc(), "QuickMenuMuscle");
        if let Some(menu) = self.try_cast::<BasicMenu>() { IBasicMenuMethods::close_anime(menu); }
    }
    pub fn rebuild(self, mode: i32) {
        if mode != self.mode() {
            let list = self.m_full_menu_item_list();
            list.clear();
            Self::get_menu_types(mode).into_iter().for_each(|i| { list.add(HubQuickMenuItem::new(i).into()); });
            self.set_show_row_num(list.count());
            self.set_mode(mode);
            self.m_status().set(32);
            self.build();
            self.after_build();
        }
    }
    pub fn update_menu_item(self, kind: HubQuickMenuKind, disable: bool) {
        if let Some(item) =
            self.m_full_menu_item_list().iter().flat_map(|v| v.try_cast::<HubQuickMenuItem>())
                .find(|i| i.get_item_kind() == kind)
        {
            if disable {
                item.set_attribute(BasicMenuItem_Attribute::disable());
                if item.m_index() == self.get_select_index() {
                    self.m_menu_content().set_cursor_color(item.get_cursor_color());
                }
            }
            item.on_build();
            item.get_menu_item_content().build_text_color();
            item.get_menu_item_content().build(item);

        }
    }
    fn get_menu_types(mode: i32) -> Vec<HubQuickMenuKind> {
        match mode {
            0 => {  // HubFac
                vec![
                    HubQuickMenuKind::NoticeBoard, HubQuickMenuKind::Animal, HubQuickMenuKind::Arena, HubQuickMenuKind::Cafe,
                    HubQuickMenuKind::Well, HubQuickMenuKind::Mascot, HubQuickMenuKind::Muscle, HubQuickMenuKind::Fishing, HubQuickMenuKind::DragonRide, HubQuickMenuKind::Refresh
                ]
            }
            1 => {  // HubMarket
                vec![HubQuickMenuKind::ItemShop, HubQuickMenuKind::WeaponShop, HubQuickMenuKind::RefineShop,
                     HubQuickMenuKind::FleaMarket, HubQuickMenuKind::GodRoom, HubQuickMenuKind::JukeBox]
            }
            2 => {  // Gmap
                vec![
                    HubQuickMenuKind::ItemShop, HubQuickMenuKind::WeaponShop, HubQuickMenuKind::RefineShop,
                    HubQuickMenuKind::GodRoom, HubQuickMenuKind::Cafe, HubQuickMenuKind::Well, HubQuickMenuKind::Arena
                ]
            }
            3 => {  // Sortie
                vec![HubQuickMenuKind::ItemShop, HubQuickMenuKind::WeaponShop, HubQuickMenuKind::RefineShop, HubQuickMenuKind::Arena, HubQuickMenuKind::NoticeBoard]
            }
            4 => { vec![HubQuickMenuKind::PushUp, HubQuickMenuKind::Situp, HubQuickMenuKind::Squat] }
            _ => vec![HubQuickMenuKind::Invalid],
        }
    }
    pub fn create_menu_items(mode: i32) -> List_1<BasicMenuItem>{
        let list = List_1::<BasicMenuItem>::new();
        Self::get_menu_types(mode).into_iter().for_each(|i| { list.add(HubQuickMenuItem::new(i).into()); });
        list
    }
}
#[unity::injected_methods]
impl HubQuickMenu {
    #[override_virtual(name="OnDispose")]
    pub fn on_dispose(self) {
        if let Some(menu) = self.get_super().try_cast::<HubQuickMenu>(){
            IBasicMenuMethods::open_anime(menu);
        }
        else if let Some(menu) = self.get_super().try_cast::<BasicMenu>(){
            IBasicMenuMethods::open_anime_all(menu);
            Self::enable_map(true);
        }
    }
}


#[repr(i32)]
#[derive(Copy, Clone, PartialEq)]
pub enum HubQuickMenuKind {
    Collector,
    HubFacilities,
    HubMarket,
    NoticeBoard,
    Arena,
    Cafe,
    Well,
    Mascot,
    Muscle,
    Refresh,
    Fishing,
    DragonRide,
    GodRoom,
    ItemShop,
    WeaponShop,
    RefineShop,
    FleaMarket,
    JukeBox,
    Animal,
    PushUp,
    Situp,
    Squat,
    Invalid,
}
impl From<i32> for HubQuickMenuKind {
    fn from(i: i32) -> Self {
        match i {
            0 => HubQuickMenuKind::Collector,
            1 => HubQuickMenuKind::HubFacilities,
            2 => HubQuickMenuKind::HubMarket,
            3 => HubQuickMenuKind::NoticeBoard,
            4 => HubQuickMenuKind::Arena,
            5 => HubQuickMenuKind::Cafe,
            6 => HubQuickMenuKind::Well,
            7 => HubQuickMenuKind::Mascot,
            8 => HubQuickMenuKind::Muscle,
            9 => HubQuickMenuKind::Refresh,
            10 => HubQuickMenuKind::Fishing,
            11 => HubQuickMenuKind::DragonRide,
            12 => Self::GodRoom,
            13 => Self::ItemShop,
            14 => Self::WeaponShop,
            15 => Self::RefineShop,
            16 => Self::FleaMarket,
            17 => Self::JukeBox,
            18 => Self::Animal,
            19 => Self::PushUp,
            20 => Self::Situp,
            21 => Self::Squat,
            _ => Self::Invalid,
        }
    }
}
impl HubQuickMenuKind {
    pub fn get_name(self) -> Il2CppString {
        match self {
            HubQuickMenuKind::Collector => Mess::get("MID_Hub_Area_ItemReturn"),
            HubQuickMenuKind::HubFacilities => Mess::get("MID_MENU_TITLE_HUB"),
            HubQuickMenuKind::HubMarket => Mess::get("MID_MENU_SHOPPING"),
            HubQuickMenuKind::NoticeBoard => Mess::get("MID_MENU_NOTICE_BOARD_TITLE"),
            HubQuickMenuKind::Arena => Mess::get("MID_Hub_Arena"),
            HubQuickMenuKind::Cafe => Mess::get("MID_Hub_Cafe") ,
            HubQuickMenuKind::Well => Mess::get("MID_Hub_Well"),
            HubQuickMenuKind::Mascot => Mess::get("MID_Hub_Mascot_Spot"),
            HubQuickMenuKind::Muscle => Mess::get("MID_Hub_TrainingSpot"),
            HubQuickMenuKind::Refresh => Mess::get("MID_Hub_Refresh"),
            HubQuickMenuKind::Fishing => Mess::get("MID_Hub_Talk_Fishing"),
            HubQuickMenuKind::DragonRide => Mess::get("MID_Hub_Talk_DragonRide"),
            HubQuickMenuKind::GodRoom => Mess::get("MID_Hub_GodRoom"),
            HubQuickMenuKind::ItemShop => Mess::get("MID_Hub_ItemShop"),
            HubQuickMenuKind::WeaponShop => Mess::get("MID_Hub_WeaponShop"),
            HubQuickMenuKind::RefineShop => Mess::get("MID_Hub_Blacksmith"),
            HubQuickMenuKind::FleaMarket => Mess::get("MID_Hub_FreeMarket"),
            HubQuickMenuKind::JukeBox => Mess::get("MID_Hub_CafeTerrace_Jukebox"),
            Self::Animal => Mess::get("MID_Hub_Animal"),
            Self::PushUp => Mess::get("MID_Hub_MuscleExercises_PushUps"),
            Self::Situp => Mess::get("MID_Hub_MuscleExercises_Ads"),
            Self::Squat => Mess::get("MID_Hub_MuscleExercises_Squat"),
            HubQuickMenuKind::Invalid => { "Invalid".into() }
        }
    }
    pub fn build_attribute(self) -> BasicMenuItem_Attribute {
        if self == HubQuickMenuKind::Collector {
            if collector::can_collects_item() { BasicMenuItem_Attribute::enable() }
            else { BasicMenuItem_Attribute::hide() }
        }
        else if self.check_facility() {
            if self == Self::Cafe && !UnitChefProc::is_available(){ BasicMenuItem_Attribute::hide()  }
            else if self.disable() { BasicMenuItem_Attribute::disable() }
            else { BasicMenuItem_Attribute::enable() }
        }
        else { BasicMenuItem_Attribute::hide() }
    }
    pub fn close_menu(self, menu_item: HubQuickMenuItem) {
        match self {
            HubQuickMenuKind::HubFacilities|HubQuickMenuKind::HubMarket|HubQuickMenuKind::Fishing|HubQuickMenuKind::Muscle|
            HubQuickMenuKind::PushUp|HubQuickMenuKind::Situp|HubQuickMenuKind::Squat|HubQuickMenuKind::DragonRide|HubQuickMenuKind::Well => {},
            _ => IBasicMenuMethods::close_anime(menu_item.m_menu()),
        }
    }
    pub fn a_call(self, menu_item: HubQuickMenuItem) -> BasicMenu_Result {
        if menu_item.is_attribute_disable() { BasicMenu_Result::se_miss() }
        else {
            let menu = menu_item.m_menu();
            self.close_menu(menu_item);
            HubQuickMenu::enable_map(false);
            match self {
                HubQuickMenuKind::HubFacilities => {
                    if let Some(menu) = menu_item.get_hub_menu() { menu.rebuild(0); }
                    else { HubQuickMenu::create_bind(menu.into(), false); }
                    BasicMenu_Result::se_cursor()
                }
                HubQuickMenuKind::HubMarket => {
                    if let Some(menu) = menu_item.get_hub_menu() { menu.rebuild(1); }
                    else { HubQuickMenu::create_bind(menu.into(), true); }
                    BasicMenu_Result::se_cursor()
                }
                HubQuickMenuKind::Collector => collector::collect_items(menu.into()),   // Good
                HubQuickMenuKind::NoticeBoard => noticeboard::create_notice_board_bind(menu.into()),    // Good
                HubQuickMenuKind::Fishing => {  // Good
                    if let Some(menu) = menu_item.get_hub_menu() { menu.update_menu_item(self, true); }
                    fishing::fishing_sequence_create_bind(menu.into())
                },
                HubQuickMenuKind::Arena => arena::arena_order_sequence_create_bind(menu.into()),    // Good
                HubQuickMenuKind::Mascot => crate::proc::mascot::mascot_create_bind(menu.into()),   // Good
                HubQuickMenuKind::DragonRide => {
                    if let Some(menu) = menu_item.get_hub_menu() { menu.update_menu_item(self, true); }
                    dragonride::dragon_ride_bind(menu.into())
                },
                HubQuickMenuKind::Muscle => {   // Good
                    if let Some(menu) = menu_item.get_hub_menu() {
                        menu.create_muscle_bind();
                        BasicMenu_Result::se_cursor()
                    }
                    else { BasicMenu_Result::se_miss() }
                }
                HubQuickMenuKind::Well => well::well_create_bind(menu.into()),  // Good
                HubQuickMenuKind::Animal => animal::animal_create_bind(menu.into()),    // Good
                HubQuickMenuKind::Refresh => market::create_bind_refresh(menu.into()),
                HubQuickMenuKind::ItemShop => market::create_bind_market(menu.into(), 0),   // Good
                HubQuickMenuKind::WeaponShop => market::create_bind_market(menu.into(), 1), // Good
                HubQuickMenuKind::RefineShop => market::create_bind_market(menu.into(), 2), // Good
                HubQuickMenuKind::FleaMarket => market::create_bind_market(menu.into(), 3), // Good
                HubQuickMenuKind::JukeBox => market::create_bind_market(menu.into(), 4),
                HubQuickMenuKind::GodRoom => market::create_bind_market(menu.into(), 5),
                HubQuickMenuKind::Cafe => cook::create_cook_bind(menu.into()),
                HubQuickMenuKind::PushUp|HubQuickMenuKind::Situp|HubQuickMenuKind::Squat => {   // Good
                    let kind = self as i32 - HubQuickMenuKind::PushUp as i32;
                    muscle::do_exercise(menu.into(), kind);
                    BasicMenu_Result::se_cursor()
                }
                HubQuickMenuKind::Invalid => BasicMenu_Result::se_miss(),
            }
        }
    }
    pub fn disable(self) -> bool {
        match self {
            Self::Muscle => GameVariableManager::get_number("G_MuscleExercise_GetPrize") != 0,
            Self::Fishing => GameVariableManager::get_number("G_Fishing_PlayCount") == 0,
            Self::DragonRide => GameVariableManager::get_number("G_DragonRide_GetPrize") != 0,
            Self::Well => WellSequence::get_use_flag() == WellSequence_UseFlags::used(),
            Self::Refresh => ["AID_馬ふれあい", "AID_水遊び", "AID_果物狩り"].into_iter().all(|x| RefreshVariable::had_visited(x)),
            Self::Mascot => HubVariable_Mascot::is_done_strok() && HubVariable_Mascot::is_done_eat_food(),
            Self::Cafe => GameVariableManager::get_bool("G_拠点_料理済み"),
            _ => false,
        }
    }
    pub fn check_facility(self) -> bool {
        let hid =
        match self {
            Self::Arena => Some("AID_闘技場"),
            Self::Fishing => Some("AID_釣り"),
            Self::Animal => Some("AID_動物小屋"),
            Self::Muscle => Some("AID_筋肉体操"),
            Self::Well => Some("AID_不思議な井戸"),
            Self::FleaMarket => Some("AID_蚤の市"),
            Self::RefineShop => Some("AID_錬成屋"),
            Self::WeaponShop => Some("AID_武器屋"),
            Self::GodRoom => Some("AID_紋章士の間"),
            Self::Cafe => Some("AID_料理"),
            Self::ItemShop => Some("AID_道具屋"),
            Self::Refresh => Some("AID_水遊び"),
            Self::Mascot => Some("AID_ソラの祭壇"),
            Self::DragonRide => Some("AID_ドラゴンライド"),
            _ => None
        };
        if let Some(aid) = hid { to_opt(HubFacilityData::get(aid.into())).is_some_and(|v| v.is_complete()) }
        else { true }
    }
    pub fn get_help(self) -> Il2CppString {
        match self {
            HubQuickMenuKind::Collector => Mess::get("MID_Hub_Area_ItemReturn"),
            HubQuickMenuKind::HubFacilities => Mess::get("MID_MENU_SOLANEL_INFO_COMMENT"),
            HubQuickMenuKind::HubMarket => Mess::get("MID_SORTIE_SHOP_HELP"),
            HubQuickMenuKind::NoticeBoard => {
                format!("{} / {}", Mess::get("MID_MENU_NOTICE_BOARD_INVESTMENT"), Mess::get("MID_MENU_NOTICE_ACHIEVEMENT")).into()
            }
            HubQuickMenuKind::Arena => {
                let total_training_count = GameParam::get_int("闘技場:回数制限");
                let used_training = GameVariableManager::get_number("G_拠点_闘技場済み");
                let uses = total_training_count - used_training;
                format!("{} {}", Mess::get("MID_Hub_Area_Count"), uses).into()
            }
            HubQuickMenuKind::Cafe => {
                if self.disable() { "".into() }
                else {
                    format!("{} {} ({} {})",
                            Mess::get("MID_Hub_CafeTerrace_Cook_Chef"), engage::app::Mess::get_game_data_name(HubUtil::get_current_cooking_pid()),
                            engage::app::Mess::create_sprite_tag(Mess_IconCategory::system(), "Plus"),  Mess::get("MID_Hub_RefineGodWeapon_Comfirm_Skill_Change_Yes")

                    ).into()
                }
            }
            HubQuickMenuKind::Well => {
                match WellSequence::get_use_flag().value {
                    1 => Mess::get("MID_Hub_Well_ItemExchange_Message"),
                    2 => Mess::get("MID_Hub_Area_ItemReturn"),
                    _ => Mess::get("MID_Hub_Well_Nothing2"),
                }
            }
            HubQuickMenuKind::Mascot => {
                if HubVariable_Mascot::is_found() { Mess::get("MID_Hub_Mascot_After") }
                else { Mess::get("MID_Hub_Mascot") }
            }
            HubQuickMenuKind::Muscle => {
                if self.disable() { "".into() }
                else {
                    let hero = UnitPool::get_hero(false);
                    engage::app::Mess::get_2("MID_Hub_Arena_Start_Confirm", hero.get_name())
                }
            }
            HubQuickMenuKind::Refresh => Mess::get("MID_TUT_HUB_REFRESH_0"),
            HubQuickMenuKind::GodRoom => Mess::get("MID_H_Hub_GodRoom"),
            HubQuickMenuKind::FleaMarket => Mess::get("MID_TUT_HUB_FLEAMARKET_0"),
            HubQuickMenuKind::JukeBox => Mess::get("MID_Hub_CafeTerrace_H_Jukebox"),
            HubQuickMenuKind::Animal => Mess::get("MID_TIPS_OTHER_13"),
            HubQuickMenuKind::PushUp => Mess::get("MID_Hub_MuscleExercises_PushUps_Rule_2"),
            HubQuickMenuKind::Situp => Mess::get("MID_Hub_MuscleExercises_Ads_Rule_2"),
            HubQuickMenuKind::Squat => Mess::get("MID_Hub_MuscleExercises_Squat_Rule_2"),
            HubQuickMenuKind::Fishing => {
                let count = GameVariableManager::get_number("G_Fishing_PlayCount");
                if count == 0 { "".into() }
                else { format!("{}: {}", Mess::get("MID_Hub_Fishing_Limit"), count).into() }
            }
            _ => { "".into() }
        }

    }
}

#[unity::inject(namespace="MyCodeMods", name="HubQuickMenuItem", parent=HubMenu_FriendItem)]
pub struct HubQuickMenuItem { pub kind: i32, }
impl HubQuickMenuItem {
    pub fn get_hub_menu(self) -> Option<HubQuickMenu> { self.m_menu().try_cast::<HubQuickMenu>() }
    pub fn set_menu_item_kind(self, kind: HubQuickMenuKind){ self.set_kind(kind as i32); }
    pub fn get_item_kind(self) -> HubQuickMenuKind { HubQuickMenuKind::from(self.kind()) }
    pub fn new(kind: HubQuickMenuKind) -> Self {
        let item = Self::instantiate().unwrap();
        IBasicMenuItemMethods::ctor(item);
        item.set_menu_item_kind(kind);
        item
    }
}
#[unity::injected_methods]
impl HubQuickMenuItem {
    #[override_virtual(name="ACall")] pub fn a_call(self) -> BasicMenu_Result { self.get_item_kind().a_call(self) }
    #[override_virtual(name="PlusCall")] pub fn plus_call(self) -> BasicMenu_Result {
        if self.get_item_kind() == HubQuickMenuKind::Cafe && !self.get_item_kind().disable() {
            let chef = UnitChefProc::instantiate().unwrap();
            let descs = chef.create_descs();
            let menu = self.m_menu();
            IBasicMenuMethods::close_anime(menu);
            chef.create_bind(menu, descs, Il2CppString::null());
            BasicMenu_Result::se_cursor()
        } 
        else if self.get_item_kind() == HubQuickMenuKind::Well && WellSequence::get_use_flag() != WellSequence_UseFlags::not_use() {
            well::bind_well_preview(self.m_menu().into());
            BasicMenu_Result::se_cursor()
        }
        else { BasicMenu_Result::pass() }
    }
    #[override_virtual(name="XCall")] pub fn x_call(self) -> BasicMenu_Result  {
        if self.get_item_kind() == HubQuickMenuKind::Well && WellSequence::get_use_flag() != WellSequence_UseFlags::not_use() {
            let rng = Random_2::get_system().get_value();
            WellSequence::set_seed(rng);
            engage::app::GameMessage::create_key_wait(self.m_menu(), Mess::get("MID_Hub_Well_ItemExchange_Message"));
            BasicMenu_Result::se_cursor()
        }
        else { BasicMenu_Result::pass() }
    }
    #[override_virtual(name="BuildAttribute")] pub fn build_attribute(self) -> BasicMenuItem_Attribute { self.get_item_kind().build_attribute() }
    #[override_virtual(name="GetName")] pub fn get_name(self) -> Il2CppString { self.get_item_kind().get_name() }
    #[override_virtual(name="GetMapAttribute")] pub fn map_attribute(self) -> BasicMenuItem_Attribute { self.get_item_kind().build_attribute() }
    #[override_virtual(name="GetHelpText")] pub fn get_help_text(self) -> Il2CppString { self.get_item_kind().get_help() }
}
