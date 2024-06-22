#[cfg(not(feature = "libc"))]
#[macro_use]
mod wasm;

#[cfg(feature = "libc")]
#[macro_use]
mod libc;

mod minimp4_c;

pub use self::minimp4_c::*;

pub mod environment {
    #[cfg(feature = "libc")]
    pub use super::libc::*;
    #[cfg(not(feature = "libc"))]
    pub use super::wasm::*;
}
