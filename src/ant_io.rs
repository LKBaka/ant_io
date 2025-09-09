use std::{cell::RefCell, rc::Rc};

use rust_ant::{
    obj_enum::object::Object,
    object::{ant_class::AntClass, ant_native_function::create_ant_native_function},
};

fn test_func(_args: Vec<Rc<RefCell<Object>>>) -> Option<Object> {
    println!("hey i am an test function. don't call me please. (from module io)");
    None
}

#[unsafe(no_mangle)]
pub fn get_all_exports() -> AntClass {
    let mut map = hashbrown::HashMap::new();

    map.insert(
        String::from("__donot_call_me_please__"),
        Object::AntNativeFunction(create_ant_native_function(None, test_func)),
    );

    AntClass::from(map)
}
