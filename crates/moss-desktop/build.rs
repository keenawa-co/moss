use std::process::Command;

fn main() {
    #[cfg(target_os = "windows")]
    let python_cmd = "python";

    #[cfg(not(target_os = "windows"))]
    let python_cmd = "python3";

    Command::new(python_cmd)
        .arg("importer.py")
        .status()
        .expect("Failed to execute script");
}
