pub mod submod_a;

mod submod_b;
pub use submod_b::func_b;


struct ObjectA {
    pub field_a: u32,
}

