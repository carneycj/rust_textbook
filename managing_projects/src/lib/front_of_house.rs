// This file must be named the same as the module.

// The semicolon indicates that the module must be loaded in from a separate
// file.  The separate file must be named the same as the module being
// loaded.
pub mod hosting;

pub mod serving {
    fn take_order() {}

    pub fn serve_order() {}

    fn take_payment() {}
}
