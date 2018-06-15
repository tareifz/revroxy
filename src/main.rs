#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod util;


use util::config_vec;
use util::Host;

fn main() {

    for host in config_vec() {
        println!("{}, {}, {}", host.host, host.ip, host.port);
    }

}
