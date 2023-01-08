use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("{0}")]
    Syn(#[from] syn::Error),

    #[error("{0}")]
    Darling(#[from] darling::Error),
}

impl GeneratorError {
    pub fn write_errors(self) -> TokenStream {
        match self {
            GeneratorError::Syn(err) => err.to_compile_error(),
            GeneratorError::Darling(err) => err.write_errors(),
        }
    }
}
pub type GeneratorResult<T> = std::result::Result<T, GeneratorError>;

pub trait IntoTokenStream {
    fn into_token_stream(self) -> TokenStream;
}
impl IntoTokenStream for GeneratorResult<TokenStream> {
    fn into_token_stream(self) -> TokenStream {
        match self {
            Ok(tokens) => tokens,
            Err(err) => err.write_errors(),
        }
    }
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

impl WithSpan for GeneratorError {
    fn with_span<T: Spanned>(self, node: &T) -> Self {
        match self {
            GeneratorError::Syn(e) => {
                GeneratorError::Darling(darling::Error::from(e).with_span(node))
            }
            GeneratorError::Darling(e) => GeneratorError::Darling(e.with_span(node)),
        }
    }
}
impl<T> WithSpan for GeneratorResult<T> {
    fn with_span<N: Spanned>(self, node: &N) -> Self {
        self.map_err(|e| e.with_span(node))
    }
}
