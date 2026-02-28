// 去敏感信息
pub trait RemovedSensitiveInfo {
    fn remove_sensitive_info(&self) -> Self;
}