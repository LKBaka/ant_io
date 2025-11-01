use std::{cell::RefCell, collections::HashMap, fs, path::Path, rc::Rc};

use once_cell::sync::Lazy;

use rust_ant::{
    builtin::builtin_func::{ant_err, ant_ok},
    byte_code_vm::{constants::NONE_OBJ, vm::vm::Vm},
    obj_enum::object::Object,
    object::{
        ant_class::AntClass, ant_method::{Method, MethodType}, ant_native_function::create_ant_native_function, ant_string::AntString, object::IAntObject
    },
};

use crate::io_err::{IoErrorType, create_io_err};

pub static FILE_CLASS: Lazy<AntClass> = Lazy::new(|| {
    let read_text_utf8_func = |_vm: &mut Vm, args: Vec<Rc<RefCell<Object>>>| {
        let o = args[0].borrow();

        let me = match &*o {
            Object::AntClass(clazz) => clazz,
            _ => return Err(format!("expected an class (self) got {}", o.inspect())),
        };

        let file_path = match match me.map.get("file_path") {
            Some(it) => it,
            None => {
                return Err(format!(
                    "object '{}' has no field 'file_path'",
                    me.inspect()
                ));
            }
        } {
            Object::AntString(s) => s,
            it => return Err(format!("expected an string path, got: {}", it.inspect())),
        };

        let file_path = Path::new(&file_path.value);

        if !file_path.exists() {
            return Ok(Some(ant_err(Object::AntClass(create_io_err(
                IoErrorType::FileNotFound,
                &format!("cannot found file from file path: '{}'", file_path.to_str().unwrap()),
            )))))
        }

        let read_file_result = fs::read_to_string(file_path);

        match read_file_result {
            Ok(s) => Ok(Some(ant_ok(Object::AntString(AntString::new(s))))),
            Err(err) => Ok(Some(ant_err(Object::AntClass(create_io_err(
                IoErrorType::Unknown,
                &err.to_string(),
            ))))),
        }
    };

    let write_text_utf8_func = |_vm: &mut Vm, args: Vec<Rc<RefCell<Object>>>| {
        let o = args[0].borrow();

        let me = match &*o {
            Object::AntClass(clazz) => clazz,
            _ => return Err(format!("expected an class (self) got {}", o.inspect())),
        };

        let file_path = match match me.map.get("file_path") {
            Some(it) => it,
            None => {
                return Err(format!(
                    "object '{}' has no field 'file_path'",
                    me.inspect()
                ));
            }
        } {
            Object::AntString(s) => s,
            it => return Err(format!("expected an string path, got: {}", it.inspect())),
        };

        let file_path = Path::new(&file_path.value);

        let contents_o = args[1].borrow();
        let contents = match &*contents_o {
            Object::AntString(s) => &s.value,
            it => return Err(format!("expected an string content, got: {}", it.inspect())),
        };

        let write_file_result = fs::write(file_path, contents);

        match write_file_result {
            Ok(_) => Ok(Some(ant_ok(NONE_OBJ.clone()))),
            Err(err) => Ok(Some(ant_err(Object::AntClass(create_io_err(
                IoErrorType::Unknown,
                &err.to_string(),
            ))))),
        }
    };

    let mut file_class_map = HashMap::new();

    file_class_map.insert(
        String::from("file_path"),
        NONE_OBJ.clone()
    );

    file_class_map.insert(
        String::from("read_text_utf8"),
        Object::Method(Method {
            me: None,
            func: MethodType::NativeFunction(
                create_ant_native_function(None, read_text_utf8_func)
            )
        }),
    );

    file_class_map.insert(
        String::from("write_text_utf8"),
        Object::Method(Method {
            me: None,
            func: MethodType::NativeFunction(
                create_ant_native_function(None, write_text_utf8_func)
            )
        }),
    );

    AntClass::from(("File", file_class_map))
});

pub fn create_file_object_from_str(path: &str) -> AntClass {
    let mut file_obj = FILE_CLASS.clone();

    file_obj.map.insert(
        String::from("file_path"),
        Object::AntString(AntString::new(path.to_owned()))
    );

    file_obj
}

pub fn for_ant_create_file_object_from_str(_vm: &mut Vm, args: Vec<Rc<RefCell<Object>>>) -> Result<Option<Object>, String> {
    let o = args[0].borrow();

    let path = match &*o {
        Object::AntString(s) => &s.value,
        _ => return Err(format!("expected an string path to open file"))
    };

    Ok(Some(Object::AntClass(create_file_object_from_str(path))))
}