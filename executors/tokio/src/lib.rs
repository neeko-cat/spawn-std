use std::pin::Pin;
use std::task::{Context, Poll};

use spawn_std_core::{Executor, Handle};
use tokio::task::{JoinError, JoinHandle};

pub struct TokioExecutor;

impl Executor for TokioExecutor {
    type Handle<T: Send + 'static> = TokioHandle<T>;

    fn spawn<T, F>(future: F) -> Self::Handle<T>
    where
        T: Send + 'static,
        F: Future<Output = T> + Send + 'static,
    {
        TokioHandle(tokio::task::spawn(future))
    }
}

pub struct TokioHandle<T>(JoinHandle<T>);

impl<T> TokioHandle<T> {
    fn inner_pin_mut(self: Pin<&mut Self>) -> Pin<&mut JoinHandle<T>> {
        unsafe { self.map_unchecked_mut(|x| &mut x.0) }
    }
}

impl<T> Unpin for TokioHandle<T> {}

impl<T> Future for TokioHandle<T> {
    type Output = Result<T, JoinError>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner_pin_mut().poll(cx)
    }
}

impl<T> Handle<T> for TokioHandle<T>
where
    T: Send + 'static,
{
    type Error = tokio::task::JoinError;

    #[inline]
    fn abort(self) {
        self.0.abort();
    }
}
