use webplatform;
use super::set_route;

pub fn navbar(doc: &webplatform::Document) {
    let btn_home = doc.element_query("#link-home").unwrap();
    let btn_signup = doc.element_query("#link-sign-up").unwrap();
    let btn_signin = doc.element_query("#link-sign-in").unwrap();

    btn_home.on("click", |_| {
        set_route("home");   
    });
    btn_signup.on("click", |_| {
        set_route("sign_up");
    });
    btn_signin.on("click", |_| {
        set_route("sign_in"); 
    });
}

