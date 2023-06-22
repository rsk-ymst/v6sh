// fn syscall<F, T>(f: F) -> Result<T, nix::Error>
// where
//     F: Fn() -> Result<T, nix::Error>,
// {
//     loop {
//         match f() {
//             Err(nix::Error::EINTR) => (),
//             result => return result
//         }
//     }
// }

// enum WorkerMsg {
//     Signal(i32),
//     Cmd(String),
// }

// enum ShellMsg {
//     Continue(i32),
//     Quit(i32),
// }
