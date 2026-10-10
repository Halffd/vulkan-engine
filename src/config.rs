use anyhow::Result;
use std::path::Path;

pub enum ConfigType {
    Toml,
    Yaml,
    Json,
    Ini,
}

#[derive(Debug)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Vulkan Engine".to_string(),
            width: 800,
            height: 600,
        }
    }
}

#[derive(Debug)]
pub struct Paths {
    pub sprite: &'static str,
    pub texture: &'static str,
    pub shader: &'static str,
    pub mesh: &'static str,
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            sprite: "assets/images",
            texture: "assets/textures",
            shader: "shaders",
            mesh: "assets/meshes",
        }
    }
}

#[derive(Debug)]
pub struct Files {
    pub sprite: Vec<String>,
    pub texture: Vec<String>,
    pub shader: Vec<String>,
    pub mesh: Vec<String>,
    pub config: Vec<String>,
}

impl Default for Files {
    fn default() -> Self {
        Self {
            sprite: Vec::new(),
            texture: Vec::new(),
            shader: Vec::new(),
            mesh: Vec::new(),
            config: Vec::new(),
        }
    }
}

pub fn load_config() -> Result<WindowConfig> {
    // For now, return default config
    // In the future, this could load from config files
    Ok(WindowConfig::default())
}

pub fn find_config_files(files: &Files) -> Option<String> {
    files
        .config
        .iter()
        .find(|p| {
            p.ends_with(".toml")
                || p.ends_with(".yaml")
                || p.ends_with(".yml")
                || p.ends_with(".json")
                || p.ends_with(".ini")
        })
        .cloned()
}

pub fn merge_config_files(config_files: &str) -> Result<String> {
    let mut config = String::new();
    for config_file in [config_files] {
        config.push_str(&std::fs::read_to_string(config_file)?);
    }
    Ok(config)
}

pub fn find_config_files_old() -> Result<Vec<String>> {
    let mut config_files = Vec::new();

    // Look for common config file locations
    let config_paths = [
        "config.toml",
        "config.yaml",
        "config.yml",
        "config.json",
        "config.ini",
    ];

    for path in config_paths {
        if Path::new(path).exists() {
            config_files.push(path.to_string());
        }
    }

    Ok(config_files)
}
