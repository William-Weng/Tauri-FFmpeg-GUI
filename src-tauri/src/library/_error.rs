use std::io::{Error, ErrorKind};

/// 產生一個io錯誤
/// # 參數
/// - kind: 錯誤類型
/// - message: 錯誤訊息
/// # 回傳
/// - `Error` - 回傳一個io錯誤
pub fn io_error_maker(kind: ErrorKind, message: &str) -> Error {
    Error::new(kind, message)
}