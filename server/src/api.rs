use hyper::server::{Handler, Request, Response};
use redis::Connection;
use std::sync::Mutex;
use util::Db;
use user;

pub struct ApiHandler {
    pub db: Db,
    pub count: i32
}

impl Handler for ApiHandler {
    fn handle(&self, mut req: Request, mut res: Response) {
        let url = req.uri.to_string();
        if url.contains("user") {
            user::api::handle(&self.db, &mut req, res);
        }
    }
}

