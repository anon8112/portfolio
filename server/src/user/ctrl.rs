use redis::{RedisResult};
use hyper::server::Request;
use util::{Db, json_to_vec, json_to_struct};
use portfolio_shared::SignUpInput;
use std::io::Read;

pub fn sign_up(db: &Db, req: &mut Request) -> String {
    let mut data = String::new();
    req.read_to_string(&mut data).unwrap();
    let user: SignUpInput = json_to_struct(&data).unwrap();
    let valid = user.valid();
    if valid.is_err() {
        return valid.err().unwrap();
    }
    let res = add(db, &data);
    match res {
        Ok(m) => m,
        Err(err) => format!("{}", err)
    }
}

pub fn sign_in(db: &Db, req: &mut Request) -> String {
    "hello sign in".into()
}

pub fn add(db: &Db, data: &str) -> RedisResult<String> {
    let key = db.key("user", db.count("user"));
    try!(db.run("HMSET", &key, json_to_vec(data)));
    try!(db.increment("user"));
    Ok("User Added Successfully".into())
}
