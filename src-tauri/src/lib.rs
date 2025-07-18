mod utility;
mod library;

use std::{
    thread,
    io::{ErrorKind, Read},
    process::Child,
    sync::Mutex,
};

use tauri::Window;
use once_cell::sync::Lazy;

use library::{
    _error::io_error_maker, 
    _path::file_exists,
    _tauri::emit_payload,
    _process::kill_process,
};

use utility::{
    _command::ffmpeg_command_maker,
    _constant::FFmpegEvent,
};

static GLOBAL_PID: Lazy<Mutex<Option<u32>>> = Lazy::new(|| Mutex::new(None));


/// 處理執行command的log訊息
/// # 參數
/// - `child` - 執行的Child
/// - `closure` - 取得每行log訊息後，要執行的功能
fn child_log<F: Fn(&str)>(child: &mut Child, closure: F) {
    if let Some(mut stderr) = child.stderr.take() {

        // 在大多數情況下（包含 ffmpeg、sh、bash 等 CLI 工具）：
        // stdout（標準輸出）：預設用來輸出「正常結果資料」，例如程式的輸出內容、計算結果、檔案內容等。
        // stderr（標準錯誤）：預設用來輸出「錯誤訊息」或「進度、警告、log」等資訊。
        // 之後 thread 內直接用 child 處理 stderr、wait
        let mut buffer = [0u8; 4096];
        let mut partial = String::new();

        loop {
            match stderr.read(&mut buffer) {
                Err(_) => break,
                Ok(0) => break,
                Ok(size) => {
                    let chunk = String::from_utf8_lossy(&buffer[..size]);
                    partial.push_str(&chunk);
                    partial_line_action(&mut partial, &closure);
                }
            }
        }
    }
}

/// 處理log部分每一行的輸出 => \r or \n
/// # 參數
/// - `partial` - 部分的標準輸出
/// - `closure` - 取得每行log訊息後，要執行的功能
fn partial_line_action<F: Fn(&str)>(partial: &mut String, closure: F) {

    while let Some(index) = partial.find(|char| char == '\r' || char == '\n') {
        let line = partial[..index].to_string();
        closure(&line);
        *partial = partial[index + 1..].to_string();
    }
}

/// 將GLOBAL_PID上鎖 (防止被更改)
fn global_pid_lock() -> std::sync::MutexGuard<'static, Option<u32>> {
    GLOBAL_PID.lock().unwrap()
}

/// 開始轉換任務的動作
/// # 參數
/// - `window` - Tauri 的 Window 物件，用來發送事件
/// - `command` - ffmpeg 命令字串，通常為 "/opt/homebrew/bin/ffmpeg"
/// - `path` - 檔案路徑
/// - `start_time` - 開始時間，格式為 "HH:MM:SS"
/// - `end_time` - 結束時間，格式為 "HH:MM:SS"
/// - `format` - 輸出檔案格式，例如 "mp4", "mkv"
/// - `encode` - 編碼格式，例如 "h264", "h265", "copy"
/// - `scale` - 影片尺寸，例如 "320:240", "1290:1080"
fn start_convert_action(window: Window, command: &str, path: &str, start_time: &str, end_time: &str, format: &str, encode: &str, scale: &str) {

    let window_clone = window.clone();

    if !file_exists(path) {
        let error = io_error_maker(ErrorKind::NotFound, "檔案不存在");
        emit_payload(&window_clone, "error", format!("{:?}", error)); return;
    }

    let (mut cmd, cmd_str) = match ffmpeg_command_maker(command, path, start_time, end_time, format, encode, scale) {
        Ok(result) => result,
        Err(error) => { emit_payload(&window_clone, FFmpegEvent::Error.as_str(), format!("{:?}", error)); return; },
    };

    // 如果沒有 move，thread 內部閉包就不能直接用外部的變數（除非它是 'static 或 Copy） / 把所有權移進來了。
    // 也就是說，move就是讓外面的變數不會提早不見
    // spawn() 就是產生一個新的 thread，這個 thread 會在背景執行 ffmpeg 命令 / spawn() => run a new thread
    thread::spawn(move || {

        let mut child = match cmd.spawn() {
            Ok(child) => child,
            Err(error) => { emit_payload(&window_clone, FFmpegEvent::Error.as_str(), format!("{:?}", error)); return; },
        };

        // 記錄子進程 pid（讓 stop_convert 能 kill）
        {
            let mut global = global_pid_lock();
            *global = Some(child.id());
        }

        child_log(&mut child, |line| {
            if !line.is_empty() { emit_payload(&window_clone, FFmpegEvent::Progress.as_str(), line); }
        });

        // 等待子進程結束並獲取輸出
        let output = match child.wait_with_output() {
            Ok(output) => output,
            Err(error) => {
                emit_payload(&window_clone, FFmpegEvent::Error.as_str(), format!("{:?}",error));
                let mut global = global_pid_lock();
                *global = None; return;
            }
        };

        emit_payload(&window_clone, FFmpegEvent::Finish.as_str(), format!("{:?}", output));
        emit_payload(&window_clone, FFmpegEvent::Finish.as_str(), &cmd_str);
    });
}

#[tauri::command]
fn start_convert(window: Window, command: &str, path: &str, start_time: &str, end_time: &str, format: &str, encode: &str, scale: &str) {
    start_convert_action(window, command, path, start_time, end_time, format, encode, scale); 
}

#[tauri::command]
fn stop_convert() {
    let mut pid_opt = GLOBAL_PID.lock().unwrap();

    // pid_opt.take() 的功能是「取出 Option 內的值，並把自己設為 None」。
    if let Some(pid) = pid_opt.take() { kill_process(pid); }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_convert, stop_convert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
