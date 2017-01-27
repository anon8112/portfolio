pub mod navbar;
pub mod signup;
pub mod signin;
pub mod home;

use webplatform;
use html;

pub fn get_data(id: &str) -> String {
    let doc = webplatform::init();
    doc.element_query(id).unwrap().html_get()
}

pub fn get_value(id: &str) -> String {
    let doc = webplatform::init();
    doc.element_query(id).unwrap().prop_get_str("value")
}

pub fn set_route(route: &str) {
    let doc = webplatform::init();
    let container = doc.element_query("#container").unwrap();
    match route {
        "sign_up" => {
            container.html_set(html::SIGN_UP);
            signup::handle();
        },
        "sign_in" => container.html_set(html::SIGN_IN),
        _ => container.html_set(html::HOME) 
    };
}
