use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, Lit, Member};

#[proc_macro]
pub fn rule(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);
    let rule = match parse_expr_to_rule(&expr) {
        Ok(tokens) => tokens,
        Err(e) => return TokenStream::from(e.to_compile_error()),
    };
    TokenStream::from(quote! { #rule })
}

// FIXME: In case of a validation error, the macro should not cause a panic.
// This needs to be fixed, or the macro should be removed.
#[proc_macro]
pub fn rule_with_validation(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);
    let rule_with_validation = match parse_expr_to_rule_with_validation(&expr) {
        Ok(tokens) => tokens,
        Err(e) => return TokenStream::from(e.to_compile_error()),
    };
    TokenStream::from(quote! { #rule_with_validation })
}

fn parse_expr_to_rule(expr: &Expr) -> syn::Result<proc_macro2::TokenStream> {
    match expr {
        // Handle binary operations (e.g., a + b, x == y)
        Expr::Binary(expr_bin) => {
            let left = parse_expr_to_rule(&expr_bin.left)?;
            let right = parse_expr_to_rule(&expr_bin.right)?;
            let op = &expr_bin.op;

            let tokens = match op {
                syn::BinOp::Add(_) => quote! { #left.add(#right) },
                syn::BinOp::Sub(_) => quote! { #left.subtract(#right) },
                syn::BinOp::Mul(_) => quote! { #left.multiply(#right) },
                syn::BinOp::Div(_) => quote! { #left.divide(#right) },
                syn::BinOp::Rem(_) => quote! { #left.modulo(#right) },
                syn::BinOp::Eq(_) => quote! { #left.eq(#right) },
                syn::BinOp::Ne(_) => quote! { #left.ne(#right) },
                syn::BinOp::Gt(_) => quote! { #left.gt(#right) },
                syn::BinOp::Lt(_) => quote! { #left.lt(#right) },
                syn::BinOp::Ge(_) => quote! { #left.gte(#right) },
                syn::BinOp::Le(_) => quote! { #left.lte(#right) },
                syn::BinOp::And(_) => quote! { #left.and(#right) },
                syn::BinOp::Or(_) => quote! { #left.or(#right) },
                _ => {
                    return Err(syn::Error::new_spanned(
                        op,
                        "Unsupported binary operator in rule macro",
                    ))
                }
            };
            Ok(tokens)
        }
        // Handle unary operations (e.g., !a)
        Expr::Unary(expr_unary) => {
            let operand = parse_expr_to_rule(&expr_unary.expr)?;
            let op = &expr_unary.op;
            let tokens = match op {
                syn::UnOp::Not(_) => quote! { #operand.not() },
                _ => {
                    return Err(syn::Error::new_spanned(
                        op,
                        "Unsupported unary operator in rule macro",
                    ))
                }
            };
            Ok(tokens)
        }
        // Handle expressions in parentheses
        Expr::Paren(expr_paren) => parse_expr_to_rule(&expr_paren.expr),
        // Handle literals (e.g., numbers, strings, booleans)
        Expr::Lit(expr_lit) => {
            let lit = &expr_lit.lit;
            match lit {
                Lit::Int(_) | Lit::Float(_) | Lit::Str(_) | Lit::Bool(_) => {
                    Ok(quote! { RawRule::value(#lit) })
                }
                _ => Err(syn::Error::new_spanned(
                    lit,
                    "Unsupported literal type in rule macro",
                )),
            }
        }
        // Handle the val!() macro to inject external variables
        Expr::Macro(expr_macro) => {
            if let Some(ident) = expr_macro.mac.path.get_ident() {
                if ident == "val" {
                    let tokens = &expr_macro.mac.tokens;
                    // Remove parentheses from tokens
                    let tokens_string = tokens.to_string();
                    let tokens_trimmed = tokens_string.trim_matches(|c| c == '(' || c == ')');
                    let tokens: proc_macro2::TokenStream = tokens_trimmed.parse().unwrap();
                    Ok(quote! { RawRule::from(#tokens) })
                } else {
                    Err(syn::Error::new_spanned(
                        expr_macro,
                        "Unsupported macro in rule macro",
                    ))
                }
            } else {
                Err(syn::Error::new_spanned(
                    expr_macro,
                    "Expected identifier in macro",
                ))
            }
        }
        // Handle variable references (e.g., age, status)
        Expr::Path(expr_path) => {
            let ident = expr_path
                .path
                .get_ident()
                .ok_or_else(|| syn::Error::new_spanned(expr_path, "Expected identifier"))?;
            let name = ident.to_string();
            Ok(quote! { RawRule::var(#name) })
        }
        // Handle field access (e.g., user.name)
        Expr::Field(expr_field) => {
            let base = parse_expr_to_string(&expr_field.base)?;
            let member = match &expr_field.member {
                Member::Named(ident) => ident.to_string(),
                Member::Unnamed(index) => index.index.to_string(),
            };
            let full_name = format!("{}.{}", base, member);
            Ok(quote! { RawRule::var(#full_name) })
        }
        // Handle indexing (e.g., array[0])
        Expr::Index(expr_index) => {
            let base = parse_expr_to_string(&expr_index.expr)?;
            let index = parse_expr_to_string(&expr_index.index)?;
            let full_name = format!("{}[{}]", base, index);
            Ok(quote! { RawRule::var(#full_name) })
        }
        // Handle method calls (e.g., obj.method(arg))
        Expr::MethodCall(expr_method_call) => {
            let receiver = parse_expr_to_string(&expr_method_call.receiver)?;
            let method = expr_method_call.method.to_string();
            let args: Vec<_> = expr_method_call
                .args
                .iter()
                .map(parse_expr_to_rule)
                .collect::<Result<_, _>>()?;
            let method_call = format!("{}.{}", receiver, method);
            Ok(quote! { RawRule::custom(#method_call, vec![#(#args),*]) })
        }
        // Return an error for unsupported expressions
        _ => Err(syn::Error::new_spanned(
            expr,
            "Unsupported expression in rule macro",
        )),
    }
}

fn parse_expr_to_rule_with_validation(expr: &Expr) -> syn::Result<proc_macro2::TokenStream> {
    match expr {
        // Handle binary operations with validation
        Expr::Binary(expr_bin) => {
            let left = parse_expr_to_rule_with_validation(&expr_bin.left)?;
            let right = parse_expr_to_rule_with_validation(&expr_bin.right)?;
            let op = &expr_bin.op;

            let tokens = match op {
                syn::BinOp::Add(_) => {
                    quote! { #left.add(#right).map_err(|e| e.to_string()).unwrap() }
                }
                syn::BinOp::Sub(_) => {
                    quote! { #left.subtract(#right).map_err(|e| e.to_string()).unwrap() }
                }
                syn::BinOp::Mul(_) => {
                    quote! { #left.multiply(#right).map_err(|e| e.to_string()).unwrap() }
                }
                syn::BinOp::Div(_) => {
                    quote! { #left.divide(#right).map_err(|e| e.to_string()).unwrap() }
                }
                syn::BinOp::Rem(_) => {
                    quote! { #left.modulo(#right).map_err(|e| e.to_string()).unwrap() }
                }
                syn::BinOp::Eq(_) => {
                    quote! { #left.eq(#right).map_err(|e| e.to_string()).unwrap() }
                }
                syn::BinOp::Ne(_) => {
                    quote! { #left.ne(#right).map_err(|e| e.to_string()).unwrap() }
                }
                syn::BinOp::Gt(_) => {
                    quote! { #left.gt(#right).map_err(|e| e.to_string()).unwrap() }
                }
                syn::BinOp::Lt(_) => {
                    quote! { #left.lt(#right).map_err(|e| e.to_string()).unwrap() }
                }
                syn::BinOp::Ge(_) => {
                    quote! { #left.gte(#right).map_err(|e| e.to_string()).unwrap() }
                }
                syn::BinOp::Le(_) => {
                    quote! { #left.lte(#right).map_err(|e| e.to_string()).unwrap() }
                }
                syn::BinOp::And(_) => {
                    quote! { #left.and(#right).map_err(|e| e.to_string()).unwrap() }
                }
                syn::BinOp::Or(_) => {
                    quote! { #left.or(#right).map_err(|e| e.to_string()).unwrap() }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        op,
                        "Unsupported binary operator in rule macro",
                    ))
                }
            };
            Ok(tokens)
        }
        // Handle unary operations with validation
        Expr::Unary(expr_unary) => {
            let operand = parse_expr_to_rule_with_validation(&expr_unary.expr)?;
            let op = &expr_unary.op;
            let tokens = match op {
                syn::UnOp::Not(_) => quote! { #operand.not().map_err(|e| e.to_string()).unwrap() },
                _ => {
                    return Err(syn::Error::new_spanned(
                        op,
                        "Unsupported unary operator in rule macro",
                    ))
                }
            };
            Ok(tokens)
        }
        // Handle expressions in parentheses
        Expr::Paren(expr_paren) => parse_expr_to_rule_with_validation(&expr_paren.expr),
        // Handle literals
        Expr::Lit(expr_lit) => {
            let lit = &expr_lit.lit;
            match lit {
                Lit::Int(_) | Lit::Float(_) | Lit::Str(_) | Lit::Bool(_) => {
                    Ok(quote! { RuleWithValidation::value(#lit) })
                }
                _ => Err(syn::Error::new_spanned(
                    lit,
                    "Unsupported literal type in rule macro",
                )),
            }
        }
        // Handle the val!() macro to inject external variables
        Expr::Macro(expr_macro) => {
            if let Some(ident) = expr_macro.mac.path.get_ident() {
                if ident == "val" {
                    let tokens = &expr_macro.mac.tokens;
                    // Remove parentheses from tokens
                    let tokens_string = tokens.to_string();
                    let tokens_trimmed = tokens_string.trim_matches(|c| c == '(' || c == ')');
                    let tokens: proc_macro2::TokenStream = tokens_trimmed.parse().unwrap();
                    Ok(quote! { RuleWithValidation::from(#tokens) })
                } else {
                    Err(syn::Error::new_spanned(
                        expr_macro,
                        "Unsupported macro in rule macro",
                    ))
                }
            } else {
                Err(syn::Error::new_spanned(
                    expr_macro,
                    "Expected identifier in macro",
                ))
            }
        }
        // Handle variable references
        Expr::Path(expr_path) => {
            let ident = expr_path
                .path
                .get_ident()
                .ok_or_else(|| syn::Error::new_spanned(expr_path, "Expected identifier"))?;
            let name = ident.to_string();
            Ok(quote! { RuleWithValidation::var(#name) })
        }
        // Handle field access
        Expr::Field(expr_field) => {
            let base = parse_expr_to_string(&expr_field.base)?;
            let member = match &expr_field.member {
                Member::Named(ident) => ident.to_string(),
                Member::Unnamed(index) => index.index.to_string(),
            };
            let full_name = format!("{}.{}", base, member);
            Ok(quote! { RuleWithValidation::var(#full_name) })
        }
        // Handle indexing
        Expr::Index(expr_index) => {
            let base = parse_expr_to_string(&expr_index.expr)?;
            let index = parse_expr_to_string(&expr_index.index)?;
            let full_name = format!("{}[{}]", base, index);
            Ok(quote! { RuleWithValidation::var(#full_name) })
        }
        // Handle method calls
        Expr::MethodCall(expr_method_call) => {
            let receiver = parse_expr_to_string(&expr_method_call.receiver)?;
            let method = expr_method_call.method.to_string();
            let args: Vec<_> = expr_method_call
                .args
                .iter()
                .map(parse_expr_to_rule_with_validation)
                .collect::<Result<_, _>>()?;
            let method_call = format!("{}.{}", receiver, method);
            Ok(quote! { RuleWithValidation::custom(#method_call, vec![#(#args),*]) })
        }
        // Return an error for unsupported expressions
        _ => Err(syn::Error::new_spanned(
            expr,
            "Unsupported expression in rule macro",
        )),
    }
}

fn parse_expr_to_string(expr: &Expr) -> syn::Result<String> {
    match expr {
        // Handle variable references
        Expr::Path(expr_path) => {
            let ident = expr_path
                .path
                .get_ident()
                .ok_or_else(|| syn::Error::new_spanned(expr_path, "Expected identifier"))?;
            Ok(ident.to_string())
        }
        // Handle field access
        Expr::Field(expr_field) => {
            let base = parse_expr_to_string(&expr_field.base)?;
            let member = match &expr_field.member {
                Member::Named(ident) => ident.to_string(),
                Member::Unnamed(index) => index.index.to_string(),
            };
            Ok(format!("{}.{}", base, member))
        }
        // Handle indexing
        Expr::Index(expr_index) => {
            let base = parse_expr_to_string(&expr_index.expr)?;
            let index = parse_expr_to_string(&expr_index.index)?;
            Ok(format!("{}[{}]", base, index))
        }
        // Return an error for unsupported expressions
        _ => Err(syn::Error::new_spanned(
            expr,
            "Expected variable name in field access",
        )),
    }
}
