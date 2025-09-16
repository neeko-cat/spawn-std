pub trait Executor {
    type Handle<T: Send + 'static>: Handle<T>;

    fn spawn<T, F>(future: F) -> Self::Handle<T>
    where
        T: Send + 'static,
        F: Future<Output = T> + Send + 'static;
}

pub trait Handle<T>: Future<Output = Result<T, Self::Error>> {
    type Error: std::fmt::Debug;

    fn abort(self);
}
