use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, ItemFn, LitStr, Stmt};

#[proc_macro_attribute]
pub fn check_header(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);
    // let header_name = parse_macro_input!(attr as LitStr).value();

    let prepended_code = quote! {
        let header_map = ctx.data::<HeaderMap>()?;
        // if header_map.get("header_name").is_none() {
        //     return Err(Error::resource_invalid("header not found", None)).extend_error()?;
        // }
    };

    let prepended_stmts =
        syn::parse2(prepended_code).unwrap_or_else(|e| panic!("Failed to parse statements: {e}"));

    input_fn.block.stmts.insert(0, prepended_stmts);

    TokenStream::from(quote! { #input_fn })
}

// #[proc_macro_attribute]
// pub fn check_header(attr: TokenStream, item: TokenStream) -> TokenStream {
//     let input_fn = parse_macro_input!(item as ItemFn);
//     let header_name = syn::parse::<syn::LitStr>(attr).unwrap();

//     // let ItemFn {
//     //     attrs,
//     //     vis,
//     //     sig,
//     //     block,
//     // } = &input_fn;

//     // let check_statement: Stmt = parse_quote! {
//     //     let header_value = ctx.http_header(#header_name)
//     //         .ok_or_else(|| async_graphql::Error::new(format!("Missing header: {}", #header_name)))?;
//     //     println!("Header {}: {}", #header_name, header_value);
//     // };

//     // let mut new_block = block.clone();
//     // new_block.stmts.insert(0, check_statement);

//     // let expanded = quote! {
//     //     #(#attrs)*
//     //     #vis #sig {
//     //         #new_block
//     //     }
//     // };

//     TokenStream::from(expanded)
// }
