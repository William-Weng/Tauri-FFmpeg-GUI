
/// 自訂的 FFmpeg 事件類型
pub enum FFmpegEvent {
    Error,
    Finish,
    Progress,
}

/// 為 FFmpegEvent 實現 as_str 方法
impl FFmpegEvent {
    pub fn as_str(&self) -> &'static str {
        match self {
            FFmpegEvent::Error => "error",
            FFmpegEvent::Finish => "finish",
            FFmpegEvent::Progress => "progress",
        }
    }
}