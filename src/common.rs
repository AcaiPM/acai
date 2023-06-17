use os_version::MacOS;
use uname::{uname, Info};

use crate::seed::Seed;

pub fn to_val(input: (i32, String)) -> i32 {
    return input.0;
}

pub fn to_str(input: (i32, String)) -> String {
    return input.1;
}

pub fn architecture() -> (i32, String) {
    let sys_info: Info = uname().unwrap();

    match sys_info.machine.as_ref() {
        "x86_64" => return (0, "amd64".to_string()),
        "x86" => return (-1, "Unsupported".to_string()),
        "arm64" => return (0, "arm64".to_string()),
        "arm64e" => return (0, "arm64".to_string()),
        "" => return (-1, "Unsupported".to_string()),
        _ => return (-1, "Unsupported".to_string()),
    }
}

pub fn macos_version() -> (i32, String) {
    let os_info = MacOS::detect().unwrap();

    if os_info.version.to_string().is_empty() {
        return (-1, "Unknown OS Version.".to_string());
    }

    return (0, os_info.version.to_string());
}

pub fn get_download_url(seed: &Seed) -> String {
    return seed
        .downloads
        .universal
        .replace("#{version}", &seed.app_version);
}
