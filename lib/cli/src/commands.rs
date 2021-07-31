//! The commands available in the Wasmer binary.
mod cache;
#[cfg(feature = "compiler")]
mod compile;
mod config;
#[cfg(all(feature = "staticlib", feature = "compiler"))]
mod create_exe;
mod inspect;
mod run;
mod self_update;
mod validate;
#[cfg(feature = "wast")]
mod wast;
#[cfg(unix)]
mod binfmt;

#[cfg(feature = "compiler")]
pub use compile::*;
#[cfg(all(feature = "staticlib", feature = "compiler"))]
pub use create_exe::*;
#[cfg(feature = "wast")]
pub use wast::*;
#[cfg(unix)]
pub use binfmt::*;
pub use {cache::*, config::*, inspect::*, run::*, self_update::*, validate::*};
