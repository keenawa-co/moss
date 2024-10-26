use std::process::Command;

fn main() {
    Command::new("python3")
        .arg("importer.py")
        .status()
        .expect("Failed to execute script");
}
