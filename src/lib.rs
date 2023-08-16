pub mod mod_a;

use crate::mod_a::object_mod::func_b;
use crate::mod_a::object_mod::submod_a::func_a;

fn main() {
    println!("Hello, world!");
    func_a();
    func_b();
}

//WILL NOT COMPILE
//use crate::mod_a::object_mod::ObjectA;
