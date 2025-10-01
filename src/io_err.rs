use std::collections::HashMap;

use once_cell::sync::Lazy;

use rust_ant::{
    byte_code_vm::constants::NONE_OBJ, obj_enum::object::Object, object::{ant_class::AntClass, ant_method::{Method, MethodType}, ant_native_function::create_ant_native_function, ant_string::AntString, object::IAntObject}
};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum IoErrorType {
    Unknown,
    FileNotFound
}

pub static IO_ERROR_CLASS: Lazy<AntClass> = Lazy::new(|| {
    let mut io_err_class = HashMap::new();
    
    io_err_class.insert(
        String::from("error_type"),
        NONE_OBJ.clone()
    );

    io_err_class.insert(
        String::from("message"),
        NONE_OBJ.clone()
    );

    io_err_class.insert(
        String::from("to_string"),
        Object::Method(Method {
            me: None,
            func: MethodType::NativeFunction(create_ant_native_function(
                None, |_vm, args| {
                    let o = args[0].borrow();
                    
                    let me = match &*o {
                        Object::AntClass(clazz) => clazz,
                        _ => return Err(format!("expected an class (self) got {}", o.inspect())),
                    };
                    
                    let error_type = match match me.map.get("error_type") {
                        Some(it) => it,
                        None => {
                            return Err(format!(
                                "object '{}' has no field 'error_type'",
                                me.inspect()
                            ));
                        }
                    } {
                        Object::AntString(s) => s,
                        it => return Err(format!("expected an string object, got: {}", it.inspect())),
                    };

                    let message = match match me.map.get("message") {
                        Some(it) => it,
                        None => {
                            return Err(format!(
                                "object '{}' has no field 'message'",
                                me.inspect()
                            ));
                        }
                    } {
                        Object::AntString(s) => s,
                        it => return Err(format!("expected an string object, got: {}", it.inspect())),
                    };

                    Ok(Some(Object::AntString(AntString::new(format!(
                        "io::Error {{ type: {}, message: \"{}\" }}",
                        error_type.value, message.value
                    )))))
                }
            ))
        })
    );

    AntClass::from(io_err_class)
});

pub fn create_io_err(
    error_type: IoErrorType,
    message: &str
) -> AntClass {
    let mut err_obj = IO_ERROR_CLASS.clone();

    err_obj.map.insert(
        String::from("error_type"),
        Object::AntString(AntString::new(format!("{:?}", error_type)))
    );

    err_obj.map.insert(
        String::from("message"),
        Object::AntString(AntString::new(message.to_owned()))
    );

    err_obj
}