use darling::util::Ignored;

pub trait FromSignature: Sized {
    fn from_signature(sig: &mut syn::Signature) -> darling::Result<Self>;
}

impl FromSignature for Ignored {
    fn from_signature(_sig: &mut syn::Signature) -> darling::Result<Self> {
        Ok(Ignored)
    }
}
