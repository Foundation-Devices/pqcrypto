#![no_std]
#![feature(c_size_t)]

use core::slice;

/// Get random bytes; exposed for PQClean implementations.
///
/// # Safety
/// Assumes inputs are valid and may panic over FFI boundary if rng failed.
///
/// # Example
/// ```rust
/// use pqcrypto_internals::*;
/// let mut buf = [0u8;10];
/// unsafe {
///   PQCRYPTO_RUST_randombytes(buf.as_mut_ptr(), buf.len());
/// }
/// ```
#[no_mangle]
pub unsafe extern "C" fn PQCRYPTO_RUST_randombytes(
    buf: *mut u8,
    len: core::ffi::c_size_t,
) -> core::ffi::c_int {
    let buf = slice::from_raw_parts_mut(buf, len);
    getrandom::getrandom(buf).expect("RNG Failed");
    0
}
