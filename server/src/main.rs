extern crate hyper;
extern crate redis;
extern crate serde_json;
extern crate rustc_serialize;
extern crate portfolio_shared;

pub mod util;
pub mod api;
pub mod user;
mod server;

fn main() {
   server::start(); 
}
