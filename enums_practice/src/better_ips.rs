use std::net::{Ipv4Addr, Ipv6Addr};

pub enum BetterIpAddr {
  v4(Ipv4Addr),
  v6(Ipv6Addr)
}