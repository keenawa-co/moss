use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

#[proc_macro]
pub fn register_contribution(input: TokenStream) -> TokenStream {
    // Парсим входной токен как функцию
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = input_fn.sig.ident.clone();

    // Генерируем уникальное имя для статического конструктора
    let contrib_entry = syn::Ident::new(&format!("__contrib_{}", fn_name), fn_name.span());

    // Генерируем код:
    // 1. Оставляем оригинальную функцию
    // 2. Создаём статическую переменную, которая добавляет вклад в `linkme::distributed_slice`
    let expanded = quote! {
        #input_fn

        #[linkme::distributed_slice(crate::state::CONTRIBUTIONS)]
        static #contrib_entry: crate::state::Contribution = #fn_name();
    };

    expanded.into()
}
