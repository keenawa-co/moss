use hashbrown::HashMap;
use serde_json::Value;

use super::configuration_model::ConfigurationLayer;

pub struct DefaultConfiguration(pub ConfigurationLayer);

impl DefaultConfiguration {
    pub fn new() -> Self {
        let contents: HashMap<String, Value> = vec![
            (
                "project.monitoring.exclude".to_string(),
                Value::Array(vec![]),
            ),
            ("editor.fontSize".to_string(), Value::from(12)),
            ("window.restoreFullScreen".to_string(), Value::Bool(true)),
            ("window.restoreTab".to_string(), Value::Bool(true)),
        ]
        .into_iter()
        .collect();

        let overrides: HashMap<String, HashMap<String, Value>> = vec![(
            "[mossql]".to_string(),
            vec![("editor.fontSize".to_string(), Value::from(10))]
                .into_iter()
                .collect(),
        )]
        .into_iter()
        .collect();

        // Self(ConfigurationEntryModel {
        //     contents,
        //     keys: vec![
        //         "project.monitoring.exclude".to_string(),
        //         "window.restoreFullScreen".to_string(),
        //         "window.restoreTab".to_string(),
        //     ],
        //     overrides,
        // })

        Self(ConfigurationLayer::empty())
    }
}
