pub use super::*;
pub(crate) mod skillinheritanceitem;
pub(crate) mod skillinheritance;

pub fn inject_sortie_menu_items() {
    if cobapi::injection::register::<skillinheritance::SortieSkillInheritanceMenuItem>().is_ok() { println!("SortieSkillInheritanceMenuItem is Injected"); }
    skillinheritanceitem::skill_inheritance_vtable_edit();

}