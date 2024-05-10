use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, ItemFn, LitStr, Token};

struct HeaderMap(Vec<String>);

impl Parse for HeaderMap {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let parsed = Punctuated::<LitStr, Token![,]>::parse_terminated(input)?;
        Ok(HeaderMap(parsed.iter().map(|lit| lit.value()).collect()))
    }
}

#[proc_macro_attribute]
pub fn require_header(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let header_map: HeaderMap = parse_macro_input!(attr as HeaderMap);

    let checks = header_map.0.iter().map(|header_name| {
        quote! {
            {
                if header_map.get(#header_name).is_none() {
                    return Err(Error::resource_invalid(
                        &format!("Header '{}' not found", #header_name),
                        None,
                    ))
                    .extend_error()?;
                }
            }
        }
    });

    let prepended_code = quote! {
        {
            let header_map = ctx
                .data::<HeaderMap>()
                .expect("Failed to retrieve HeaderMap from context");

            #(#checks)*
        }
    };

    let prepended_stmts =
        syn::parse2(prepended_code).unwrap_or_else(|e| panic!("Failed to parse statements: {}", e));

    input_fn.block.stmts.insert(0, prepended_stmts);

    TokenStream::from(quote! { #input_fn })
}
