use engage::app::{GameUserData, IGameUserData, IGameUserDataMethods, ISingletonClass_1Methods, IStructData_1Methods};
use engage::unity_engine::{IComponent, IComponentMethods, IGameObjectMethods};
use unity::{Cast, Class, Il2CppString};

pub fn to_opt<C: Cast>(obj: C) -> Option<C> { if !obj.is_null() { Some(obj) } else { None } }

pub fn enable_component<O: IComponentMethods + Cast>(component: O, enable: bool) {
    if !component.is_null(){
        let go =     component.get_game_object();
        if !go.is_null() {
            go.set_active(enable);
        }
    }
}
pub fn add_bond(amount: i32) {
    let user = GameUserData::get_instance();
    user.set_m_piece_of_bond(amount + user.get_piece_of_bond());
}

pub fn override_vtable(namespace: &str, name: &str, method_name: &str, method: *mut u8) {
    if let Some(virtual_method) = Class::try_lookup(namespace, name).ok()
        .map(|v| v.raw_mut())
        .and_then(|v| v.get_virtual_method_mut(method_name))
    {
        virtual_method.method_ptr = method;
    }
    else {
        println!("Did not find {}.{}$${}", namespace, name, method_name);
    }
}
pub fn override_vtable_by_slot(namespace: &str, name: &str, slot: usize, method: *mut u8) {
    if let Some(class) = Class::try_lookup(namespace, name).ok()
        .map(|v| v.raw_mut())
    {
        class.get_vtable_mut()[slot].method_ptr = method;
    }
}

pub fn try_get_data_by_key<T>(key: impl Into<Il2CppString>) -> Option<T>
where
    T: IStructData_1Methods<T> + Cast,
{
    to_opt(T::get(key.into()))

}

pub fn try_get_data_by_hash<T>(hash: i32) -> Option<T>
where
    T: IStructData_1Methods<T> + Cast,
{
    to_opt(T::try_get_from_hash(hash))

}