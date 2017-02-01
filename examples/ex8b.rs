#![feature(conservative_impl_trait)]
extern crate futures;
extern crate futures_cpupool;
#[macro_use]
extern crate text_io;
use std::{thread, time};
use futures::future::*;
use futures::future::poll_fn;
use futures::{Async, Poll};

macro_rules! sleep {
    ($v:expr) => (thread::sleep(time::Duration::from_millis($v));)
}

macro_rules! to_async {
    ($e:expr) => (Ok(Async::Ready($e)))
}

macro_rules! print_time_taken {
    ($t:expr) => (println!("Time taken: {}.{} sec",
             $t.as_secs(),
             $t.subsec_nanos() / 1000_000);)
}

fn read_line() -> Poll<String, std::io::Error> {
    sleep!(900);
    to_async!("Value: 42".into())
}

#[allow(unused_imports)]
fn parse_line(val: String) -> Poll<i32, std::io::Error> {
    sleep!(400);
    to_async!(read!("Value: {}", val.bytes()))
}

fn main() {
    let start_time = time::Instant::now();
    let mut future = poll_fn(read_line).map(|v| parse_line(v));

    print_time_taken!(start_time.elapsed());
    println!("Val: {:?}", future.poll());
    print_time_taken!(start_time.elapsed());
}