#[cfg_attr(unix, path = "linux/mod.rs")]
#[cfg_attr(windows, path = "windows/mod.rs")]
mod _platform;

pub use _platform::*;

#[cfg(all(not(windows), not(unix),))]
compile_error!("The platform you're compiling for is not supported");
