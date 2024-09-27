/* Zen Vulnerability library, contains Zen vulnerability code written in Rust and exported
 * Using FFI. Currently we support the following algorithms :
 * - Shell Injection
 * - Route Building
 * - SQL Injection (WIP)
 */
mod helpers;
mod route_builder;
mod shell_injection;
mod sql_injection;

// FFI Bindings :
pub mod ffi_bindings;
