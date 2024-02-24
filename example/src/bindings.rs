// Generated by `wit-bindgen` 0.18.0. DO NOT EDIT!
const _: () = {
  
  #[doc(hidden)]
  #[export_name = "hello-world"]
  #[allow(non_snake_case)]
  unsafe extern "C" fn __export_hello_world(arg0: i32,arg1: i32,arg2: i32,arg3: i32,) -> i32 {
    #[allow(unused_imports)]
    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
    
    // Before executing any other code, use this function to run all static
    // constructors, if they have not yet been run. This is a hack required
    // to work around wasi-libc ctors calling import functions to initialize
    // the environment.
    //
    // This functionality will be removed once rust 1.69.0 is stable, at which
    // point wasi-libc will no longer have this behavior.
    //
    // See
    // https://github.com/bytecodealliance/preview2-prototyping/issues/99
    // for more details.
    #[cfg(target_arch="wasm32")]
    wit_bindgen::rt::run_ctors_once();
    
    let result1 = <_GuestImpl as Guest>::hello_world(wit_bindgen::rt::bool_lift(arg0 as u8), match arg1 {
      0 => None,
      1 => {
        let e = {
          let len0 = arg3 as usize;
          let bytes0 = Vec::from_raw_parts(arg2 as *mut _, len0, len0);
          
          wit_bindgen::rt::string_lift(bytes0)
        };
        Some(e)
      }
      _ => wit_bindgen::rt::invalid_enum_discriminant(),
    });
    let ptr2 = _RET_AREA.0.as_mut_ptr() as i32;
    let vec3 = (result1.into_bytes()).into_boxed_slice();
    let ptr3 = vec3.as_ptr() as i32;
    let len3 = vec3.len() as i32;
    ::core::mem::forget(vec3);
    *((ptr2 + 4) as *mut i32) = len3;
    *((ptr2 + 0) as *mut i32) = ptr3;
    ptr2
  }
  
  const _: () = {
    #[doc(hidden)]
    #[export_name = "cabi_post_hello-world"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __post_return_hello_world(arg0: i32,) {
      let l0 = *((arg0 + 0) as *const i32);
      let l1 = *((arg0 + 4) as *const i32);
      wit_bindgen::rt::dealloc(l0, (l1) as usize, 1);
    }
  };
};
use super::Component as _GuestImpl;
pub trait Guest {
  fn hello_world(goodbye: bool,name: Option<wit_bindgen::rt::string::String>,) -> wit_bindgen::rt::string::String;
}

#[allow(unused_imports)]
use wit_bindgen::rt::{alloc, vec::Vec, string::String};

#[repr(align(4))]
struct _RetArea([u8; 8]);
static mut _RET_AREA: _RetArea = _RetArea([0; 8]);

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:example"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 199] = [0, 97, 115, 109, 13, 0, 1, 0, 0, 25, 22, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 45, 101, 110, 99, 111, 100, 105, 110, 103, 4, 0, 7, 75, 1, 65, 2, 1, 65, 3, 1, 107, 115, 1, 64, 2, 7, 103, 111, 111, 100, 98, 121, 101, 127, 4, 110, 97, 109, 101, 0, 0, 115, 4, 0, 11, 104, 101, 108, 108, 111, 45, 119, 111, 114, 108, 100, 1, 1, 4, 1, 25, 99, 111, 109, 112, 111, 110, 101, 110, 116, 58, 101, 120, 97, 109, 112, 108, 101, 47, 101, 120, 97, 109, 112, 108, 101, 4, 0, 11, 13, 1, 0, 7, 101, 120, 97, 109, 112, 108, 101, 3, 0, 0, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 50, 49, 46, 48, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 56, 46, 48];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
