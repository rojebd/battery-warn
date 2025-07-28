use std::{fs, path::PathBuf, process, thread, time};

// Requires the use of fyi for sending notifications: "https://codeberg.org/dnkl/fyi"
// Main battery you want to track
static MAIN_BATTERY: &str = "BAT0";

fn get_battery() -> Option<PathBuf> {
    for dir in fs::read_dir("/sys/class/power_supply/").unwrap() {
        let dir = dir.expect("[ERROR] Reading directory failed");
        if dir.file_name() == MAIN_BATTERY {
            let dir = dir.path().join("capacity");
            return Some(dir);
        }
    }
    None
}

fn read_battery_contents(battery: &PathBuf) -> u8 {
    let file: String = fs::read_to_string(battery)
        .expect("[ERROR] Can't read file to string")
        .replace("\n", "");
    // println!("{:?}", file.parse::<u8>().unwrap());
    file.parse::<u8>().unwrap()
}

fn main() {
    println!("[LOG] Battery-Warn started");
    let battery_path = get_battery();
    if let Some(battery_path) = battery_path {
        loop {
            let battery = read_battery_contents(&battery_path);
            if battery < 20 {
                let _ = process::Command::new("fyi")
                    .arg("LOW BATTERY")
                    .arg("Please plug in to power.")
                    .status();
            } else if battery > 85 {
                let _ = process::Command::new("fyi")
                    .arg("FULL BATTERY")
                    .arg("Please unplug power.")
                    .status();
            }
            // println!("{}", battery);
            // 300 seconds = 5 minutes
            thread::sleep(time::Duration::from_secs(300));
        }
    }
    println!("[LOG] Battery-Warn ended");
}
