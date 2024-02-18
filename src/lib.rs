mod module;
mod serde_utils;
mod utils;

pub use module::entry::{Entry, Kind as EntryKind};
pub use module::source::{Source, SourceError};
pub use module::{Module, ModuleError};
