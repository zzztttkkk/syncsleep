#![deny(clippy::all)]

use std::u64;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sleep(ms: u32) -> Result<(), napi::Error> {
  if ms > 1000 {
    return Err(napi::Error::from_reason(
      "you can not sleep more than 1000ms",
    ));
  }
  std::thread::sleep(std::time::Duration::from_millis(ms as u64));
  return Ok(());
}
