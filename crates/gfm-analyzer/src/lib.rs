pub mod backend;
mod handle;
pub mod span;
pub use gfm_syntax as gfm;
pub mod types;

pub type OkSome<T> = Result<Option<T>, anyhow::Error>;
