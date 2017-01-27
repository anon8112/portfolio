use webplatform;
use super::get_value;
use http;
use rustc_serialize::json;
use portfolio_shared::SignUpInput;

pub fn handle() {
    let doc = webplatform::init();
    let btn = doc.element_query("#btn-signup").unwrap();
    btn.on("click", move |_| {
        let email = get_value("#email");
        let username = get_value("#username");
        let password = get_value("#password");
        let rpassword = get_value("#rpassword");

        if password != rpassword {
            webplatform::alert("passwords don't match");
            return;
        }
        
        
        
        let s = SignUpInput{email:email, username: username, password: password};
        let js = json::encode(&s).unwrap();
        let res = http::post("/user/sign_up", &js);
        println!("{}", res);
    })
}
