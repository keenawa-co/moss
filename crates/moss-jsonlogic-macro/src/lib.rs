// macros.rs

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, Lit, Member};

#[proc_macro]
pub fn rule(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);
    match parse_expr_to_rule(&expr) {
        Ok(tokens) => TokenStream::from(quote! { #tokens }),
        Err(e) => e.to_compile_error().into(),
    }
}

fn parse_expr_to_rule(expr: &Expr) -> syn::Result<proc_macro2::TokenStream> {
    match expr {
        // Handle binary operations (e.g., a + b, x == y)
        Expr::Binary(expr_bin) => {
            let left = parse_expr_to_rule(&expr_bin.left)?;
            let right = parse_expr_to_rule(&expr_bin.right)?;
            let op = &expr_bin.op;

            let method = match op {
                syn::BinOp::Add(_) => quote! { add },
                syn::BinOp::Sub(_) => quote! { subtract },
                syn::BinOp::Mul(_) => quote! { multiply },
                syn::BinOp::Div(_) => quote! { divide },
                syn::BinOp::Rem(_) => quote! { modulo },
                syn::BinOp::Eq(_) => quote! { eq },
                syn::BinOp::Ne(_) => quote! { ne },
                syn::BinOp::Gt(_) => quote! { gt },
                syn::BinOp::Lt(_) => quote! { lt },
                syn::BinOp::Ge(_) => quote! { gte },
                syn::BinOp::Le(_) => quote! { lte },
                syn::BinOp::And(_) => quote! { and },
                syn::BinOp::Or(_) => quote! { or },
                _ => {
                    return Err(syn::Error::new_spanned(
                        op,
                        "Unsupported binary operator in rule macro",
                    ))
                }
            };

            Ok(quote! { #left.#method(#right) })
        }
        // Handle unary operations (e.g., !a)
        Expr::Unary(expr_unary) => {
            let operand = parse_expr_to_rule(&expr_unary.expr)?;
            let op = &expr_unary.op;

            let method = match op {
                syn::UnOp::Not(_) => quote! { not },
                _ => {
                    return Err(syn::Error::new_spanned(
                        op,
                        "Unsupported unary operator in rule macro",
                    ))
                }
            };

            Ok(quote! { #operand.#method() })
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
                    let inner_expr: Expr = syn::parse2(tokens.clone())?;

                    Ok(quote! {
                        RawRule::from(#inner_expr)
                    })
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
            let args = expr_method_call
                .args
                .iter()
                .map(parse_expr_to_rule)
                .collect::<Result<Vec<_>, _>>()?;
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
