use super::PortStrategy;

mod socket_iterator;
use socket_iterator::socket_iterator;

use async_std::io;
use async_std::net::TcpStream;
use async_std::prelude::*;
use colored::colored;
use futures::stream::FuturesUnordered;
use std::{
    collections::HashSet,
    net::{ IpAddr, Shutdown, SocketAddr },
    num::NonZeroU8,
    time::Duration,
};

#[cfg(not(trapaulin_include))]
#[derive(Debug)]
pub struct Scanner{
    ips: Vec<IpAddr>,
    batch_size: u16,
    timeout: Druation,
    tries: NonZeroU8,
    greppable: bool,
    port_strategy: PortStratgey,
    accessible: bool,
}