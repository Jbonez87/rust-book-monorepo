use std::net::IpAddr;

pub fn non_error_example() {
  let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP Address should be valid.");

}