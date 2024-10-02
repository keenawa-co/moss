use platform_configuration::{
    configuration_registry::{
        ConfigurationNode, ConfigurationNodeType as Type,
        ConfigurationPropertySchema as PropertySchema, PropertyMap,
    },
    property_key,
};

lazy_static! {
    pub static ref WORKBENCH_TAO_WINDOW: ConfigurationNode = ConfigurationNode {
        id: "window".to_string(),
        title: Some("Window".to_string()),
        description: None,
        order: None,
        typ: Default::default(),
        scope: Default::default(),
        source: None,
        properties: {
            let mut properties = PropertyMap::new();

            properties.insert(
                property_key!(window.restoreFullScreen),
                PropertySchema {
                    typ: Some(Type::Number),
                    default: Some(serde_json::Value::Bool(true)),
                    description: Some("Determines whether the window should be restored in full-screen mode on the next launch".to_string()),
                    ..Default::default()
                },
            );
            properties.insert(
                property_key!(window.restoreTab),
                PropertySchema {
                    typ: Some(Type::Number),
                    default: Some(serde_json::Value::Bool(true)),
                    description: Some("Determines whether the window should restore the last opened tab on the next launch".to_string()),
                    ..Default::default()
                },
            );
            properties.insert(
                property_key!(window.defaultWidth),
                PropertySchema {
                    typ: Some(Type::Number),
                    default: Some(serde_json::Value::Number(serde_json::Number::from(1400))),
                    description: Some("The default window width in logical pixels".to_string()),
                    ..Default::default()
                },
            );
            properties.insert(
                property_key!(window.defaultHeight),
                PropertySchema {
                    typ: Some(Type::Number),
                    protected_from_contribution: true,
                    default: Some(serde_json::Value::Number(serde_json::Number::from(750))),
                    description: Some("The default window height in logical pixels".to_string()),
                    ..Default::default()
                },
            );
            properties.insert(
                property_key!(window.minWidth),
                PropertySchema {
                    typ: Some(Type::Number),
                    protected_from_contribution: true,
                    included: false,
                    default: Some(serde_json::Value::Number(serde_json::Number::from(800))),
                    description: Some(
                        "The minimal allowable window width in logical pixels".to_string(),
                    ),
                    ..Default::default()
                },
            );
            properties.insert(
                property_key!(window.minHeight),
                PropertySchema {
                    typ: Some(Type::Number),
                    protected_from_contribution: true,
                    included: false,
                    default: Some(serde_json::Value::Number(serde_json::Number::from(500))),
                    description: Some(
                        "The minimal allowable window height in logical pixels".to_string(),
                    ),
                    ..Default::default()
                },
            );

            Some(properties)
        },
        parent_of: None,
    };
}
