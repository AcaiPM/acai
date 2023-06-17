use std::path::Path;
use std::process::Command;

// need use something built in later
fn copy(src: &str, dest: &str) {
    let args = ["-a", src, dest];

    let copy = Command::new("cp")
        .env("PATH", "/bin")
        .args(&args)
        .output()
        .expect("Failed to execute cp");

    assert!(copy.status.success());
}

pub fn xattr(path: &str) {
    let args = ["-cr", path];

    let xattr = Command::new("xattr")
        .env("PATH", "/usr/bin")
        .args(&args)
        .output()
        .expect("Failed to execute xattr");

    assert!(xattr.status.success());
}

pub fn install_app(app_path: &str, app_name: &str) -> (i32, String) {
    let install_path = format!("/Applications/{}", app_name);

    if !Path::new(&app_path).exists() {
        return (-1, "Unable to find application path.".to_string());
    }

    copy(&app_path, &install_path); // need add checks after copy
    return (0, install_path);
}
