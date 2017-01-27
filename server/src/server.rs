use hyper::server::Server;
use std::sync::Mutex;
use util::Db;
use redis;
use api;

pub fn start() {
    let server = match Server::http("0.0.0.0:3000") {
        Ok(server) => server,
        Err(err) => panic!("{}", err)
    };
    
    let client = match redis::Client::open("redis://127.0.0.1") {
        Ok(client) => client,
        Err(err) => panic!("{}", err)
    };

    let con = match client.get_connection() {
        Ok(con) => Mutex::new(con),
        Err(err) => panic!("{}", err)
    };

    server.handle(api::ApiHandler{db: Db{con: con, name: "portfolio".into()}, count: 0}).unwrap();
}
