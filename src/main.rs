#![allow(unused, dead_code)]
use tokio::time::Duration;
use rust_futures_for_string::MyFuture;
use async_await;


use futures::{join, executor::block_on, Future, future};
// async keyward can be used to define asynchronous tasks or coroutines

#[tokio::main]
async fn main() {
    let result_future = MyFuture.await;
    println!("{}", result_future);
    let resut_async_await = async_await::bar().await;
    println!("{}", resut_async_await);
    let result_move_to_async_block = move_to_async_block().await;
    println!("{:?}", result_move_to_async_block);


}

fn move_to_async_block () -> impl Future<Output=String> {
    let name = "Rana".to_string();
    async move {
        format!("Hello {}", name)
    }
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