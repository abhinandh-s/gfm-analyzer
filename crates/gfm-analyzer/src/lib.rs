pub mod backend;
pub mod span;
mod handle;
pub use gfm_syntax as gfm;
pub mod types;

pub type OkSome<T> = Result<Option<T>, anyhow::Error>;
