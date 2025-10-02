use std::{cell::RefCell, collections::HashMap, rc::Rc};

use rust_ant::{
    byte_code_vm::vm::vm::Vm,
    obj_enum::object::Object,
    object::{ant_class::AntClass, ant_native_function::create_ant_native_function},
};

use crate::{io_console::CONSOLE_CLASS, io_file::for_ant_create_file_object_from_str};

pub fn test_func(_vm: &mut Vm, _args: Vec<Rc<RefCell<Object>>>) -> Result<Option<Object>, String> {
    println!("hey i am a test function. don't call me please. (from module io)");
    Ok(None)
}

#[unsafe(no_mangle)]
pub fn get_all_exports() -> AntClass {
    let mut io_mod = HashMap::new();

    io_mod.insert(
        String::from("__donot_call_me_please__"),
        Object::AntNativeFunction(create_ant_native_function(None, test_func)),
    );

    io_mod.insert(
        String::from("open"),
        Object::AntNativeFunction(create_ant_native_function(
            None,
            for_ant_create_file_object_from_str,
        )),
    );

    io_mod.insert(
        String::from("console"),
        Object::AntClass(CONSOLE_CLASS.clone())
    );

    AntClass::from(io_mod)
}
