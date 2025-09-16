use std::convert::Infallible;
use std::pin::Pin;
use std::task::{Context, Poll};

use smol::Task;
use spawn_std_core::{Executor, Handle};

pub struct SmolExecutor;

impl Executor for SmolExecutor {
    type Handle<T: Send + 'static> = SmolHandle<T>;

    fn spawn<T, F>(future: F) -> Self::Handle<T>
    where
        T: Send + 'static,
        F: Future<Output = T> + Send + 'static,
    {
        SmolHandle(smol::spawn(future))
    }
}

pub struct SmolHandle<T>(Task<T>);

impl<T> SmolHandle<T> {
    #[inline]
    fn inner_pin_mut(self: Pin<&mut Self>) -> Pin<&mut Task<T>> {
        unsafe { self.map_unchecked_mut(|x| &mut x.0) }
    }
}

impl<T> Unpin for SmolHandle<T> {}

impl<T> Future for SmolHandle<T> {
    type Output = Result<T, Infallible>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner_pin_mut().poll(cx).map(Ok)
    }
}

impl<T> Handle<T> for SmolHandle<T> {
    type Error = Infallible;

    #[inline(always)]
    fn abort(self) {
        drop(self.0);
    }
}
