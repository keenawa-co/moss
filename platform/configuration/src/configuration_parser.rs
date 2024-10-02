use anyhow::Result;
use hashbrown::HashMap;
use lazy_regex::{Lazy, Regex};
use moss_std::collection::extend::Extend;
use platform_core::context_v2::{atom::Atom, Context};
use radix_trie::{Trie, TrieCommon};
use serde_json::Value;

use crate::{
    configuration_model::{AttributeName, ConfigurationModel},
    configuration_registry::ConfigurationRegistry,
};

static OVERRIDE_PROPERTY_REGEX: &'static Lazy<Regex> = regex!(r"^(\[.*\])+$");

pub struct ConfigurationParser {
    registry: Atom<ConfigurationRegistry>,
}

struct ConfigurationOverride {
    ident: String,
    attribute_names: Vec<String>,
    content: Trie<String, serde_json::Value>,
}

impl ConfigurationParser {
    pub fn new(registry: Atom<ConfigurationRegistry>) -> Self {
        Self { registry }
    }

    pub fn parse(&self, ctx: &mut Context, content: &str) -> Result<ConfigurationModel> {
        let raw_content: HashMap<String, Value> = serde_json::from_str(content)?;
        let mut model = ConfigurationModel::empty();

        for (attribute_name, value) in &raw_content {
            if OVERRIDE_PROPERTY_REGEX.is_match(attribute_name) {
                if let Some(override_definition) = self.process_override(ctx, attribute_name, value)
                {
                    model.overrides.push(override_definition.ident);
                    model.content.extend(override_definition.content.iter());
                }

                continue;
            }

            if self.inspect_attribute(ctx, attribute_name) {
                model.set_value(AttributeName::format(attribute_name), value.clone());
            }
        }

        Ok(model)
    }

    // TODO: return diags
    // TODO: use logs, not println
    fn inspect_attribute(&self, ctx: &mut Context, attribute_name: &str) -> bool {
        let configuration_properties = self.registry.read(ctx).properties();

        match configuration_properties.get(attribute_name) {
            Some(registered_property) => {
                if registered_property.is_protected_from_contribution() {
                    println!(
                        "Property `{}` is protected from contribution",
                        attribute_name
                    );
                    return false;
                }

                true
            }
            None => {
                println!("Unknown property `{}` was detected", attribute_name);
                return false;
            }
        }
    }

    fn process_override(
        &self,
        ctx: &mut Context,
        attribute_name: &str,
        value: &Value,
    ) -> Option<ConfigurationOverride> {
        let content = if let Value::Object(obj) = value {
            obj
        } else {
            // If the override is not an object, then we don't want to handle it in any way.
            return None;
        };

        let override_identifiers = self.registry.read(ctx).override_identifiers();
        let formatted_identifier = attribute_name.trim_matches(|c| c == '[' || c == ']');

        if override_identifiers.get(formatted_identifier).is_none() {
            println!(
                "Unknown override identifier `{}` was detected",
                formatted_identifier
            );
            return None;
        }

        let mut result = ConfigurationOverride {
            ident: formatted_identifier.to_string(),
            attribute_names: Vec::new(),
            content: Trie::new(),
        };

        for (attribute_name, value) in content {
            if self.inspect_attribute(ctx, attribute_name) {
                // let formatted_key = format!("$.[{}].{}", formatted_identifier, attribute_name);
                let formatted_key =
                    AttributeName::format_with_override(attribute_name, formatted_identifier);
                result.content.insert(formatted_key.clone(), value.clone());
                result.attribute_names.push(formatted_key);
            }
        }

        Some(result)
    }
}
