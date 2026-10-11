//! Thin wrappers around libc: hostname, syslog and process priority.

use std::ffi::CString;
use std::io;

/// The machine's node name (`platform.node()`).
pub fn hostname() -> String {
    let mut buf = [0u8; 256];
    // SAFETY: buf is valid for buf.len() bytes.
    let rc = unsafe { libc::gethostname(buf.as_mut_ptr().cast(), buf.len()) };
    if rc != 0 {
        return String::new();
    }
    let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    String::from_utf8_lossy(&buf[..end]).into_owned()
}

/// Log an error to syslog under the `runner` identity.
pub fn syslog_error(msg: &str) {
    let msg = CString::new(format!("[ERROR] {}", msg.replace('\0', ""))).unwrap();
    // SAFETY: ident is a 'static C string; format and message are valid C strings.
    unsafe {
        libc::openlog(c"runner".as_ptr(), 0, libc::LOG_USER);
        libc::syslog(libc::LOG_ERR, c"%s".as_ptr(), msg.as_ptr());
    }
}

/// Set the niceness of the current process (inherited by children).
pub fn set_priority(niceness: i32) -> io::Result<()> {
    // SAFETY: plain syscall without pointers.
    if unsafe { libc::setpriority(libc::PRIO_PROCESS, 0, niceness) } != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}
