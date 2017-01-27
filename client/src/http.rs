use libc;

use std::ffi::{CString, CStr};
use std::str;
use interop::Interop;

pub fn get(path: &str) -> String {
    let a = js!{ (path) b"\
        var xmlHttp = new XMLHttpRequest();\
    	xmlHttp.open( 'GET', UTF8ToString($0), false );\
    	xmlHttp.send( null );\
        return allocate(intArrayFromString(xmlHttp.responseText), 'i8', ALLOC_STACK);\
        \0"
    };
    unsafe {
        str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
    }
}
pub fn post(path: &str, data: &str) -> String {
    let url = format!("{}{}", "http://localhost:3000", path);
    let a = js!{ (url.as_str(), data) b"\
        var xmlHttp = new XMLHttpRequest();\
    	xmlHttp.open( 'POST', UTF8ToString($0), false );\
    	xmlHttp.send( UTF8ToString($1) );\
        return allocate(intArrayFromString(xmlHttp.responseText), 'i8', ALLOC_STACK);\
        \0"
    };
    unsafe {
        str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
    }
}


