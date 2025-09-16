use std::convert::Infallible;
use std::pin::Pin;
use std::task::{Context, Poll};

use async_std::task::JoinHandle;
use spawn_std_core::{Executor, Handle};

pub struct AsyncStdExecutor;

impl Executor for AsyncStdExecutor {
    type Handle<T: Send + 'static> = AsyncStdHandle<T>;

    fn spawn<T, F>(future: F) -> Self::Handle<T>
    where
        T: Send + 'static,
        F: Future<Output = T> + Send + 'static,
    {
        AsyncStdHandle(async_std::task::spawn(future))
    }
}

pub struct AsyncStdHandle<T>(JoinHandle<T>);

impl<T> AsyncStdHandle<T> {
    #[inline]
    fn inner_pin_mut(self: Pin<&mut Self>) -> Pin<&mut JoinHandle<T>> {
        unsafe { self.map_unchecked_mut(|x| &mut x.0) }
    }
}

impl<T> Future for AsyncStdHandle<T> {
    type Output = Result<T, Infallible>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner_pin_mut().poll(cx).map(Ok)
    }
}

impl<T> Handle<T> for AsyncStdHandle<T> {
    type Error = Infallible;

    #[inline(always)]
    fn abort(self) {
        drop(self.0);
    }
}
