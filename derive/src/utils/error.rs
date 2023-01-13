use proc_macro2::TokenStream;
use syn::spanned::Spanned;

pub trait IntoTokenStream {
    fn into_token_stream(self) -> TokenStream;
}

impl IntoTokenStream for darling::Result<TokenStream> {
    fn into_token_stream(self) -> TokenStream {
        match self {
            Ok(tokens) => tokens,
            Err(err) => err.write_errors(),
        }
    }
}

pub trait WithSpan {
    fn with_span<T: Spanned>(self, node: &T) -> Self;
}

impl<T> WithSpan for darling::Result<T> {
    fn with_span<N: Spanned>(self, node: &N) -> Self {
        self.map_err(|e| e.with_span(node))
    }
}
