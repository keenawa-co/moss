pub mod rule;

// FIXME: The module cannot be public due to issues with the macro for creating validation
// rules. The macro needs to be fixed first, and only then can the module be made public.
mod rule_with_validation;
