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
        Expr::Binary(expr_bin) => {
            let left = parse_expr_to_rule(&expr_bin.left)?;
            let right = parse_expr_to_rule(&expr_bin.right)?;
            let op = &expr_bin.op;

            let tokens = match op {
                syn::BinOp::Add(_) => {
                    quote! { #left.add(#right) }
                }
                syn::BinOp::Sub(_) => {
                    quote! { #left.subtract(#right)}
                }
                syn::BinOp::Mul(_) => {
                    quote! { #left.multiply(#right)}
                }
                syn::BinOp::Div(_) => {
                    quote! { #left.divide(#right)}
                }
                syn::BinOp::Rem(_) => {
                    quote! { #left.modulo(#right)}
                }
                syn::BinOp::Eq(_) => quote! { #left.eq(#right)},
                syn::BinOp::Ne(_) => {
                    quote! { #left.ne(#right) }
                }
                syn::BinOp::Gt(_) => {
                    quote! { #left.gt(#right) }
                }
                syn::BinOp::Lt(_) => {
                    quote! { #left.lt(#right) }
                }
                syn::BinOp::Ge(_) => {
                    quote! { #left.gte(#right)}
                }
                syn::BinOp::Le(_) => {
                    quote! { #left.lte(#right) }
                }
                syn::BinOp::And(_) => {
                    quote! { #left.and(#right) }
                }
                syn::BinOp::Or(_) => {
                    quote! { #left.or(#right) }
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
        Expr::Paren(expr_paren) => parse_expr_to_rule(&expr_paren.expr),
        Expr::Lit(expr_lit) => {
            let lit = &expr_lit.lit;
            match lit {
                Lit::Int(_) | Lit::Float(_) | Lit::Str(_) | Lit::Bool(_) => {
                    Ok(quote! { Rule::value(#lit) })
                }
                _ => Err(syn::Error::new_spanned(
                    lit,
                    "Unsupported literal type in rule macro",
                )),
            }
        }
        Expr::Path(expr_path) => {
            let ident = expr_path
                .path
                .get_ident()
                .ok_or_else(|| syn::Error::new_spanned(expr_path, "Expected identifier"))?;
            let name = ident.to_string();
            Ok(quote! {
                Rule::var(#name)
            })
        }
        Expr::Field(expr_field) => {
            let base = parse_expr_to_string(&expr_field.base)?;
            let member = match &expr_field.member {
                Member::Named(ident) => ident.to_string(),
                Member::Unnamed(index) => index.index.to_string(),
            };
            let full_name = format!("{}.{}", base, member);
            Ok(quote! {
                Rule::var(#full_name)
            })
        }
        Expr::Index(expr_index) => {
            let base = parse_expr_to_string(&expr_index.expr)?;
            let index = parse_expr_to_string(&expr_index.index)?;
            let full_name = format!("{}[{}]", base, index);
            Ok(quote! {
                Rule::var(#full_name)
            })
        }
        Expr::MethodCall(expr_method_call) => {
            let receiver = parse_expr_to_string(&expr_method_call.receiver)?;
            let method = expr_method_call.method.to_string();
            let args: Vec<_> = expr_method_call
                .args
                .iter()
                .map(parse_expr_to_rule)
                .collect::<Result<_, _>>()?;
            let method_call = format!("{}.{}", receiver, method);
            Ok(quote! {
                Rule::custom(#method_call, vec![#(#args),*])
            })
        }
        _ => Err(syn::Error::new_spanned(
            expr,
            "Unsupported expression in rule macro",
        )),
    }
}

fn parse_expr_to_rule_with_validation(expr: &Expr) -> syn::Result<proc_macro2::TokenStream> {
    match expr {
        Expr::Binary(expr_bin) => {
            let left = crate::parse_expr_to_rule_with_validation(&expr_bin.left)?;
            let right = crate::parse_expr_to_rule_with_validation(&expr_bin.right)?;
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
                syn::BinOp::Eq(_) => quote! { #left.eq(#right).map_err(|e| e.to_string()).unwrap()},
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
        Expr::Unary(expr_unary) => {
            let operand = crate::parse_expr_to_rule_with_validation(&expr_unary.expr)?;
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
        Expr::Paren(expr_paren) => crate::parse_expr_to_rule_with_validation(&expr_paren.expr),
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
        Expr::Path(expr_path) => {
            let ident = expr_path
                .path
                .get_ident()
                .ok_or_else(|| syn::Error::new_spanned(expr_path, "Expected identifier"))?;
            let name = ident.to_string();
            Ok(quote! {
                RuleWithValidation::var(#name)
            })
        }
        Expr::Field(expr_field) => {
            let base = parse_expr_to_string(&expr_field.base)?;
            let member = match &expr_field.member {
                Member::Named(ident) => ident.to_string(),
                Member::Unnamed(index) => index.index.to_string(),
            };
            let full_name = format!("{}.{}", base, member);
            Ok(quote! {
                RuleWithValidation::var(#full_name)
            })
        }
        Expr::Index(expr_index) => {
            let base = parse_expr_to_string(&expr_index.expr)?;
            let index = parse_expr_to_string(&expr_index.index)?;
            let full_name = format!("{}[{}]", base, index);
            Ok(quote! {
                RuleWithValidation::var(#full_name)
            })
        }
        Expr::MethodCall(expr_method_call) => {
            let receiver = parse_expr_to_string(&expr_method_call.receiver)?;
            let method = expr_method_call.method.to_string();
            let args: Vec<_> = expr_method_call
                .args
                .iter()
                .map(crate::parse_expr_to_rule_with_validation)
                .collect::<Result<_, _>>()?;
            let method_call = format!("{}.{}", receiver, method);
            Ok(quote! {
                RuleWithValidation::custom(#method_call, vec![#(#args),*])
            })
        }
        _ => Err(syn::Error::new_spanned(
            expr,
            "Unsupported expression in rule macro",
        )),
    }
}

fn parse_expr_to_string(expr: &Expr) -> syn::Result<String> {
    match expr {
        Expr::Path(expr_path) => {
            let ident = expr_path
                .path
                .get_ident()
                .ok_or_else(|| syn::Error::new_spanned(expr_path, "Expected identifier"))?;
            Ok(ident.to_string())
        }
        Expr::Field(expr_field) => {
            let base = parse_expr_to_string(&expr_field.base)?;
            let member = match &expr_field.member {
                Member::Named(ident) => ident.to_string(),
                Member::Unnamed(index) => index.index.to_string(),
            };
            Ok(format!("{}.{}", base, member))
        }
        Expr::Index(expr_index) => {
            let base = parse_expr_to_string(&expr_index.expr)?;
            let index = parse_expr_to_string(&expr_index.index)?;
            Ok(format!("{}[{}]", base, index))
        }
        _ => Err(syn::Error::new_spanned(
            expr,
            "Expected variable name in field access",
        )),
    }
}
