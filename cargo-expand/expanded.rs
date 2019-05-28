#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
pub fn add(a: u16, b: u8) -> u16 {
    a + (b as u16)
}
#[export_name = "add"]
#[allow(non_snake_case)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wasm_bindgen_generated_add(
    arg0: <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi,
    arg1: <u8 as wasm_bindgen::convert::FromWasmAbi>::Abi,
) -> <u16 as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
    let _ret = {
        let mut __stack = unsafe { wasm_bindgen::convert::GlobalStack::new() };
        let arg0 =
            unsafe { <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(arg0, &mut __stack) };
        let arg1 =
            unsafe { <u8 as wasm_bindgen::convert::FromWasmAbi>::from_abi(arg1, &mut __stack) };
        add(arg0, arg1)
    };
    <u16 as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret, &mut unsafe {
        wasm_bindgen::convert::GlobalStack::new()
    })
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe_add() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <u16 as WasmDescribe>::describe();
    <u8 as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_33567a2dc84b370e: [u8; 100usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b".\x00\x00\x00{\"schema_version\":\"0.2.45\",\"version\":\"0.2.45\"}.\x00\x00\x00\x01\x00\x00\x00\x02\x01a\x01b\x03add\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x15test-c004a3a0c48629cf\x00"
};
