use std::path::PathBuf;
use proc_macro::TokenStream;

#[proc_macro]
pub fn include_builtin_symbols(_path: TokenStream) -> TokenStream {
    let file = _path.to_string();
    let path = PathBuf::from(file);

    let mut tokens = TokenStream::new();
}
