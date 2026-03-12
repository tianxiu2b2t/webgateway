use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    version: String,
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    project: Project,
}

pub fn main() {
    println!("cargo:rerun-if-changed=../../project.toml");
    println!("cargo:rerun-if-changed=build.rs");
    let content = include_str!("../../project.toml");
    let cfg: ProjectConfig = toml::from_str(content).unwrap();
    println!("cargo:rustc-env=WG_VERSION={}", cfg.project.version);
    println!("cargo:rustc-env=WG_PROJECT_NAME={}", cfg.project.name);
    // 生成 info.rs
}
