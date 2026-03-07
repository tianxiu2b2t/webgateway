export type DNSProviderType = 'tencent';
export interface DNSProviderTencent {
    secret_id: string;
    secret_key: string;
}
/*
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseDNSProvider {
    pub id: ObjectId,
    #[serde(flatten)]
    pub provider: DatabaseDNSProviderKind,
    pub domains: Vec<String>,
        pub name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
    */
export interface DatabaseDNSProvider<T = any> {
    id: string;
    type: DNSProviderType;
    config: T;
    domains: string[];
    name: string;
    created_at: string;
    updated_at: string;
}
