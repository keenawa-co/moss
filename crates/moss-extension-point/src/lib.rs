use anyhow::Result;
use hcl::{
    eval::{Context, Evaluate},
    Block,
};
use std::{collections::HashMap, path::PathBuf};

pub mod extends;

pub struct ExtensionPoint {}

pub struct Loader {
    points: HashMap<PathBuf, ExtensionPoint>,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            points: HashMap::new(),
        }
    }

    pub fn process(&self, input: &str) -> Result<()> {
        let mut ctx = hcl::eval::Context::new();

        let mut body: hcl::Body = hcl::from_str(input)?;
        for block in body.blocks_mut() {
            match block.identifier.as_str() {
                "locals" => self.process_locals(&mut ctx, block),
                _ => continue,
            }
        }

        Ok(())
    }

    fn process_locals(&self, ctx: &mut Context, block: &mut Block) {
        for attribute in block.body.attributes_mut() {
            dbg!(attribute.evaluate(ctx).unwrap());

            // ctx.declare_var(format!("local.{}", attribute.key.as_str()), attribute.expr().);
        }
    }
}

#[cfg(test)]
mod tests {
    use hcl::{edit::Ident, eval::Evaluate, Body, Identifier};

    use crate::Loader;

    #[test]
    fn test2() {
        let input = r#"
pragma required_version ">= 1.0" {}
pragma static {}

locals {
    max_subnet_length = 10
}

extends "configuration" {
    parameter "window.defaultWidth" {
        type = number
        minimum = 800
        maximum = 3840
        default = 800
        description = "The width of the application window in pixels."
    }
}
    "#;
        // let loader = Loader::new();
        // loader.process(input).unwrap();

        let mut ctx = hcl::eval::Context::new();

        // ctx.declare_var("name", "Test");
        ctx.declare_var("number", "number");
        let body: hcl::Body = hcl::from_str(input).unwrap();
        let r = body.evaluate(&ctx).unwrap();

        let json_value: serde_json::Value = hcl::from_body(r).unwrap();
        let pretty_json = serde_json::to_string_pretty(&json_value).unwrap();
        println!("{}", pretty_json);
    }
}
