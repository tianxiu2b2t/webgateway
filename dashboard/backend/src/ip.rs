use geoip2::{City, Reader};
use std::{net::IpAddr, path::PathBuf, sync::LazyLock};

// Reader now holds a 'static reference to the mmdb data
pub struct CityInstance {
    // We keep the slice only to conceptually own the data,
    // but it's not strictly necessary for functionality.
    _data: &'static [u8],
    reader: Reader<'static, City<'static>>,
}

impl CityInstance {
    pub fn new(data: &'static [u8]) -> Self {
        Self {
            _data: data,
            reader: Reader::<City>::from_bytes(data).unwrap(),
        }
    }

    pub fn lookup(&self, ip: IpAddr) -> anyhow::Result<()> {
        let city = self.reader.lookup(ip);
        println!("{:?}", city);
        Ok(())
    }
}

static ROOT: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("./").join("assets").join("ipdb"));

static CITY_FILE: LazyLock<PathBuf> = LazyLock::new(|| ROOT.clone().join("GeoLite2-City.mmdb"));

static CITY_INSTANCE: LazyLock<CityInstance> = LazyLock::new(|| {
    let content = std::fs::read(CITY_FILE.clone()).unwrap();
    // Leak the Vec to obtain a 'static slice
    let leaked: &'static [u8] = Box::leak(Box::new(content));
    CityInstance::new(leaked)
});

pub fn lookup(ip: IpAddr) -> anyhow::Result<()> {
    CITY_INSTANCE.lookup(ip)
}
