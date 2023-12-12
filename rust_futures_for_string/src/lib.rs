use std::{future::Future, task::Poll};

pub struct MyFuture;

impl Future for MyFuture {
    type Output = &'static str;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        Poll::Ready("Hello, Future!")
    }
}