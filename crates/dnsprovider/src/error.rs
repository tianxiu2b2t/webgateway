pub type DNSProviderResult<T> = Result<T, DNSProviderError>;

#[derive(Debug)]
pub enum DNSProviderError {
    RequestError(reqwest::Error),
    ResponseJsonDecodeError(reqwest::Error),
    ResponseError(String),
}
