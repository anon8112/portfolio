use redis::{cmd, Connection, RedisResult};
use hyper::server::Request;
use std::sync::{Mutex, MutexGuard};
use rustc_serialize::json;
use rustc_serialize::Decodable;
use serde_json;

pub struct Db {
    pub con: Mutex<Connection>,
    pub name: String
}

impl Db {
    pub fn run(&self, op: &str , key: &str, data: Vec<String>) -> RedisResult<String> {
        cmd(op).arg(key).arg(data).query(&*self.con.lock().unwrap())
    }
    
    pub fn count(&self, table: &str) -> usize {
        let key = format!("{}:{}_count", &self.name, table);
        println!("{}", key);
        match cmd("GET").arg(&key).query(&*self.con.lock().unwrap()) {
            Ok(c) => c,
            Err(_) => {
                let r: RedisResult<String> = 
                        cmd("SET").arg(&key).arg(0).query(&*self.con.lock().unwrap());
                match r {
                    Ok(_) => 0_usize,
                    Err(err) => panic!("{}", err)
                }
            }
        }
    }
    pub fn increment(&self, table: &str) -> RedisResult<()> {
        let key = format!("{}:{}_count", &self.name, table);
        try!(cmd("INCR").arg(key).query(&*self.con.lock().unwrap()));
        Ok(())
    }
    pub fn key(&self, table: &str, id: usize) -> String {
        format!("{}:{}:{}", &self.name, table, id)
    }
}


pub fn json_to_struct<T: Decodable>(data: &str) -> Result<T, String> {
    match json::decode::<T>(data) {
        Ok(obj) => Ok(obj),
        Err(err) => Err(format!("{}",err))
    }
}


pub fn json_to_vec(data: &str) -> Vec<String>  {
    let json: serde_json::Value = serde_json::from_str(data).unwrap();
    let obj = json.as_object().unwrap();
    let mut arr: Vec<String> = Vec::new();
    for(key, val) in obj.iter() {
        arr.push(key.as_str().into());
        arr.push(val.as_str().unwrap().into());
    }
    arr

}
