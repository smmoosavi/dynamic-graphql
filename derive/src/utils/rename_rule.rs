use darling::FromMeta;
use inflector::Inflector;

#[derive(Debug, Copy, Clone, FromMeta)]
pub enum RenameRule {
    #[darling(rename = "lowercase")]
    Lower,
    #[darling(rename = "UPPERCASE")]
    Upper,
    #[darling(rename = "PascalCase")]
    Pascal,
    #[darling(rename = "camelCase")]
    Camel,
    #[darling(rename = "snake_case")]
    Snake,
    #[darling(rename = "SCREAMING_SNAKE_CASE")]
    ScreamingSnake,
}

impl RenameRule {
    fn rename(&self, name: impl AsRef<str>) -> String {
        match self {
            Self::Lower => name.as_ref().to_lowercase(),
            Self::Upper => name.as_ref().to_uppercase(),
            Self::Pascal => name.as_ref().to_pascal_case(),
            Self::Camel => name.as_ref().to_camel_case(),
            Self::Snake => name.as_ref().to_snake_case(),
            Self::ScreamingSnake => name.as_ref().to_screaming_snake_case(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum RenameTarget {
    Type,
    EnumItem,
    Field,
    Argument,
}

impl RenameTarget {
    fn rule(&self) -> RenameRule {
        match self {
            RenameTarget::Type => RenameRule::Pascal,
            RenameTarget::EnumItem => RenameRule::ScreamingSnake,
            RenameTarget::Field => RenameRule::Camel,
            RenameTarget::Argument => RenameRule::Camel,
        }
    }

    pub fn rename(&self, name: impl AsRef<str>) -> String {
        self.rule().rename(name)
    }
}

pub trait RenameRuleExt {
    fn rename(&self, name: impl AsRef<str>, target: RenameTarget) -> String;
}

impl RenameRuleExt for Option<RenameRule> {
    fn rename(&self, name: impl AsRef<str>, target: RenameTarget) -> String {
        self.unwrap_or(target.rule()).rename(name)
    }
}

pub fn calc_field_name(
    name: &Option<String>,
    ident: &syn::Ident,
    rename_rule: &Option<RenameRule>,
) -> String {
    match name {
        Some(name) => name.to_owned(),
        None => match rename_rule {
            Some(rename_fields) => rename_fields.rename(ident.to_string()),
            None => RenameTarget::Field.rename(ident.to_string()),
        },
    }
}

pub fn calc_type_name(name: &Option<String>, ident: &syn::Ident) -> String {
    match name {
        Some(name) => name.to_owned(),
        None => RenameTarget::Type.rename(ident.to_string()),
    }
}
