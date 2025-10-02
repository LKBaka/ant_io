use std::{cell::RefCell, collections::HashMap, rc::Rc};

use once_cell::sync::Lazy;
use std::io::{self, Write};

use rust_ant::{
    byte_code_vm::vm::vm::Vm,
    obj_enum::object::Object,
    object::{
        ant_class::AntClass, ant_native_function::create_ant_native_function, ant_string::AntString, object::IAntObject
    },
};

pub static CONSOLE_CLASS: Lazy<AntClass> = Lazy::new(|| {
    let input = |_vm: &mut Vm, args: Vec<Rc<RefCell<Object>>>| {
        let o = args[0].borrow();

        let propmt = match &*o {
            Object::AntString(s) => s,
            it => return Err(format!("expected an string propmt, got: {}", it.inspect())),
        };

        print!("{}", propmt.value);
        io::stdout().flush().map_err(|e| format!("flush failed: {}", e))?;

        let mut input_str = String::new();
        io::stdin().read_line(&mut input_str).map_err(|e| format!("read failed: {}", e))?;

        let result = Object::AntString(AntString::new(input_str.trim_end().to_string()));

        Ok(Some(result))
    };

    let print = |_vm: &mut Vm, args: Vec<Rc<RefCell<Object>>>| {
        let o = args[0].borrow();
        let end = args[1].borrow();

        let end = match &*end {
            Object::AntString(s) => s,
            it => return Err(format!("expected an string end of line, got: {}", it.inspect())),
        };

        print!("{}{}", o.inspect(), end.value);
        io::stdout().flush().map_err(|e| format!("flush failed: {}", e))?;

        Ok(None)
    };

    let mut console_class_map = HashMap::new();

    console_class_map.insert(
        String::from("input"),
        Object::AntNativeFunction(create_ant_native_function(None, input))
    );

    console_class_map.insert(
        String::from("print"),
        Object::AntNativeFunction(create_ant_native_function(None, print))
    );

    AntClass::from(console_class_map)
});