use crate::mod_a::object_mod::ObjectA;

const B: ObjectA = ObjectA { field_a: 0 };

pub fn func_b() {
    println!("func_b() called");
    println!("B.field_a = {}", B.field_a);
}
