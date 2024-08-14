use platform_configuration_common::{
    configuration_registry::{
        ConfigurationNode, ConfigurationNodeType as Type, ConfigurationPropertySchema as Schema,
        PropertyMap,
    },
    property_key,
};

lazy_static! {
    pub static ref WORKBENCH_TGUI_WINDOW: ConfigurationNode = ConfigurationNode {
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
                Schema {
                    typ: Some(Type::Number),
                    default: Some(serde_json::Value::Bool(true)),
                    description: Some("Determines whether the window should be restored in full-screen mode on the next launch".to_string()),
                    ..Default::default()
                },
            );

            Some(properties)
        },
        parent_of: None,
    };
}
