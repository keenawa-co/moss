use once_cell::sync::Lazy;
use schemars::{json_schema, Schema};

static THEME_SCHEMA: Lazy<Schema> = Lazy::new(|| {
    json_schema!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "Theme.json",
        "type": "object",
        "properties": {
            "name": {
              "type": "string",
            },
            "slug": {
              "type": "string",
            },
            "type": {
              "type": "string",
             },
            "isDefault": {
              "type": "boolean",
            },
            "colors": {
              "$ref": "#/$defs/Colors",
            }
        },
        "required": ["name", "slug", "type", "isDefault", "color"],
        "$defs": {
            "Colors": {
                "type": "object",
                "properties": {
                    "primary": { "$ref": "#/$defs/ColorType" },
                    "sideBar.background": { "$ref": "#/$defs/ColorType" },
                    "toolBar.background": { "$ref": "#/$defs/ColorType" },
                    "page.background": { "$ref": "#/$defs/ColorType" },
                    "statusBar.background": { "$ref": "#/$defs/ColorType" },
                    "windowsCloseButton.background": { "$ref": "#/$defs/ColorType" },
                    "windowControlsLinux.background": { "$ref": "#/$defs/ColorType" },
                    "windowControlsLinux.text": { "$ref": "#/$defs/ColorType" },
                    "windowControlsLinux.hoverBackground": { "$ref": "#/$defs/ColorType" },
                    "windowControlsLinux.activeBackground": { "$ref": "#/$defs/ColorType" }
                },
                "required": [
                    "primary",
                    "sideBar.background",
                    "toolBar.background",
                    "page.background",
                    "statusBar.background",
                    "windowsCloseButton.background",
                    "windowControlsLinux.background",
                    "windowControlsLinux.text",
                    "windowControlsLinux.hoverBackground",
                    "windowControlsLinux.activeBackground"
                ]
            },
            "ColorType": {
                "anyOf": [
                    {
                        "type": "object",
                        "properties": {
                            "type": { "type": "string", "const": "solid" },
                            "value": { "$ref": "#/$defs/ColorTokenValue" }
                        },
                        "required": ["type", "value"]
                    },
                    {
                        "type": "object",
                        "properties": {
                            "type": { "type": "string", "const": "gradient" },
                            "direction": { "type": "string" },
                            "value": {
                                "type": "array",
                                "items": { "$ref": "#/$defs/ColorGradientEntry" }
                            }
                        },
                        "required": ["type", "direction", "value"]
                    }
                ]
            },
            "ColorTokenValue": {
                "type": "string",
                "pattern": "#[0-9a-fA-F]{3,8}|rgb(a)?\\(.+\\)|hsl(a)?\\(.+\\)|[a-z]+"
            },
            "ColorGradientEntry": {
                "type": "object",
                "properties": {
                    "color": { "$ref": "#/$defs/ColorTokenValue" },
                    "position": {
                        "anyOf": [
                            { "type": "number" },
                            { "type": "string" }
                        ]
                    }
                },
                "required": ["color", "position"]
            }
        }
    })
});

#[cfg(test)]
mod tests {
    use super::*;
    use schemars::Schema;

    #[test]
    fn test_2() {
        let schema_json = serde_json::to_string_pretty(&*THEME_SCHEMA).unwrap();
        println!("{}", schema_json);
    }

    #[test]
    fn test() {
        use serde_json::{json, Map, Value};

        let mut schema = Schema::default();

        {
            let root_obj = schema.ensure_object();

            root_obj.insert(
                "$schema".into(),
                Value::String("https://json-schema.org/draft/2020-12/schema".into()),
            );
            root_obj.insert("$id".into(), Value::String("Moss Theme".into()));

            root_obj.insert("type".into(), Value::String("object".into()));

            let mut properties = Map::new();
            properties.insert("name".into(), json!({"type":"string"}));
            properties.insert("slug".into(), json!({"type":"string"}));
            properties.insert("type".into(), json!({"type":"string"}));
            properties.insert("isDefault".into(), json!({"type":"boolean"}));

            properties.insert("color".into(), json!({"$ref":"#/$defs/Color"}));

            root_obj.insert("properties".into(), Value::Object(properties));
            root_obj.insert(
                "required".into(),
                json!(["name", "slug", "type", "isDefault", "color"]),
            );

            let mut defs = Map::new();

            // Color
            let mut color_obj = Map::new();
            color_obj.insert("type".into(), Value::String("object".into()));

            let mut color_props = Map::new();
            color_props.insert("primary".into(), json!({"$ref":"#/$defs/ColorType"}));
            color_props.insert(
                "sideBar.background".into(),
                json!({"$ref":"#/$defs/ColorType"}),
            );
            color_props.insert(
                "toolBar.background".into(),
                json!({"$ref":"#/$defs/ColorType"}),
            );
            color_props.insert(
                "page.background".into(),
                json!({"$ref":"#/$defs/ColorType"}),
            );
            color_props.insert(
                "statusBar.background".into(),
                json!({"$ref":"#/$defs/ColorType"}),
            );
            color_props.insert(
                "windowsCloseButton.background".into(),
                json!({"$ref":"#/$defs/ColorType"}),
            );
            color_props.insert(
                "windowControlsLinux.background".into(),
                json!({"$ref":"#/$defs/ColorType"}),
            );
            color_props.insert(
                "windowControlsLinux.text".into(),
                json!({"$ref":"#/$defs/ColorType"}),
            );
            color_props.insert(
                "windowControlsLinux.hoverBackground".into(),
                json!({"$ref":"#/$defs/ColorType"}),
            );
            color_props.insert(
                "windowControlsLinux.activeBackground".into(),
                json!({"$ref":"#/$defs/ColorType"}),
            );

            color_obj.insert("properties".into(), Value::Object(color_props));
            color_obj.insert(
                "required".into(),
                json!([
                    "primary",
                    "sideBar.background",
                    "toolBar.background",
                    "page.background",
                    "statusBar.background",
                    "windowsCloseButton.background",
                    "windowControlsLinux.background",
                    "windowControlsLinux.text",
                    "windowControlsLinux.hoverBackground",
                    "windowControlsLinux.activeBackground"
                ]),
            );

            defs.insert("Color".into(), Value::Object(color_obj));

            // ColorType
            let solid_schema = json!({
              "type":"object",
              "properties":{
                "type":{"type":"string","const":"solid"},
                "value":{"$ref":"#/$defs/ColorTokenValue"}
              },
              "required":["type","value"]
            });

            let gradient_schema = json!({
              "type":"object",
              "properties":{
                "type":{"type":"string","const":"gradient"},
                "direction":{"type":"string"},
                "value":{
                  "type":"array",
                  "items":{"$ref":"#/$defs/ColorGradientEntry"}
                }
              },
              "required":["type","direction","value"]
            });

            let color_type_obj = json!({
              "anyOf":[ solid_schema, gradient_schema ]
            });

            defs.insert("ColorType".into(), color_type_obj);

            // ColorTokenValue
            let color_token_value_schema = json!({
              "type":"string",
              "pattern":"#[0-9a-fA-F]{3,8}|rgb(a)?\\(.+\\)|hsl(a)?\\(.+\\)|[a-z]+"
            });
            defs.insert("ColorTokenValue".into(), color_token_value_schema);

            // ColorGradientEntry
            let cge_schema = json!({
              "type":"object",
              "properties": {
                "color": {"$ref":"#/$defs/ColorTokenValue"},
                "position": {
                  "anyOf": [
                    {"type":"number"},
                    {"type":"string"}
                  ]
                }
              },
              "required":["color","position"]
            });
            defs.insert("ColorGradientEntry".into(), cge_schema);

            root_obj.insert("$defs".into(), Value::Object(defs));
        }

        let schema_json = serde_json::to_string_pretty(&schema).unwrap();
        println!("{}", schema_json);
    }

    // schemars = "0.8.21"
    // use schemars::{
    //     schema::{
    //         ArrayValidation, InstanceType, Metadata, ObjectValidation, RootSchema, Schema,
    //         SchemaObject, SingleOrVec, StringValidation, SubschemaValidation,
    //     },
    //     Map, Set,
    // };
    // #[test]
    // fn test() {
    //     let mut root = RootSchema {
    //         meta_schema: Some("https://json-schema.org/draft/2020-12/schema".to_string()),
    //         schema: SchemaObject {
    //             metadata: Some(Box::new(Metadata {
    //                 id: Some("Theme.json".to_string()),
    //                 ..Default::default()
    //             })),
    //             instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Object))),
    //             ..Default::default()
    //         },
    //         definitions: Map::new(),
    //     };

    //     let mut root_properties = Map::new();
    //     root_properties.insert(
    //         "name".to_string(),
    //         Schema::Object(SchemaObject {
    //             instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
    //             ..Default::default()
    //         }),
    //     );
    //     root_properties.insert(
    //         "slug".to_string(),
    //         Schema::Object(SchemaObject {
    //             instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
    //             ..Default::default()
    //         }),
    //     );
    //     root_properties.insert(
    //         "type".to_string(),
    //         Schema::Object(SchemaObject {
    //             instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
    //             ..Default::default()
    //         }),
    //     );
    //     root_properties.insert(
    //         "isDefault".to_string(),
    //         Schema::Object(SchemaObject {
    //             instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Boolean))),
    //             ..Default::default()
    //         }),
    //     );

    //     let mut color_ref = SchemaObject::default();
    //     color_ref.reference = Some("#/$defs/Color".to_string());
    //     root_properties.insert("color".to_string(), Schema::Object(color_ref));

    //     let required_fields: Set<String> = ["name", "slug", "type", "isDefault", "color"]
    //         .iter()
    //         .map(|s| s.to_string())
    //         .collect();

    //     root.schema.object = Some(Box::new(ObjectValidation {
    //         properties: root_properties,
    //         required: required_fields,
    //         ..Default::default()
    //     }));

    //     // defs

    //     let color_token_value_schema = Schema::Object(SchemaObject {
    //         instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
    //         string: Some(Box::new(StringValidation {
    //             pattern: Some(
    //                 "#[0-9a-fA-F]{3,8}|rgb(a)?\\(.+\\)|hsl(a)?\\(.+\\)|[a-z]+".to_string(),
    //             ),
    //             ..Default::default()
    //         })),
    //         ..Default::default()
    //     });

    //     let mut cge_properties = Map::new();
    //     let mut cge_color_ref = SchemaObject::default();
    //     cge_color_ref.reference = Some("#/$defs/ColorTokenValue".to_string());
    //     cge_properties.insert("color".to_string(), Schema::Object(cge_color_ref));

    //     let position_any_of = Schema::Object(SchemaObject {
    //         subschemas: Some(Box::new(SubschemaValidation {
    //             any_of: Some(vec![
    //                 Schema::Object(SchemaObject {
    //                     instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Number))),
    //                     ..Default::default()
    //                 }),
    //                 Schema::Object(SchemaObject {
    //                     instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
    //                     ..Default::default()
    //                 }),
    //             ]),
    //             ..Default::default()
    //         })),
    //         ..Default::default()
    //     });
    //     cge_properties.insert("position".to_string(), position_any_of);

    //     let color_gradient_entry_schema = Schema::Object(SchemaObject {
    //         instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Object))),
    //         object: Some(Box::new(ObjectValidation {
    //             properties: cge_properties,
    //             required: ["color".to_string(), "position".to_string()]
    //                 .iter()
    //                 .cloned()
    //                 .collect(),
    //             ..Default::default()
    //         })),
    //         ..Default::default()
    //     });

    //     let mut solid_properties = Map::new();
    //     solid_properties.insert(
    //         "type".to_string(),
    //         Schema::Object(SchemaObject {
    //             instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
    //             const_value: Some(json!("solid")),
    //             ..Default::default()
    //         }),
    //     );
    //     let mut solid_value_ref = SchemaObject::default();
    //     solid_value_ref.reference = Some("#/$defs/ColorTokenValue".to_string());
    //     solid_properties.insert("value".to_string(), Schema::Object(solid_value_ref));

    //     let solid_schema = Schema::Object(SchemaObject {
    //         instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Object))),
    //         object: Some(Box::new(ObjectValidation {
    //             properties: solid_properties,
    //             required: ["type".to_string(), "value".to_string()]
    //                 .iter()
    //                 .cloned()
    //                 .collect(),
    //             ..Default::default()
    //         })),
    //         ..Default::default()
    //     });

    //     let mut gradient_properties = Map::new();
    //     gradient_properties.insert(
    //         "type".to_string(),
    //         Schema::Object(SchemaObject {
    //             instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
    //             const_value: Some(json!("gradient")),
    //             ..Default::default()
    //         }),
    //     );
    //     gradient_properties.insert(
    //         "direction".to_string(),
    //         Schema::Object(SchemaObject {
    //             instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
    //             ..Default::default()
    //         }),
    //     );
    //     gradient_properties.insert(
    //         "value".to_string(),
    //         Schema::Object(SchemaObject {
    //             instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Array))),
    //             array: Some(Box::new(ArrayValidation {
    //                 items: Some(SingleOrVec::Single(Box::new(Schema::Object(
    //                     SchemaObject {
    //                         reference: Some("#/$defs/ColorGradientEntry".to_string()),
    //                         ..Default::default()
    //                     },
    //                 )))),
    //                 ..Default::default()
    //             })),
    //             ..Default::default()
    //         }),
    //     );

    //     let gradient_schema = Schema::Object(SchemaObject {
    //         instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Object))),
    //         object: Some(Box::new(ObjectValidation {
    //             properties: gradient_properties,
    //             required: [
    //                 "type".to_string(),
    //                 "direction".to_string(),
    //                 "value".to_string(),
    //             ]
    //             .iter()
    //             .cloned()
    //             .collect(),
    //             ..Default::default()
    //         })),
    //         ..Default::default()
    //     });

    //     let color_type_schema = Schema::Object(SchemaObject {
    //         subschemas: Some(Box::new(SubschemaValidation {
    //             any_of: Some(vec![solid_schema, gradient_schema]),
    //             ..Default::default()
    //         })),
    //         ..Default::default()
    //     });

    //     let color_tokens = [
    //         "primary",
    //         "sideBar.background",
    //         "toolBar.background",
    //         "page.background",
    //         "statusBar.background",
    //         "windowsCloseButton.background",
    //         "windowControlsLinux.background",
    //         "windowControlsLinux.text",
    //         "windowControlsLinux.hoverBackground",
    //         "windowControlsLinux.activeBackground",
    //     ];
    //     let mut color_properties = Map::new();
    //     for token in &color_tokens {
    //         let mut ref_obj = SchemaObject::default();
    //         ref_obj.reference = Some("#/$defs/ColorType".to_string());
    //         color_properties.insert(token.to_string(), Schema::Object(ref_obj));
    //     }

    //     let color_schema = Schema::Object(SchemaObject {
    //         instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Object))),
    //         object: Some(Box::new(ObjectValidation {
    //             properties: color_properties,
    //             required: color_tokens.iter().map(|s| s.to_string()).collect(),
    //             ..Default::default()
    //         })),
    //         ..Default::default()
    //     });

    //     root.definitions.insert("Color".to_string(), color_schema);
    //     root.definitions
    //         .insert("ColorType".to_string(), color_type_schema);
    //     root.definitions
    //         .insert("ColorTokenValue".to_string(), color_token_value_schema);
    //     root.definitions.insert(
    //         "ColorGradientEntry".to_string(),
    //         color_gradient_entry_schema,
    //     );

    //     let schema_json = serde_json::to_string_pretty(&root).unwrap();
    //     println!("{}", schema_json);
    // }
}
