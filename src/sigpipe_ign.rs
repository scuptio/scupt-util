
#[cfg(unix)]
pub fn sigpipe_ign() {
    unsafe {
        ::libc::signal(::libc::SIGPIPE, ::libc::SIG_IGN);
    }
}

#[cfg(not(unix))]
pub fn sigpipe_ign() {

}