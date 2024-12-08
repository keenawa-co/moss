use clap::Parser;
use moss_themeconv::{
    json_converter::JsonThemeConverter, jsonschema_validator::JsonSchemaValidator, ThemeConverter,
};
use serde_json::Value;
use std::{fs, io::Write, path::PathBuf, sync::Arc};

#[derive(Parser, Debug)]
#[command(about)]
struct InstallationArgs {
    /// Path to the JSON Schema
    #[arg(short, long)]
    schema: PathBuf,

    /// Path to the JSON file
    #[arg(short, long)]
    input: PathBuf,

    /// Path to the output directory where the CSS file should be generated
    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    let args = InstallationArgs::parse();

    let schema_str =
        fs::read(args.schema).expect("Failed to read the file. Ensure the path is accessible.");
    let schema: serde_json::Value = serde_json::from_slice(&schema_str)
        .expect("Failed to load the JSON schema. Please ensure the schema file is valid.");

    let validator = JsonSchemaValidator::new(Arc::new(schema));
    let converter = JsonThemeConverter::new(Arc::new(validator));
    let input_content = fs::read_to_string(&args.input).unwrap_or_else(|_| {
        panic!(
            "Failed to read the input file at '{}'. Ensure the file exists and is readable.",
            args.input.display()
        )
    });

    let input_json: Value = serde_json::from_str(&input_content).unwrap_or_else(|_| {
        panic!("Failed to parse the input JSON file. Ensure the file contains valid JSON.")
    });

    let output_name = input_json
        .get("slug")
        .and_then(|slug| slug.as_str())
        .unwrap_or_else(|| {
            panic!(
                "The input JSON is missing the required 'slug' field or the field is not a string."
            )
        });

    let output_path = args.output.join(output_name).with_extension("css");

    let output_css = converter
        .convert_to_css(input_content)
        .unwrap_or_else(|err| {
            panic!("Failed to convert JSON to CSS. Error: {}", err);
        });

    let mut output_file = fs::File::create(&output_path).unwrap_or_else(|_| {
        panic!(
            "Failed to create the output file at '{}'. Check your permissions or directory path.",
            output_path.display()
        )
    });

    output_file
        .write_all(output_css.as_bytes())
        .unwrap_or_else(|_| {
            panic!(
                "Failed to write CSS content to the file at '{}'.",
                output_path.display()
            )
        });

    println!(
        "Successfully generated CSS file '{}' at '{}'",
        output_name,
        output_path.display()
    );
}
