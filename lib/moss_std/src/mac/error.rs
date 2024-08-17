#[macro_export]
macro_rules! this_transparent_error {
    ($inner_type:ty, $outer_variant:ident, $inner_variant:path, $err_type:ty) => {
        impl From<$err_type> for $inner_type {
            fn from(err: $err_type) -> Self {
                <$inner_type>::$outer_variant($inner_variant(err))
            }
        }
    };
}
