use super::*;
use engage::{
    app::{
        godbond::*,
        godbondholder::*,
        godunit::*,
        battlegrow::*,
        reliancepopupcontroller::*,
        arenaordersequence::*,
        unit::*,
        bondpopupcontroller::*,
        GameSound,
        GodLevelData,
        GodPool,
        IBattleCalculatorMethods,
        IBattleInfoSide,
        IGodLevelDataMethods,
        IStructData_1Methods,
        IUnitMethods,
        AchieveData,
        BasicMenu_Result,
        ResourceManager_2,
        TitleBar_FooterType
    },
    combat::arenacombatsequence::*,
    combat::Character,
    BasicMenuExt,
    unity_engine::{IComponentMethods, Object_2}
};
use unity::{method_info, Array, Cast, FromIlInstance, IlNull, OptionalMethod};
use crate::support::update_reliance;
use crate::utils::to_opt;
const ARENA_EXP: [&str; 6] = ["闘技場:支援値(仲間・敗北)", "闘技場:絆値(紋章・敗北)", "闘技場:支援値(仲間・勝利)", "闘技場:絆値(紋章・勝利)", "闘技場_支援値(仲間・引分)", "闘技場:絆値(紋章・引分)"];

pub fn arena_order_sequence_create_bind(this: ProcInst) -> BasicMenu_Result {
    ArenaOrderSequence::create_bind(this);
    engage::app::TitleBar::get_instance().open_footer(TitleBar_FooterType::piece_of_bond());
    if let Some(arena) = this.get_child().try_cast::<ArenaOrderSequence>() {
        let descs = arena.m_descs();
        [29, 30, 32, 33, 38].into_iter().for_each(|x| descs.set(x, Proc::call_2(ProcVoidMethod::from_fn(arena, skip).unwrap())));
        descs.set(2, Proc::call_2(ProcVoidMethod::new(arena.into(), ArenaOrderSequence::background_in_method_info().into())));
        descs.set(35, Proc::call_2(ProcVoidMethod::from_fn(arena, arena_combat_create_bind).unwrap()));
        // ArenaOrderSequence$$Finish
        descs.set(36, Proc::call_2(ProcVoidMethod::from_fn(arena, arena_finish_training).unwrap()));
        descs.set(44, Proc::call_2(ProcVoidMethod::from_fn(arena.into(), background_out).unwrap()));
    }
    BasicMenu_Result::se_cursor()
}
extern "C" fn skip(_proc: ArenaOrderSequence, _optional_method: OptionalMethod) {}
extern fn background_out(this: ArenaOrderSequence, _optional_method: OptionalMethod) {
    this.background_out();
    if let Some(menu) = this.get_super().try_cast::<BasicMenu>() { BasicMenuExt::open_anime(menu); }
}
extern "C" fn arena_combat_create_bind(arena: ArenaOrderSequence, _optional_method: OptionalMethod){
    let calc = arena.get_calculator();
    let emblem_battle = arena.get_is_emblem_battle();
    let bond_exp = if arena.get_training_type().value == 1 { arena.get_bond_exp() }
    else {
        let index =
            match (calc.get_dead_side().value) {
                0 => 0,
                1 => 2,
                _ => 4,
            } + emblem_battle as usize;
        engage::app::GameParam::get_int(ARENA_EXP[index])
    };
    let arena_combat = ArenaCombatSequence::new(calc, arena.get_sim_calculator(), arena.get_is_emblem_battle(), arena.get_is_special_battle(), bond_exp);

    let descs =
        [
            Proc::call_2(ProcVoidMethod::new(arena_combat.into(), ArenaCombatSequence::result_method_info().into())),
            Proc::call_2(ProcVoidMethod::from_fn(arena_combat, arena_battle_grow).unwrap()),
            Proc::call_2(ProcVoidMethod::new(arena_combat.into(), ArenaCombatSequence::grow3_method_info().into())),
            Proc::call_2(ProcVoidMethod::from_fn(arena_combat, arena_combat_exit).unwrap()),
            Proc::end()
        ];
    if let Some(descs) = Array::<ProcDesc>::from_slice(&descs) {
        arena_combat.create_bind(arena, descs, "ArenaSkipSequence");
    }
}
extern "C" fn arena_finish_training(arena: ArenaOrderSequence, _optional_method: OptionalMethod) {
    let unit = arena.get_training_unit();
    if !arena.m_god_unit().is_null() { unit.set_god_unit(arena.m_god_unit()); }
    else if !arena.m_ring().is_null() { unit.set_ring(arena.m_ring()); }
    let dead_side = arena.get_calculator().get_dead_side();
    if !arena.get_is_emblem_battle() {
        AchieveData::add_count_unit_battle();
        if dead_side.value == 1 { AchieveData::add_count_unit_battle_win(); }
    }
    else {
        AchieveData::add_count_god_battle();
        if dead_side.value == 1 { AchieveData::add_count_god_battle_win(); }
    }
    if arena.get_training_type().value == 0 {
        let arena_count = GameVariableManager::get_number("G_拠点_闘技場済み");
        GameVariableManager::set_number("G_拠点_闘技場済み", arena_count+1);
    }
    collect_achievements();
    update_reliance();
}
extern "C" fn arena_battle_grow(this: ArenaCombatSequence, optional_method: OptionalMethod){
    let calculator = this.get_calculator();
    if !this.get_is_special_battle() { BattleGrow::create_bind(this, calculator, 0.0); }
    if calculator.get_dead_side().value == 1 { GameSound::post_event("Good2", Character::null()); }
    else { GameSound::post_event("Bad", Character::null()); }
    let unit_a = calculator.get_side_2(0).m_unit();
    let unit_b = calculator.get_side_2(1).m_unit();
    if !this.get_is_emblem_battle() {
        engage::app::UnitReliance::try_add_exp_by_hub(unit_a, unit_b, this.get_bond_exp());
        let pop = ReliancePopUpController::create();
        if !pop.is_null() { pop.pop_up(unit_a, unit_b, 2); }
        this.set_m_reliance_controller(pop)
    }
    else {
        if let Some(gid) = unit_b.get_pid().to_rust_string().split("_").last() {
            let god_unit = if gid == "ルフレ" { GodPool::try_get("GID_クロム", false) }
            else { GodPool::try_get(format!("GID_{}", gid), false) };
            if !god_unit.is_null() {
                let bond = god_unit.m_bonds().get(unit_a);
                let level = bond.m_level() as i32;
                bond.try_add_exp(this.get_bond_exp());
                let mut level_exp = bond.get_level_from_exp();

                let mut levels = 0;
                if let Some(level_data) = to_opt(GodLevelData::try_get_2(level_exp + 1)) {
                    if (level_data.get_exp() - 1) != bond.get_exp() {
                        levels = level_exp - level;
                    }
                    else if !level_data.get_reliance_level().is_null() { level_exp += 1; }
                }
                levels = level_exp - level;
                if levels > 0 { for _ in 0..levels { bond.level_up(); } }
                unit_a.inherit_aptitude(god_unit);
            }
        }
    }
}
extern "C" fn arena_combat_exit(this: ArenaCombatSequence, optional_method: OptionalMethod) {
    ResourceManager_2::release_global(this.start_telop_path());
    ResourceManager_2::release_global(this.get_result_telop_path());
    let bond_controller = this.m_bond_controller();
    if !bond_controller.is_null() {
        let go = bond_controller.get_game_object();
        Object_2::destroy_2(go);
    }
    let reliance_controller = this.m_reliance_controller();
    if !reliance_controller.is_null() {
        let go = reliance_controller.get_game_object();
        Object_2::destroy_2(go);
    }
    if !this.get_is_emblem_battle() { ReliancePopUpController::unload_prefab(); }
    else { BondPopUpController::unload_prefab(); }
}