//export all public items of submod_a
pub mod submod_a;

//selectively re-export public items of submod_b
mod submod_b;
pub use submod_b::func_b;

struct ObjectA {
    pub field_a: u32,
}
