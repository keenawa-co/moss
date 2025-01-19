use anyhow::{anyhow, Result};
use hashbrown::HashMap;
use hcl::{
    eval::{Context, Evaluate},
    Expression, Map, Value as HclValue,
};

// #[derive(Debug, Deserialize, Serialize)]
// pub enum ParameterValueType {
//     #[serde(rename = "number")]
//     Number,
//     #[serde(rename = "string")]
//     String,
// }

// impl ParameterValueType {
//     pub fn default_json_value(&self) -> JsonValue {
//         match self {
//             ParameterValueType::Number => JsonValue::Number(Number::from(0)),
//             ParameterValueType::String => JsonValue::String(String::new()),
//         }
//     }
// }

// #[derive(Debug, Default, Deserialize, Serialize)]
// pub enum ParameterScope {
//     APPLICATION,
//     #[default]
//     WINDOW,
//     RESOURCE,
//     #[allow(non_camel_case_types)]
//     LANGUAGE_SPECIFIC,
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct ParameterDecl {
//     #[serde(rename = "type")]
//     pub typ: ParameterValueType,
//     #[serde(default)]
//     pub maximum: JsonValue,
//     #[serde(default)]
//     pub minimum: JsonValue,
//     #[serde(default)]
//     pub default: JsonValue,
//     /// The order in which the parameter appears within its group in the settings UI.
//     pub order: Option<usize>,
//     #[serde(default)]
//     pub scope: ParameterScope,
//     pub description: Option<String>,
//     /// Excluded parameters are hidden from the UI but can still be registered.
//     #[serde(default)]
//     pub excluded: bool,
//     /// Indicates if this setting is protected from addon overrides.
//     #[serde(default)]
//     pub protected: bool,
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct OverrideDecl {
//     pub value: JsonValue,
//     pub context: Option<HashSet<String>>,
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct ConfigurationDecl {
//     pub title: Option<String>,
//     pub description: Option<String>,

//     /// The order in which this group appears in the settings UI.
//     pub order: Option<usize>,
//     #[serde(rename = "parameter")]
//     pub parameters: HashMap<ArcStr, Arc<ParameterDecl>>,
//     #[serde(rename = "override")]
//     pub overrides: HashMap<ArcStr, Arc<OverrideDecl>>,
// }

#[derive(Debug)]
pub struct ParserLocalVarDecl {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug)]
pub struct ParsedConfigurationParameterDecl {
    pub ident: String,
    pub default: Expression,
}

#[derive(Debug)]
pub struct ParsedConfigurationDecl {
    pub ident: Option<String>,
    pub parent_ident: Option<String>,
    pub display_name: Option<Expression>,
    pub description: Option<Expression>,
    pub order: Option<Expression>,
    pub parameters: Vec<ParsedConfigurationParameterDecl>,
}

impl ParsedConfigurationDecl {
    pub fn evaluate(self, ctx: &Context) -> Result<ConfigurationNode> {
        let mut parameters = HashMap::new();
        for param in self.parameters {
            parameters.insert(
                param.ident.clone(),
                ParameterNode {
                    ident: param.ident,
                    default: param.default.evaluate(ctx)?,
                },
            );
        }

        Ok(ConfigurationNode {
            ident: self.ident,
            parent_ident: self.parent_ident,
            display_name: try_evaluate_option_string(ctx, self.display_name)?,
            description: try_evaluate_option_string(ctx, self.description)?,
            order: try_evaluate_option_u64(ctx, self.order)?,
            parameters,
        })
    }
}
fn try_evaluate_option_string(ctx: &Context, expr: Option<Expression>) -> Result<Option<String>> {
    try_evaluate_option(ctx, expr, |v| v.as_str().map(ToString::to_string))
}

fn try_evaluate_option_u64(ctx: &Context, expr: Option<Expression>) -> Result<Option<u64>> {
    try_evaluate_option(ctx, expr, |v| v.as_u64())
}

fn try_evaluate_option<T, F>(
    ctx: &Context,
    expr: Option<Expression>,
    extract: F,
) -> Result<Option<T>>
where
    F: Fn(HclValue) -> Option<T>,
{
    match expr {
        Some(e) => {
            let evaluated = e.evaluate(ctx)?;
            let value = extract(evaluated).ok_or_else(|| anyhow!("failed to extract value"))?;

            Ok(Some(value))
        }
        None => Ok(None),
    }
}

#[derive(Debug, Default)]
pub struct Scope {
    pub configurations: Vec<ParsedConfigurationDecl>,
    pub locals: Map<String, HclValue>,
}

impl Scope {
    pub fn evaluate(self, ctx: &mut Context) -> Result<ResolvedScope> {
        let mut package = ResolvedScope::new();
        ctx.declare_var("local", hcl::Value::Object(self.locals));

        for decl in self.configurations {
            package.configurations.push(decl.evaluate(ctx)?);
        }

        Ok(package)
    }
}

#[derive(Debug)]
pub struct ConfigurationNode {
    pub ident: Option<String>,
    pub parent_ident: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub order: Option<u64>,
    pub parameters: HashMap<String, ParameterNode>,
}

#[derive(Debug)]
pub struct ParameterNode {
    pub ident: String,
    pub default: HclValue,
}

#[derive(Debug)]
pub struct ResolvedScope {
    pub configurations: Vec<ConfigurationNode>,
}

impl ResolvedScope {
    pub fn new() -> Self {
        Self {
            configurations: Vec::new(),
        }
    }
}
