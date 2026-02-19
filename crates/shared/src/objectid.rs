use chrono::{DateTime, Utc};
use sqlx::postgres::PgHasArrayType;
use std::error::Error as StdError;
use std::{
    fmt,
    str::FromStr,
    sync::atomic::{AtomicUsize, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

/// BSON ObjectId 是一个 12 字节的唯一标识符，结构如下：
/// - 4 字节：时间戳（自 Unix 纪元以来的秒数）
/// - 5 字节：随机值（通常为机器标识符 + 进程标识符）
/// - 3 字节：自增计数器（起始随机，保证同一进程内的唯一性）
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectId {
    bytes: [u8; 12],
}

impl ObjectId {
    /// 生成一个新的 ObjectId
    pub fn new() -> Self {
        let timestamp = Self::timestamp_bytes();
        let random = Self::random_bytes();
        let counter = Self::counter_bytes();

        let mut bytes = [0u8; 12];
        bytes[0..4].copy_from_slice(&timestamp);
        bytes[4..9].copy_from_slice(&random);
        bytes[9..12].copy_from_slice(&counter);
        ObjectId { bytes }
    }

    /// 从 12 字节的数组构造 ObjectId
    pub fn from_bytes(bytes: [u8; 12]) -> Self {
        ObjectId { bytes }
    }

    /// 返回内部字节数组的引用
    pub fn as_bytes(&self) -> &[u8; 12] {
        &self.bytes
    }

    /// 将 ObjectId 转换为十六进制字符串
    pub fn to_hex(&self) -> String {
        hex::encode(self.bytes)
    }

    /// 从十六进制字符串解析 ObjectId
    pub fn from_hex(s: &str) -> Result<Self, ObjectIdError> {
        let bytes = hex::decode(s).map_err(|_| ObjectIdError::InvalidHex)?;
        if bytes.len() != 12 {
            return Err(ObjectIdError::InvalidLength);
        }
        let mut arr = [0u8; 12];
        arr.copy_from_slice(&bytes);
        Ok(ObjectId { bytes: arr })
    }

    /// 返回 ObjectId 的时间戳部分（秒数）
    pub fn timestamp(&self) -> u32 {
        u32::from_be_bytes([self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3]])
    }

    /// 返回时间戳对应的 DateTime<Utc>
    pub fn datetime(&self) -> DateTime<Utc> {
        let secs = self.timestamp() as i64;
        DateTime::from_timestamp(secs, 0).expect("valid timestamp")
    }

    // 内部函数：获取当前时间的秒数（4 字节大端序）
    fn timestamp_bytes() -> [u8; 4] {
        let start = SystemTime::now();
        let since_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards");
        (since_epoch.as_secs() as u32).to_be_bytes()
    }

    // 内部函数：生成 5 字节随机数（机器 + 进程标识）
    fn random_bytes() -> [u8; 5] {
        let mut bytes = [0u8; 5];
        rand::fill(&mut bytes);
        bytes
    }

    // 内部函数：获取 3 字节的原子计数器（大端序）
    fn counter_bytes() -> [u8; 3] {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);

        // 初始化时随机化起始值（避免不同进程/实例产生相同计数器序列）
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| {
            let initial = rand::random::<u16>() as usize;
            COUNTER.store(initial, Ordering::Relaxed);
        });

        let count = COUNTER.fetch_add(1, Ordering::Relaxed) & 0x00ff_ffff; // 只保留低 24 位
        let [_, b1, b2, b3] = (count as u32).to_be_bytes();
        [b1, b2, b3]
    }

    fn get_type_size() -> usize {
        24
    }
}

impl Default for ObjectId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ObjectId({})", self.to_hex())
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl FromStr for ObjectId {
    type Err = ObjectIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s)
    }
}

/// 解析 ObjectId 时可能发生的错误
#[derive(Debug, thiserror::Error)]
pub enum ObjectIdError {
    #[error("invalid hex string")]
    InvalidHex,
    #[error("invalid length, expected 12 bytes")]
    InvalidLength,
}

// 可选：实现 serde 的序列化和反序列化
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};
use sqlx::{
    Decode, Encode, Postgres, Type,
    encode::IsNull,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
};

impl Serialize for ObjectId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_hex())
        } else {
            serializer.serialize_bytes(&self.bytes)
        }
    }
}

impl<'de> Deserialize<'de> for ObjectId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let s = String::deserialize(deserializer)?;
            ObjectId::from_hex(&s).map_err(D::Error::custom)
        } else {
            let bytes: &[u8] = <&[u8]>::deserialize(deserializer)?;
            if bytes.len() != 12 {
                return Err(D::Error::invalid_length(bytes.len(), &"12 bytes"));
            }
            let mut arr = [0u8; 12];
            arr.copy_from_slice(bytes);
            Ok(ObjectId { bytes: arr })
        }
    }
}

// support sqlx pg row
impl<'q> Encode<'q, Postgres> for ObjectId {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, Box<dyn StdError + Send + Sync + 'static>> {
        let bytes = self.to_hex();
        if bytes.len() != Self::get_type_size() {
            return Err(format!(
                "Invalid size for usize, data_len: {}, expected: {}",
                bytes.len(),
                Self::get_type_size()
            )
            .into());
        }
        <[u8; 24] as Encode<Postgres>>::encode_by_ref(bytes.as_bytes().try_into().unwrap(), buf)
    }

    fn size_hint(&self) -> usize {
        Self::get_type_size()
    }
}

impl<'r> Decode<'r, Postgres> for ObjectId {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        // 从大端字节序解析为usize
        let bytes = <[u8; 24] as Decode<Postgres>>::decode(value)?;
        if bytes.len() != Self::get_type_size() {
            return Err(format!(
                "Invalid size for usize, data_len: {}, expected: {}",
                bytes.len(),
                Self::get_type_size()
            )
            .into());
        }
        Ok(ObjectId::from_hex(&String::from_utf8(bytes.to_vec())?)?)
    }
}

impl Type<Postgres> for ObjectId {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("TEXT")
    }
}

impl PgHasArrayType for ObjectId {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("TEXT[]")
    }
}
