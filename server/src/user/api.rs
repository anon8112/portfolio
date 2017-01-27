use hyper::server::{Request, Response};
use super::ctrl;
use util::Db;
use hyper::header::AccessControlAllowOrigin;

pub fn handle(db: &Db, req: &mut Request, mut res: Response) {
    res.headers_mut().set(AccessControlAllowOrigin::Any);
    match req.uri.to_string().as_str() {
        "/user/sign_up" => {
            res.send(
                ctrl::sign_up(db, req).as_bytes()
            ).unwrap()
        }
        "/user/sign_in" => {
            res.send(
                ctrl::sign_in(db, req).as_bytes()
            ).unwrap()
        }
        _ => println!("User route not found")
    }
}
