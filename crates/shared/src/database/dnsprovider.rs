use crate::models::dns::{DNSProvider, DNSProviderUpdate};

use crate::database::Database;

pub trait DatabaseDNSProvider {
    fn get_dns_provider_from_id(
        &self,
        id: String,
    ) -> impl Future<Output = anyhow::Result<DNSProvider>> + Send;
    // add
    fn add_dns_provider(
        &self,
        dns_provider: DNSProvider,
    ) -> impl Future<Output = anyhow::Result<DNSProvider>> + Send;
    // update
    fn update_dns_provider(
        &self,
        dns_provider: DNSProviderUpdate,
    ) -> impl Future<Output = anyhow::Result<DNSProvider>> + Send;

    // init table
    fn init_dns_provider_table(&self) -> impl Future<Output = anyhow::Result<()>> + Send;
}

/*#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSProvider {
    pub id: String,
    #[serde(flatten)]
    pub provider: DNSProviderKind,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
} */

// impl DatabaseDNSProvider for Database {
//     // async fn add_dns_provider(&self, dns_provider: DNSProvider) -> anyhow::Result<DNSProvider> {
//     //     self.add_dns_provider(dns_provider).await
//     // }

//     // async fn get_dns_provider_from_id(&self, id: String) -> anyhow::Result<DNSProvider> {
//     //     self.get_dns_provider_from_id(id).await
//     // }

//     // async fn update_dns_provider(&self, dns_provider: DNSProviderUpdate) -> anyhow::Result<()> {
//     //     sqlx::query("UPDATE dns_providers SET type = $1, config = $2, updated_at = NOW() WHERE id = $3").bind(
//     //         dns_provider.provider.get_type(),
//     //     ).bind(            serde_json::to_value(dns_provider.provider)?)
//     //     .bind(dns_provider.id)
//     //     .execute(&self.pool)
//     //     .await?;
//     //     Ok(())
//     // }

//     // async fn init_dns_provider_table(&self) -> anyhow::Result<()> {
//     //     sqlx::query(r#"CREATE TABLE IF NOT EXISTS dns_providers(id TEXT PRIMARY KEY, type TEXT NOT NULL, created_at TIMESTAMP NOT NULL, updated_at TIMESTAMP NOT NULL, config jsonb);"#).execute(&self.pool).await;
//     //     Ok(())
//     // }
// }
