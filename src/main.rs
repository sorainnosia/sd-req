#[macro_use]
extern crate lazy_static;
mod sd_req;
mod http;

use sd_req::sd_reqo;

fn main() {
    let obj = sd_req::sd_reqo::new();
    obj.sdcall();
}