use anyhow::{anyhow, Result};
use hashbrown::HashMap;
use hcl::eval::{Context, Evaluate};
use hcl::{Expression, Map, Value};

pub fn evaluate_locals(locals: HashMap<String, Expression>) -> Result<Map<String, Value>> {
    // OPTIMIZE: maybe using Acyclic would be better?
    let mut graph = petgraph::Graph::<String, ()>::new();
    let mut node_map = HashMap::new();
    let mut name_map = HashMap::new();
    for local_name in locals.keys() {
        let idx = graph.add_node(local_name.clone());
        node_map.insert(local_name.clone(), idx);
        name_map.insert(idx, local_name.clone());
    }

    for (name, expr) in locals.iter() {
        // TODO: Should we raise an error at this step when encountering unresolvable reference?
        let from_idx = node_map[name];
        let deps = crate::parse::collect_local_refs(&expr);

        for dep in deps {
            if let Some(&to_idx) = node_map.get(&dep) {
                graph.add_edge(from_idx, to_idx, ());
            } else {
                return Err(anyhow!("Cannot resolve symbol `{}`", dep));
            }
        }
    }

    let dependency_chain = petgraph::algo::toposort(&graph, None)
        .map_err(|_| anyhow!("Cycle detected in locals"))?
        .iter()
        .map(|idx| name_map.get(idx).unwrap().to_string())
        .rev()
        .collect::<Vec<String>>();

    let mut evaluated = Map::<String, Value>::new();

    for name in dependency_chain.iter() {
        let expr = locals.get(name).unwrap();
        let mut ctx = Context::new();
        ctx.declare_var("local", Value::Object(evaluated.clone()));
        let value = expr.evaluate(&ctx)?;
        evaluated.insert(name.to_string(), value);
    }

    Ok(evaluated)
}
