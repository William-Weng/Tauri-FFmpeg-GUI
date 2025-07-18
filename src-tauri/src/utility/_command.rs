use std::{
    str,
    io::Error,
    collections::HashMap,
    process::{Command, Stdio}
};

use crate::library:: {
    _process::{combine_command},
    _path::{full_path_maker},
};

/// 產生一個完整的 ffmpeg 指令並返回 Command 對象 (使用管道輸出)
/// # 參數
/// - `command` - ffmpeg 命令字串，通常為 "/opt/homebrew/bin/ffmpeg"
/// - `path` - 檔案路徑
/// - `start_time` - 開始時間，格式為 "HH:MM:SS"
/// - `end_time` - 結束時間，格式為 "HH:MM:SS"
/// - `format` - 輸出檔案格式，例如 "mp4", "mkv"
/// - `encode` - 編碼格式，例如 "h264", "h265", "copy"
/// - `scale` - 影片尺寸，例如 "320:240", "1290:1080"
/// # 回傳
/// - `Ok((Command, String))` - 成功時回傳 Command 對象和完整的 ffmpeg 指令
/// - `Err(Error)` - 失敗時回傳錯誤
pub fn ffmpeg_command_maker(command: &str, path: &str, start_time: &str, end_time: &str, format: &str, encode: &str, scale: &str) -> Result<(Command, String), Error> {

    let ffmpeg_cmd = ffmpeg_code_maker(command, path, start_time, end_time, format, encode, scale)?;
    let mut command = combine_command(ffmpeg_cmd.as_str());

    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    Ok((command, ffmpeg_cmd))
}

/// 產生一個完整的 ffmpeg 指令
/// # 參數
/// - `command` - ffmpeg 命令字串，通常為 "/opt/homebrew/bin/ffmpeg"
/// - `path` - 檔案路徑
/// - `start_time` - 開始時間，格式為 "HH:MM:SS"
/// - `end_time` - 結束時間，格式為 "HH:MM:SS"
/// - `format` - 輸出檔案格式，例如 "mp4", "mkv"
/// - `encode` - 編碼格式，例如 "h264", "h265", "copy"
/// - `scale` - 影片尺寸，例如 "320:240", "1290:1080"
/// # 回傳
/// - `Ok(String)` - 成功時回傳完整的 ffmpeg 指令
/// - `Err(Box<Error>)` - 失敗時回傳錯誤
fn ffmpeg_code_maker(command: &str, path: &str, start_time: &str, end_time: &str, format: &str, encode: &str, scale: &str) -> Result<String, Error> {
    let codec = encode_codec(encode).unwrap_or("-c copy");
    let output_path = full_path_maker(path, format)?;

    let video_scale = if scale.is_empty() {
        String::new()
    } else {
        format!("-vf scale=\"{}\"", scale)
    };

    let ffmpeg_cmd = format!("{} -ss {} -to {} -i \"{}\" {} {} \"{}\"", command, start_time, end_time, path, codec, video_scale, output_path.display());
    Ok(ffmpeg_cmd)
}

/// 根據編碼格式返回對應的 ffmpeg 編碼參數
/// # 參數
/// - `encode` - 編碼格式，例如 "h264", "h265", "copy"
/// # 回傳
/// - `Some(&str)` - 如果編碼格式存在，則返回對應的 ffmpeg 編碼參數
/// - `None` - 如果編碼格式不存在，則返回 None
fn encode_codec(encode: &str) -> Option<&'static str> {
    let encodes: HashMap<&str, &str> = [
        ("copy", "-c copy"),
        ("h264", "-c:v libx264 -pix_fmt yuv420p -c:a aac"),
        ("h265", "-c:v libx265 -pix_fmt yuv420p -tag:v hvc1 -c:a aac"),
    ].into_iter().collect();

    encodes.get(encode).copied()
}