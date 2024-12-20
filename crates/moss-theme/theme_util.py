import argparse
from pathlib import Path

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description="Generate Rust Constants for JSON Schema ")
    parser.add_argument("--schema", type=argparse.FileType('r'), required=True)
    parser.add_argument("--output", type=argparse.FileType('w+'), required=True)
    args = parser.parse_args()

    schema_file = args.schema
    output_file = args.output

    schema = schema_file.read()

    output_code = \
        f"""
use serde_json::{{json, Value}};
        
pub const SCHEMA_{Path(schema_file.name).stem.upper()}: Value = json!({schema}
);
"""

    output_file.write(output_code)
