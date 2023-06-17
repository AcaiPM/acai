use home::home_dir;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::Command;

static TMP: &str = ".acai/temp";

// Returns temp mount path
pub fn mount_dir() -> String {
    let home_dir = String::from(home_dir().unwrap().to_string_lossy());
    return format!("{}/{}/mount", home_dir, TMP);
}

// Returns temp path
pub fn temp_dir() -> String {
    let home_dir = String::from(home_dir().unwrap().to_string_lossy());
    return format!("{}/{}", home_dir, TMP);
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
    let args = [
        "attach",
        "-nobrowse",
        "-noautoopenro",
        dmg_path,
        "-mountpoint",
        &mount_dir(),
    ];

    return hdiutil(&args);
}

// Mounts dmg at temp mount point and returns the path to the .app
pub fn get_app_path(dmg_path: &str, app_name: &str) -> (i32, String) {
    detach(&mount_dir()); // clear mount directory
    attach(dmg_path);

    let app_path = format!("{}/{}", mount_dir(), app_name);

    if !Path::new(&dmg_path).exists() {
        return (-1, "Path to dmg is invalid".to_string());
    }

    if !Path::new(&app_path).exists() {
        return (-1, "Path to .app is invalid".to_string());
    }

    return (0, app_path);
}

pub fn download_dmg(url: &str) {
    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                let file = File::create(format!("{}/app.dmg", temp_dir())).unwrap();
                let mut buf_writer = BufWriter::new(file);
                let mut buffer = Vec::new();
                response.into_reader().read_to_end(&mut buffer).unwrap();
                buf_writer.write(&buffer).unwrap();
            } else {
                println!("Failed to download file.");
            }
        }
        Err(ureq::Error::Status(status, response)) => {
            println!(
                "Request failed with status {}: {}",
                status,
                response.into_string().unwrap_or_default()
            );
        }
        Err(err) => {
            println!("Request failed with error: {}", err);
        }
    }
}
