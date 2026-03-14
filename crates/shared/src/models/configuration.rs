use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sqlx::{FromRow, Row, postgres::PgRow, types::Json};

#[derive(Debug, Clone)]
pub struct Configuration<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    key: String,
    value: Json<ConfigurationHelper<T>>,
}

unsafe impl<T> Sync for Configuration<T> where T: Serialize + DeserializeOwned + Clone {}
unsafe impl<T> Send for Configuration<T> where T: Serialize + DeserializeOwned + Clone {}
impl<T> Unpin for Configuration<T> where T: Serialize + DeserializeOwned + Clone {}

impl<T> Configuration<T> where
    T: Serialize + DeserializeOwned + Clone,
 {
    pub fn new(key: impl Into<String>, value: T) -> Self {
        Self {
            key: key.into(),
            value: Json(ConfigurationHelper { data: value }),
        }
    }

    pub fn value(&self) -> &T {
        &self.value.data
    }

    pub fn into_value(self) -> T {
        self.value.data.clone()
    }

    pub(crate) fn get_helper_value(&self) -> &ConfigurationHelper<T> {
        &self.value.0
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn set_value(&mut self, value: T) {
        self.value = Json(ConfigurationHelper { data: value });
    }

    pub fn set_key(&mut self, key: impl Into<String>) {
        self.key = key.into();
    }
}


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(bound = "T: Serialize + DeserializeOwned")]
pub(crate) struct ConfigurationHelper<T>
where
    T: Serialize + DeserializeOwned,
{
    data: T,
}


impl<'r, T> FromRow<'r, PgRow> for Configuration<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            key: row.try_get("key")?,
            value: row.try_get::<Json<ConfigurationHelper<T>>, _>("value")?,
        })
    }
}