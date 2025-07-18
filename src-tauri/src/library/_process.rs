use std::{process::Command};

/// 結合成完整的指令 (含各系統的啟動shell / cmd)
/// # 參數
/// - `command`: 主要指令
/// # 回傳
/// - `Command`: 回傳完整的指令
pub fn combine_command(command: &str) -> Command {

    #[cfg(unix)] { combine_unix_command(command) }
    #[cfg(windows)] { combine_windows_command(command) }
}

/// 終止正在運行的 ffmpeg 子進程
/// # 參數
/// - `pid` - 子進程的 PID
pub fn kill_process(pid: u32) {
    #[cfg(unix)] { kill_unix_process(pid as i32) }
    #[cfg(windows)] { kill_windows_process(pid) }
}

/// 終止正在運行的 ffmpeg 子進程（Unix）
/// # 參數
/// - `pid` - 子進程的 PID
/// # 注意
/// - 這個函數使用 `nix` crate 的 `kill` 函數來終止進程。
/// - 確保在使用此函數前，已經導入了 `nix` crate。
#[cfg(unix)]
fn kill_unix_process(pid: i32) {

    use nix::sys::signal::{kill, Signal};
    use nix::unistd::Pid;

    let _ = kill(Pid::from_raw(pid as i32), Signal::SIGKILL);
}

/// 終止正在運行的 ffmpeg 子進程（Windows）
/// # 參數
/// - `pid` - 子進程的 PID
/// # 注意
/// - 這個函數使用 `OpenProcess` 和 `TerminateProcess` API
///   來終止進程。
/// - 確保在使用此函數前，已經導入了 `windows-sys` crate。
#[cfg(windows)]
fn kill_windows_process(pid: u32) {

    use std::ptr::null_mut;
    use windows_sys::Win32::System::Threading::PROCESS_TERMINATE;
    use windows_sys::Win32::System::Threading::{OpenProcess, TerminateProcess};

    unsafe {
        let handle = OpenProcess(PROCESS_TERMINATE, 0, pid);
        if handle != null_mut() {
            let _ = TerminateProcess(handle, 1);
        }
    }
}

/// 結合成完整的指令（Unix）
/// # 參數
/// - `command`: 主要指令
/// # 回傳
/// - `Command`: 回傳完整的指令
#[cfg(unix)]
fn combine_unix_command(command: &str) -> Command {

    let mut cmd = Command::new("sh");
    cmd.arg("-c");
    cmd.arg(command);
    cmd
}

/// 結合成完整的指令（Windows）
/// # 參數
/// - `command`: 主要指令
/// # 回傳
/// - `Command`: 回傳完整的指令
#[cfg(windows)]
fn combine_windows_command(command: &str) -> Command {

    use std::os::windows::process::CommandExt;

    let mut cmd = Command::new("cmd");
    cmd.arg("/C");
    cmd.arg(command);
    cmd.creation_flags(0x08000000);
    cmd
}