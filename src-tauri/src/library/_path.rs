use std::path::{Path, PathBuf};
use std::io::{Error, ErrorKind};

use crate::library::_string::timestamp_filename;
use crate::library::_error::io_error_maker;

/// 該檔案路徑是否存在
/// # 參數
/// - `file_path`: 檔案路徑
/// # 回傳
/// - `true` 如果檔案存在，否則 `false`
pub fn file_exists(file_path: &str) -> bool {
    Path::new(file_path).exists()
}

/// 該檔案路徑的上一層的資料夾路徑
/// # 參數
/// - `file_path`: 檔案路徑
/// # 回傳
/// - `Some(&Path)` 如果有上一層資料夾則回傳其路徑，否則回傳 `None`
pub fn file_parent_dir(file_path: &str) -> Option<&Path> {
    Path::new(file_path).parent()
}

/// 該檔案路徑的檔名（不包含副檔名）
/// # 參數
/// - `file_path`: 檔案路徑
/// # 回傳
/// - `Some(&str)` 如果有檔名則回傳其名稱，否則回傳 `None`
pub fn file_stem(file_path: &str) -> Option<&str> {
    Path::new(file_path).file_stem().and_then(|s| s.to_str())
}

/// 產生一個完整的檔案路徑
/// # 參數
/// - `path`: 檔案路徑
/// - `format`: 檔案格式，例如 "mp4", "mkv"
/// # 回傳
/// - `Ok(PathBuf)` 如果成功，回傳一個完整的檔案路徑
/// - `Err(Error)` 如果無法取得父目錄，則回傳錯誤
pub fn full_path_maker(path: &str, format: &str) -> Result<PathBuf, Error>{
    
    let dir_path = match file_parent_dir(path) {
        Some(dir) => dir,
        None => {
            let error_message = &format!("無法取得父目錄: {}", path);
            return Err(io_error_maker(ErrorKind::NotFound, error_message));
        },
    };
    
    let prefix = file_stem(path).unwrap_or("output");
    let filename = timestamp_filename(prefix, format);

    Ok(dir_path.join(filename))
}
