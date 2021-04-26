
//external crates
use std::os::raw::c_char;
use std::ffi::CString;
use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

lazy_static! {
    static ref SC_GLOBAL: Mutex<Option<RustCallback>> = Mutex::new(None);
}

//#[macro_use] extern crate log;

//static mut STRING_POINTER: *mut c_char = 0 as *mut c_char;

pub struct RustCallback {
    callback: extern "stdcall" fn(*mut c_char),
}

impl RustCallback {
    fn new(read_fn: extern "stdcall" fn(*mut c_char)) -> Self {
        RustCallback {
            callback: read_fn,
        }
    }
}

#[no_mangle]
pub extern "C" fn create(callback: extern "stdcall" fn(*mut c_char))  -> u32 {
    let mut d = SC_GLOBAL.lock().unwrap();
    *d = Some(RustCallback::new(callback));

    0
}

#[no_mangle]
pub extern "C" fn trigger(_x: i32)  ->  u32 {
//    let d = SC_GLOBAL.lock().unwrap();
//    let g = (*d).as_ref().unwrap().callback;
//    g(x);

    0
}

#[no_mangle]
pub extern "C" fn init() {
}


#[no_mangle]
pub extern "C" fn pow(n: i32) -> i32 {
    let d = SC_GLOBAL.lock().unwrap();
    let g = (*d).as_ref().unwrap().callback;

    let to_log: String = format!("pow with param: {}", n);
    
    let raw_str = CString::new(to_log).unwrap().into_raw();

    g(raw_str);
    
    unsafe { 
        let _ = CString::from_raw(raw_str);
    }

    n * n
}
