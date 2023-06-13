use std::process::Command;
use std::env::home_dir;
use std::path::Path;

static TMP_MNT: &str = ".acai/temp/mount";

// Retruns temp mount path
fn mount_dir() -> String {
    let home_dir = String::from(home_dir().unwrap().to_string_lossy());
    return format!("{}/{}", home_dir, TMP_MNT);
}

// Executes the hdiutil command with the given arguments.
fn hdiutil(args: &[&str]) -> (i32, String) {
    let hdiutil = Command::new("hdiutil")
                    .env("PATH", "/usr/bin")
                    .args(args)
                    .output()
                    .expect("Failed to execute hdiutil");
                
    let stdout: String = String::from_utf8(hdiutil.stdout).unwrap();
    let status: i32 = hdiutil.status.code().unwrap_or(-1);  

    return (status, stdout);      
}

// 'hdiutil detach <mount_point>'
pub fn detach(mount_point: &str) -> (i32, String) {
    let args = ["detach", mount_point];

    return hdiutil(&args);
}

// 'hdiutil attach <path_to_dmg>'
pub fn attach(dmg_path: &str) -> (i32, String) {
    let args = ["attach", "-nobrowse",
                "-noautoopenro", dmg_path,
                "-mountpoint", &mount_dir()];
    
    return hdiutil(&args);
}

// Mounts dmg at temp mount point and returns the path to the .app
pub fn get_app_path(dmg_path: &str, app_name: &str) -> (i32, String) {
    detach(&mount_dir()); // clear mount directory
    attach(dmg_path);
    
    let app_path = format!("{}/{}", mount_dir(), app_name);

    if !Path::new(&dmg_path).exists() {
        return (-1, "Path to dmg is invalid".to_string())
    } 

    if !Path::new(&app_path).exists() {
        return (-1, "Path to .app is invalid".to_string())
    } 

    return (0, app_path);
}