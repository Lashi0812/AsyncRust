use std::io;

#[cfg(target_family = "unix")]
#[link(name = "c")]
extern "C" {
    fn write(fd: i32, buf: *const u8, count: usize) -> i32;
}

#[cfg(target_family = "unix")]
fn syscall(message: String) -> io::Result<()> {
    let message_ptr = message.as_ptr();
    let message_len = message.len();

    let res = unsafe { write(1, message_ptr, message_len) };
    if res == -1 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

fn main() {
    let message = "Hello world from syscall!\n";
    let message = String::from(message);
    syscall(message).unwrap();
}
