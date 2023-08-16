mod submod_a;
mod submod_b;

struct ObjectA {
    pub field_a: u32,
}

pub fn func_a() {
    println!("func_a() called");
    println!("A.field_a = {}", A.field_a);
}
