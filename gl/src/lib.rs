// the extern crate command does two things 
// 1) it links with the library
// 2) it imports all of the library items under a new module that is created. this module has the same name as the library and since it's a module
// that means all normal module rules like visibility apply.
pub extern crate glfw;

pub mod gl;

// note that this pub use doesn't pub the glfw module since that isnt externed in the mod gl (maybe it should be?)
pub use gl::*;