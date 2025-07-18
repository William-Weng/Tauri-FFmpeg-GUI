use serde::Serialize;
use tauri::{Emitter, Window};

/// 透過IPC（跨程序通訊，Inter-Process Communication）機制，把事件名稱和資料（payload）從 Rust 傳送到前端 WebView
/// # 參數
/// - `target`: 目標窗口
/// - `event`: 事件類型
/// - `payload`: 要發送的資料
pub fn emit_payload<S: Serialize + Clone>(target: &Window, event: &str, payload: S) {
    let _ = target.emit(event, payload);
}
