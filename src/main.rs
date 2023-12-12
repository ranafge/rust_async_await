#![allow(unused, dead_code)]

use rust_futures_for_string::MyFuture;

use std::time::Duration;

use futures::{join, executor::block_on, Future, future};
// async keyward can be used to define asynchronous tasks or coroutines

#[tokio::main]
async fn main() {
    let result_future = MyFuture.await;
    println!("{}", result_future);

}


async fn add(x:i32) -> i32{
    println!("x + 1");
    x + 1
}

async fn some_wait()->i32 {
    tokio::time::sleep(Duration::from_secs(4));
    println!("all done");
    for i in 0..10{
        println!("{}", i);
        tokio::task::yield_now().await;
    };
    0
}

async fn sub(x:i32) -> i32{
    println!("x - 2");
    tokio::task::yield_now().await;
    x -2
}

// simile commit