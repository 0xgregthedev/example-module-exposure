use crate::mod_a::object_mod::ObjectA;

const A: ObjectA = ObjectA { field_a: 0 };


pub fn func_a() {
    println!("func_a() called");
    println!("A.field_a = {}", A.field_a);
}
