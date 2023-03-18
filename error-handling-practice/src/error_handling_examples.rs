use std::net::IpAddr;

pub fn non_error_example() {
  /*
    The home variable is a hardcoded IP address that will never throw an
    error. However, since `.parse()` will return a `Result` type, we still
    need to add `.expect` to handle a potential `Err`.
   */
  let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP Address should be valid.");

}