
use std::{
    fs::read_to_string,
};

use tauri::{
    AppHandle, 
    Manager,
    path:: {BaseDirectory},
};

/// 讀取JSON檔案資料夾檔名列表
/// ## 參數
/// - `app`: Tauri 應用的 AppHandle
/// - `path`: 資料夾路徑
/// - `filename`: 檔案名稱
/// ## 返回
/// - `String`: 成功時返回記錄的 JSON 字符串，失敗
pub fn read_file(app: AppHandle, path: &str, filename: &str) -> Result<String, String> {

    let file_path = app.path()
        .resolve(path, BaseDirectory::Resource)
        .map_err(|error| format!("無法獲取資源目錄: {}", error))?
        .join(filename);
    
    read_to_string(&file_path)
        .map_err(|error| format!("無法讀取檔案: {}", error))
}