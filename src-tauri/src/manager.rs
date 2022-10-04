use serde::{Deserialize, Serialize};
use std::fs::File;
use std::process::Command;
use tauri::api::path::local_data_dir;
#[derive(Debug, Clone)]
pub struct Package {
    name: String,
    version: String,
    description: String,
    homepage: String,
    license: String,
    repository: String,
}
#[derive(Debug, Clone)]
pub struct Manager {
    locked: bool,
    currently_installing: Option<String>,
    currently_uninstalling: Option<String>,
    version: String,
    packages: Vec<Package>,
    path: String,
}
#[derive(Serialize, Deserialize)]
struct Settings {
    packageManagerPath: String,
    apiPath: String,
    installOptions: Vec<String>,
}
impl Manager {
    pub fn new() -> Self {
        let path = local_data_dir().unwrap().join("settings.json");
        let file = File::open(path).unwrap();
        let settings: Settings = serde_json::from_reader(file).unwrap();
        let version = Command::new(&settings.packageManagerPath)
            .arg("--version")
            .output()
            .expect("failed to execute process");
        let version = String::from_utf8(version.stdout).unwrap();
        let version = version.split("\n").collect::<Vec<&str>>()[0].to_string();
        let version = version.split(" ").collect::<Vec<&str>>()[1].to_string();

        Self {
            locked: false,
            currently_installing: None,
            currently_uninstalling: None,
            version,
            packages: Vec::new(),
            path: settings.packageManagerPath,
        }
    }
    pub fn get_version(&self) -> String {
        self.version.clone()
    }
    pub fn get_packages(&self) -> Vec<Package> {
        self.packages.clone()
    }
    pub fn get_package(&self, name: &str) -> Option<Package> {
        self.packages.iter().find(|p| p.name == name).cloned()
    }
    pub fn get_currently_installing(&self) -> Option<String> {
        self.currently_installing.clone()
    }
    pub fn get_currently_uninstalling(&self) -> Option<String> {
        self.currently_uninstalling.clone()
    }
    pub fn is_locked(&self) -> bool {
        self.locked
    }
    pub fn update_package(&mut self, package: Package) {
        self.packages.retain(|p| p.name != package.name);
        self.packages.push(package);
    }
    pub fn update_packages(&mut self, packages: Vec<Package>) {
        self.packages = packages;
    }
}
