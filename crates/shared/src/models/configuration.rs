use serde::{Deserialize, Serialize, de::DeserializeOwned};
use simple_shared::objectid::ObjectId;
use sqlx::{FromRow, Row, postgres::PgRow, types::Json};

#[derive(Debug, Clone)]
pub struct Configuration<T>
where
    T: Serialize + DeserializeOwned,
{
    id: ObjectId,
    key: String,
    value: Json<ConfigurationHelper<T>>,
}

impl<T> Configuration<T> where
    T: Serialize + DeserializeOwned,
 {
    fn inner_new(id: ObjectId, key: impl Into<String>, value: T) -> Self {
        Self {
            id,
            key: key.into(),
            value: Json(ConfigurationHelper { data: value }),
        }
    }

    pub fn new(key: impl Into<String>, value: T) -> Self {
        Self::inner_new(ObjectId::new(), key, value)
    }

    pub fn value(&self) -> &T {
        &self.value.data
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn id(&self) -> &ObjectId {
        &self.id
    }

    pub fn set_value(&mut self, value: T) {
        self.value = Json(ConfigurationHelper { data: value });
    }

    pub fn set_key(&mut self, key: impl Into<String>) {
        self.key = key.into();
    }
}


#[derive(Debug, Clone, Deserialize)]
#[serde(bound = "T: Serialize + DeserializeOwned")]
struct ConfigurationHelper<T>
where
    T: Serialize + DeserializeOwned,
{
    data: T,
}


impl<'r, T> FromRow<'r, PgRow> for Configuration<T>
where
    T: Serialize + DeserializeOwned,
{
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            key: row.try_get("key")?,
            value: row.try_get::<Json<ConfigurationHelper<T>>, _>("value")?,
        })
    }
}