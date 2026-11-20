pub(crate) mod quality;
pub(crate) mod chef;

pub fn install_cook_menus() {
    quality::taste_selection_install();
    if cobapi::injection::register::<chef::UnitChefProc>().is_ok() { println!("UnitChefProc Injected"); }
    if cobapi::injection::register::<chef::UnitChefSelectionMenuItem>().is_ok() { println!("UnitChefSelectionMenuItem Injected"); }
}