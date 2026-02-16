#[allow(dead_code)]
#[cfg(any(
    target_os = "windows",
    target_os = "redox",
    target_os = "espidf",
    target_os = "horizon"
))]
pub const LISTEN_BACKLOG_SIZE: i32 = 128;

/// This is a special case for some target(s) supported by `mio`.  This value
/// is needed because `libc::SOMAXCON` (used as a fallback for unknown targets)
/// is not implemented for them. Feel free to update this if the `libc` crate
/// changes.
#[allow(dead_code)]
#[cfg(target_os = "hermit")]
pub const LISTEN_BACKLOG_SIZE: i32 = 1024;

#[allow(dead_code)]
#[cfg(any(
    // Silently capped to `/proc/sys/net/core/somaxconn`.
    target_os = "linux",
    // Silently capped to `kern.ipc.soacceptqueue`.
    target_os = "freebsd",
    // Silently capped to `kern.somaxconn sysctl`.
    target_os = "openbsd",
    // Silently capped to the default 128.
    target_vendor = "apple",
))]
pub const LISTEN_BACKLOG_SIZE: i32 = -1;

#[allow(dead_code)]
#[cfg(not(any(
    target_os = "windows",
    target_os = "redox",
    target_os = "espidf",
    target_os = "horizon",
    target_os = "linux",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "wasi",
    target_os = "hermit",
    target_vendor = "apple",
)))]
pub const LISTEN_BACKLOG_SIZE: i32 = libc::SOMAXCONN;
