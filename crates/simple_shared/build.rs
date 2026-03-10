use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    version: String,
    project: String,
}

pub fn main() {
    println!("cargo:rerun-if-changed=../../../Cargo.toml");
    println!("cargo:rerun-if-changed=build.rs");
    let content = include_str!("../../Cargo.toml");
    let info: Info = toml::from_str(content).unwrap();
    println!("cargo:rustc-env=WG_VERSION={}", info.version);
    println!("cargo:rustc-env=WG_PROJECT={}", info.project);
    // 生成 info.rs

}