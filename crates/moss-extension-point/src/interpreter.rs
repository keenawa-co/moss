pub mod types;

use anyhow::Result;
use arcstr::ArcStr;
use hashbrown::HashMap;
use hcl::{
    eval::{Context, Evaluate},
    Expression, Map, Value as HclValue,
};
use std::str::FromStr;
use std::sync::Arc;

use types::configuration::*;
use types::scope::ResolvedScope;

#[inline]
fn is_null_expression(expr: &Expression) -> bool {
    match expr {
        Expression::Null => true,
        _ => false,
    }
}

#[derive(Debug)]
pub struct ConfigurationParameterDecl {
    pub ident: ArcStr,
    pub value_type: Expression,
    pub maximum: Expression,
    pub minimum: Expression,
    pub default: Expression,
    pub order: Expression,
    pub scope: Expression,
    pub description: Expression,
    pub excluded: Expression,
    pub protected: Expression,
}

#[derive(Debug)]
pub struct ConfigurationOverrideDecl {
    pub ident: ArcStr,
    pub value: Expression,
    pub context: Expression,
}

#[derive(Debug)]
pub struct ConfigurationDecl {
    pub ident: Option<ArcStr>,
    pub parent_ident: Option<ArcStr>,
    pub display_name: Expression,
    pub description: Expression,
    pub order: Expression,
    pub parameters: Vec<ConfigurationParameterDecl>,
    pub overrides: Vec<ConfigurationOverrideDecl>,
}

impl ConfigurationDecl {
    pub fn evaluate(self, ctx: &Context) -> Result<ConfigurationNode> {
        let mut parameters = HashMap::new();
        for parameter_decl in self.parameters {
            let typ = match parameter_decl.value_type {
                Expression::Array(_vec) => unimplemented!(),
                Expression::Object(_vec_map) => unimplemented!(),
                Expression::Variable(variable) => ParameterType::try_from(variable)?,
                _ => {
                    // TODO: Add logging for encountering an unknown type
                    continue;
                }
            };

            parameters.insert(
                parameter_decl.ident.clone(),
                Arc::new(Parameter {
                    ident: parameter_decl.ident,
                    typ,
                    maximum: try_evaluate_to_u64(ctx, parameter_decl.maximum)?,
                    minimum: try_evaluate_to_u64(ctx, parameter_decl.minimum)?,
                    default: serde_json::from_str(
                        parameter_decl.default.evaluate(ctx)?.to_string().as_str(),
                    )?,
                    scope: try_evaluate_to_string(ctx, parameter_decl.scope)?
                        .and_then(|value| ParameterScope::from_str(&value).ok())
                        .unwrap_or_default(),
                    order: try_evaluate_to_u64(ctx, parameter_decl.order)?,
                    description: try_evaluate_to_string(ctx, parameter_decl.description)?,
                    excluded: try_evaluate_to_bool(ctx, parameter_decl.excluded)?.unwrap_or(false),
                    protected: try_evaluate_to_bool(ctx, parameter_decl.protected)?
                        .unwrap_or(false),
                }),
            );
        }

        let mut overrides = HashMap::new();
        for override_decl in self.overrides {
            let value = if is_null_expression(&override_decl.value) {
                // TODO: Add logging
                continue;
            } else {
                serde_json::from_str(override_decl.value.evaluate(ctx)?.to_string().as_str())?
            };

            let _context = if !is_null_expression(&override_decl.context) {
                unimplemented!()
            };

            overrides.insert(
                override_decl.ident.clone(),
                Arc::new(Override {
                    ident: override_decl.ident,
                    value,
                    context: None,
                }),
            );
        }

        Ok(ConfigurationNode {
            ident: self.ident.unwrap_or_default(),
            parent_ident: self.parent_ident,
            display_name: try_evaluate_to_string(ctx, self.display_name)?,
            description: try_evaluate_to_string(ctx, self.description)?,
            order: try_evaluate_to_u64(ctx, self.order)?,
            parameters,
            overrides,
        })
    }
}

fn try_evaluate_to_string(ctx: &Context, expr: Expression) -> Result<Option<String>> {
    Ok(expr.evaluate(ctx)?.as_str().map(ToString::to_string))
}

fn try_evaluate_to_u64(ctx: &Context, expr: Expression) -> Result<Option<u64>> {
    Ok(expr.evaluate(ctx)?.as_u64())
}

fn try_evaluate_to_bool(ctx: &Context, expr: Expression) -> Result<Option<bool>> {
    Ok(expr.evaluate(ctx)?.as_bool())
}

#[derive(Debug, Default)]
pub struct ParsedScope {
    pub configurations: Vec<ConfigurationDecl>,
    pub configuration_extends: Vec<ConfigurationDecl>,
    pub locals: Map<String, HclValue>,
}

impl ParsedScope {
    pub fn evaluate_with_context(self, ctx: &Context) -> Result<ResolvedScope> {
        let mut ctx = ctx.clone();
        let mut package = ResolvedScope::new();
        ctx.declare_var("local", hcl::Value::Object(self.locals));

        for decl in self.configurations {
            package.insert_configuration(decl.evaluate(&ctx)?);
        }

        Ok(package)
    }

    pub fn evaluate(self) -> Result<ResolvedScope> {
        self.evaluate_with_context(&Context::default())
    }
}
