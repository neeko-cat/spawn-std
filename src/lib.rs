//! Runtime-agnostic async task spawner.

pub use spawn_std_core::*;

#[cfg(any(
    feature = "tokio",
    feature = "smol_1",
    feature = "smol_2",
    feature = "async-std"
))]
pub mod executors {
    //! Built-in executors.

    #[cfg(feature = "async-std")]
    pub use spawn_std_async_std as async_std;

    #[cfg(feature = "smol_1")]
    pub use spawn_std_smol_1 as smol_1;

    #[cfg(feature = "smol_2")]
    pub use spawn_std_smol_2 as smol_2;

    #[cfg(feature = "tokio")]
    pub use spawn_std_tokio as tokio;
}
