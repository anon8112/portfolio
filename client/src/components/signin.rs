use webplatform;
use super::get_value;
use http;

pub fn handle() {
    let doc = webplatform::init();
    let btn = doc.element_query("#btn-sign-in").unwrap();
    btn.on("click", |_| {
        let username = get_value("#username");
        let password = get_value("#password");
        let res = http::post("/user/sign_in", &username);
        println!("{}", res);
    });
}
