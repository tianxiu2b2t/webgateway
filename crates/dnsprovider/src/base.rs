use crate::error::DNSProviderResult;

pub trait BaseRecord {
    fn hostname(&self) -> &str;
    fn domain(&self) -> &str;
    fn record_type(&self) -> &str;
    fn record_value(&self) -> &str;
}

pub trait BaseDNS<ListRecordType>
where
    ListRecordType: BaseRecord,
{
    fn add_record(
        &self,
        domain: &str,
        hostname: &str,
        record_type: &str,
        record_value: &str,
    ) -> impl Future<Output = DNSProviderResult<()>> + Send;

    fn remove_record(
        &self,
        domain: &str,
        hostname: &str,
        record_type: &str,
    ) -> impl Future<Output = DNSProviderResult<Option<String>>> + Send;

    fn modify_record(
        &self,
        domain: &str,
        hostname: &str,
        record_type: &str,
        record_value: &str,
    ) -> impl Future<Output = DNSProviderResult<Option<String>>> + Send;

    fn get_record(
        &self,
        domain: &str,
        hostname: &str,
        record_type: &str,
    ) -> impl Future<Output = DNSProviderResult<Option<String>>> + Send;

    fn list_records(
        &self,
        domain: &str,
    ) -> impl Future<Output = DNSProviderResult<Vec<ListRecordType>>> + Send;
}
