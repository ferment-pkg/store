use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, Read};
use std::process::{Command, Stdio};
use std::ptr::read;
use std::sync::mpsc::{Receiver, Sender};
use tauri::api::path::local_data_dir;
use tauri::window::Window;
#[derive(Debug, Clone)]
pub struct Manager {
    locked: bool,
    currently_installing: Option<String>,
    currently_uninstalling: Option<String>,
    version: String,
    packages: Vec<String>,
    path: String,
    queue: Vec<String>,
}
struct ChannelPayload {
    package: String,
    action: String,
    finished: bool,
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
        let packages = Command::new(&settings.packageManagerPath)
            .arg("list")
            .output()
            .expect("failed to execute process");
        let packages = String::from_utf8(packages.stdout).unwrap();
        let packages = packages.split("\n").collect::<Vec<&str>>();
        let mut packages = packages
            .iter()
            .map(|x| x.to_string())
            .filter(|x| x != "")
            .collect::<Vec<String>>();
        packages.remove(0);

        Self {
            locked: false,
            currently_installing: None,
            currently_uninstalling: None,
            version,
            packages,
            path: settings.packageManagerPath,
            queue: Vec::new(),
        }
    }
    pub fn new_test(path: String) -> Self {
        let version = Command::new(&path)
            .arg("--version")
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute process");
        let version = String::from_utf8(version.stdout).unwrap();

        let version = version.split("\n").collect::<Vec<&str>>()[0].to_string();

        let version = version.split(" ").collect::<Vec<&str>>()[1].to_string();
        let packages = Command::new(&path)
            .arg("list")
            .output()
            .expect("failed to execute process");

        let packages = String::from_utf8(packages.stdout).unwrap();

        let packages = packages.split("\n").collect::<Vec<&str>>();

        let mut packages = packages
            .iter()
            .map(|x| x.to_string())
            .filter(|x| x != "")
            .collect::<Vec<String>>();

        packages.remove(0);
        Self {
            locked: false,
            currently_installing: None,
            currently_uninstalling: None,
            version,
            packages,
            path,
            queue: Vec::new(),
        }
    }
    pub fn get_version(&self) -> String {
        self.version.clone()
    }
    pub fn get_packages(&self) -> Vec<String> {
        self.packages.clone()
    }
    pub fn get_package(&self, name: &str) -> Option<String> {
        self.packages
            .iter()
            .find(|p| p.to_string() == name.to_string())
            .cloned()
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
    pub fn update_package(&mut self, package: String) {
        self.packages.retain(|p| p.to_string() != package);
        self.packages.push(package);
    }
    pub fn update_packages(&mut self, packages: Vec<String>) {
        self.packages = packages;
    }
    //implement future

    pub fn install_package(&mut self, package: String) {
        while self.locked {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        self.currently_installing = Some(package.clone());

        self.locked = true;
        //spawn a thread
        let path = self.path.clone();
        //create a channel

        let (tx, rx): (Sender<ChannelPayload>, Receiver<ChannelPayload>) =
            std::sync::mpsc::channel();
        std::thread::spawn(move || {
            //mock a stdout
            let cmd = Command::new(path)
                .arg("install")
                .arg(package)
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to execute process");
            let output = cmd.stdout.unwrap();
            let reader = std::io::BufReader::new(output);
            for line in reader.lines() {
                let line = line.unwrap();
                let line = line.split(" ").collect::<Vec<&str>>();
                let package = line[0].to_string();
                let action = line[1].to_string();
                let finished = line[2].to_string() == "true";
                tx.send(ChannelPayload {
                    package,
                    action,
                    finished,
                })
                .unwrap();
            }
        });
        loop {
            let payload = rx.recv().unwrap();
            if payload.finished {
                break;
            }
        }
    }
}
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let manager = Manager::new_test("/usr/local/ferment/ferment".to_string());
        assert_eq!(manager.get_version(), "0.4.9-beta");
    }
    #[test]
    fn test_get_packages() {
        let mut manager = Manager::new_test("/usr/local/ferment/ferment".to_string());
        manager.install_package("test".to_string());
        assert_eq!(manager.get_packages().len(), 1);
    }
}
