//! Runtime-agnostic async task spawner.

pub use spawn_std_core::*;


#[cfg(feature = "tokio")]
pub mod executors {
    //! Built-in executors.

    #[cfg(feature = "tokio")]
    pub use spawn_std_tokio as tokio;
}
