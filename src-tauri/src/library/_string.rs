use chrono;

/// 產生一個帶有時間戳的檔案名稱
/// # 參數
/// - `prefix`: 檔案名稱前綴，例如 "output"
/// - `format`: 檔案格式，例如 "mp4", "mkv"
/// # 回傳
/// - `String` - 回傳格式為 "output_YYYYMMDD_HHMMSS.format"
pub fn timestamp_filename(prefix: &str, format: &str) -> String {

    let now = chrono::Local::now();
    let datetime_str = now.format("%Y%m%d_%H%M%S").to_string();
    
    format!("{}_{}.{}", prefix, datetime_str, format)
}