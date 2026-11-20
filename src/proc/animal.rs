use engage::app::{AnimalSequence, BasicMenu_Result, IAnimalSequenceMethods, ProcBoolMethod};
use super::*;


pub fn animal_create_bind(this: ProcInst) -> BasicMenu_Result {
    AnimalSequence::create_bind(this);
    if let Some(proc) = this.get_child().try_cast::<AnimalSequence>() {
        let descs = proc.m_descs();
        descs.set(5, Proc::jump_true_3(ProcBoolMethod::from_fn(proc, animal_is_dirty).unwrap(), 2));
        descs.set(17, Proc::call_2(ProcVoidMethod::from_fn(proc, animal_exit).unwrap()));
    }
    BasicMenu_Result::se_cursor()
}
extern "C" fn animal_is_dirty(animal_sequence: AnimalSequence, _optional_method: OptionalMethod) -> bool {
    if is_hub_solanel() { animal_sequence.is_dirty() } else { false }
}
extern "C" fn animal_exit(animal_sequence: AnimalSequence, _optional_method: OptionalMethod) {
    animal_sequence.exit();
    open_menu(animal_sequence.into(), None);
}