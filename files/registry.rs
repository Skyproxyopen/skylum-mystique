//! Windows registry read helpers for Skylum Mystique.

pub const APP_KEY: &str = "Software\\Skylum\\Skylum Mystique";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Skylum Mystique";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Skylum Mystique") },
        RegValue { name: "Version",     value: String::from("v2.5.29") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Skylum Mystique") },
        RegValue { name: "Publisher",       value: String::from("Skylum") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Skylum Mystique\\uninstall.exe\"") },
    ]
}
