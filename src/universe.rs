#[repr(u8)]
#[derive(Debug)]
pub enum EUniverse {
  Invalid = 0,
  Public = 1,
  Beta = 2,
  Internal = 3,
  Dev = 4,
  // RC = 5, // no longer exists
  Max
}