use std::arch::asm;

#[cfg(target_os = "linux")]
#[inline(never)]
fn syscall(message: String) {
    let message_ptr = message.as_ptr();
    let message_len = message.len();

    unsafe {
        asm!(
            "mov rax, 1", // write
            "mov rdi, 1", // stdout
            "syscall", // invoke syscall
            in("rsi") message_ptr, // pointer to message
            in("rdx") message_len, // length of message
            out("rax") _, 
            out("rdi") _,
            lateout("rsi") _,
            lateout("rdx") _
        );
    }
}

fn main() {
    let message = String::from("Hello, World!");
    syscall(message);
}
