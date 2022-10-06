/// Standard file-system routines, but logged with tracing
pub mod logged;

pub use self::platforms::*;
pub use ::platforms::platform;
pub use ::platforms::target::*;
pub use ::platforms::Platform;
pub use formatting::*;
pub use util::*;

mod formatting;
mod platforms;
mod util;
