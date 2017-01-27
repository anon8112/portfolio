#[macro_use]
extern crate webplatform;
extern crate libc;
extern crate rustc_serialize;
extern crate portfolio_shared;

pub mod interop;
pub mod http;
pub mod html;
mod components;

fn main() {
    //let b = http::get("/test.html");
    
    let doc = webplatform::init();
    {
        let body = doc.element_query("body").unwrap();
        body.html_set(html::NAVBAR);
        body.html_append("<div id='container'></div>");
        components::set_route("");
        components::navbar::navbar(&doc);
        webplatform::spin();
    }
}

