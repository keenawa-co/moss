//! One way to the contents of an entity, as you might do for debugging. A similar pattern could
//! also be useful for serialization, or other row-oriented generic operations.

type FormattingFunction = &'static dyn Fn(crater::EntityRef<'_>) -> Option<String>;

fn format_entity(entity: crater::EntityRef<'_>) -> String {
    fn fmt<T: crater::Component + std::fmt::Display>(entity: crater::EntityRef<'_>) -> Option<String> {
        Some(entity.get::<&T>()?.to_string())
    }

    const FUNCTIONS: &[FormattingFunction] = &[&fmt::<i32>, &fmt::<bool>, &fmt::<f64>];

    let mut out = String::new();
    for f in FUNCTIONS {
        if let Some(x) = f(entity) {
            if out.is_empty() {
                out.push('[');
            } else {
                out.push_str(", ");
            }
            out.push_str(&x);
        }
    }
    if out.is_empty() {
        out.push_str("[]");
    } else {
        out.push(']');
    }
    out
}

fn main() {
    let mut world = crater::Frame::new();
    let e = world.spawn((42, true));
    println!("{}", format_entity(world.entity(e).unwrap()));
}
