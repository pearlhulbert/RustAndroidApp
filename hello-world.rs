use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

#[no_mangle]
pub extern fn rust_hello_world() -> &'static str {
    return "hello world?";
}

#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;
    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring};
    // Access our RustBridge class doSomething function
    #[no_mangle]
    pub unsafe extern fn Java_com_rustapp_RustBridge_helloWorld(env: JNIEnv, _: JClass) -> jstring {
        let info = rust_hello_world();
        let output = env.new_string(info.unwrap()).expect("Couldn't create string from Android Function!");
        output.into_inner()
    }
}
