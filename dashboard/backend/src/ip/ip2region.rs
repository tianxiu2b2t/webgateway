use std::{collections::HashMap, net::IpAddr, path::PathBuf, sync::LazyLock};

use anyhow::anyhow;
use ip2region::Searcher;

use crate::ip::{LookupResult, ROOT};

static V4_FILE: LazyLock<PathBuf> = LazyLock::new(|| ROOT.clone().join("ip2region_v4.xdb"));
static V6_FILE: LazyLock<PathBuf> = LazyLock::new(|| ROOT.clone().join("ip2region_v6.xdb"));

static V4_INSTANCE: LazyLock<Searcher> = LazyLock::new(|| {
    Searcher::new(V4_FILE.clone()).unwrap()
});

static V6_INSTANCE: LazyLock<Searcher> = LazyLock::new(|| {
    Searcher::new(V6_FILE.clone()).unwrap()
});

static MAPPING_PROVINCE_TO_COUNTRY: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("香港特别行政区", "HK");
    map.insert("澳门特别行政区", "MO");
    map.insert("台湾省", "TW");
    map.iter().map(|v| (v.0.to_string(), v.1.to_string())).collect()
});

static MAPPING_PROVINCE_TO_CODES: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("广东省", "GD");
    map.insert("内蒙古", "NM");
    map.insert("山东省", "SD");
    map.insert("吉林省", "JL");
    map.insert("贵州省", "GZ");
    map.insert("云南省", "YN");
    map.insert("北京市", "BJ");
    map.insert("西藏自治区", "XZ");
    map.insert("四川省", "SC");
    map.insert("天津市", "TJ");
    map.insert("宁夏回族自治区", "NX");
    map.insert("安徽省", "AH");
    map.insert("广西壮族自治区", "GX");
    map.insert("江西省", "JX");
    map.insert("海南省", "HI");
    map.insert("河南省", "HA");
    map.iter().map(|v| (v.0.to_string(), v.1.to_string())).collect()
});

#[derive(Clone, Debug)]
pub struct Ip2Region {
    pub country: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub isp: Option<String>,
    pub country_code: Option<String>,
}

fn inner_lookup(
    ip: IpAddr
) -> anyhow::Result<Ip2Region> {
    let searcher = if ip.is_ipv4() {
        &V4_INSTANCE
    } else {
        &V6_INSTANCE
    };

    let data = searcher.search(&ip.to_string()).map_err(|e| anyhow!(format!("{e:?}")))?;
    let data = data.split('|').collect::<Vec<&str>>();
    let result = Ip2Region {
        country: data.first().filter(|v| v.to_string() != "0").map(|v| v.to_string()),
        province: data.get(1).filter(|v| v.to_string() != "0").map(|v| v.to_string()).map(|v| MAPPING_PROVINCE_TO_CODES.get(&v).map(|v| v.to_string()).unwrap_or(v)),
        city: data.get(2).filter(|v| v.to_string() != "0").map(|v| v.to_string()),
        isp: data.get(3).filter(|v| v.to_string() != "0").map(|v| v.to_string()),
        country_code: data.get(4).filter(|v| v.to_string() != "0").map(|v| v.to_string()),
    };

    Ok(result)
}

pub fn lookup(ip: IpAddr) -> anyhow::Result<LookupResult> {
    inner_lookup(ip).map(|v| LookupResult { ip, country: {
        if let Some(province) = &v.province && let Some(country) = MAPPING_PROVINCE_TO_COUNTRY.get(province) {
            Some(country.clone())
        } else {
            v.country_code
        }
    }, city: v.province })
}
