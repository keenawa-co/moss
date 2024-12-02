use clap::Parser;
use moss_themeconv::json_converter::JsonThemeConverter;
use moss_themeconv::ThemeConverter;
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(about)]
struct InstallationArgs {
    /// Path to the JSON file
    #[arg(short, long)]
    input: PathBuf,

    /// Path to the output directory where the CSS file should be generated
    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    let args = InstallationArgs::parse();
    let converter = JsonThemeConverter::new();

    let input_content = fs::read_to_string(&args.input).unwrap();
    let input_json: Value = serde_json::from_str(&input_content).unwrap();

    let output_name = input_json.get("slug").unwrap().as_str().unwrap();
    let output_path = args.output.join(output_name).with_extension("css");
    let output_css = converter.convert_to_css(input_content).unwrap();

    let mut output_file = fs::File::create(&output_path).unwrap();
    output_file.write_all(output_css.as_bytes()).unwrap();

    println!(
        "Successfully generate {} at {}",
        output_name,
        output_path.display()
    );
}
