/*
Cliply by Alyx Shang.
Licensed under the FSL v1.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the main module.
pub use modules::cliply::*;

/// Re-exporting the module
/// to handle any errors
/// that might occur.
pub use modules::errors::*;