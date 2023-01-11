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

impl RenameRuleExt for Option<&RenameRule> {
    fn rename(&self, name: impl AsRef<str>, target: RenameTarget) -> String {
        self.unwrap_or(&target.rule()).rename(name)
    }
}

/// Calculate the name of a field.
/// @arg name: The name of the field, specified by the user.
/// @arg ident_name: The name of the field, extracted from the code.
pub fn calc_field_name(
    name: Option<&str>,
    ident_name: &str,
    rename_rule: Option<&RenameRule>,
) -> String {
    match name {
        Some(name) => name.to_owned(),
        None => match rename_rule {
            Some(rename_fields) => rename_fields.rename(ident_name),
            None => RenameTarget::Field.rename(ident_name),
        },
    }
}

pub fn calc_input_field_name(
    name: Option<&str>,
    ident_name: &str,
    rename_rule: Option<&RenameRule>,
) -> String {
    match name {
        Some(name) => name.to_owned(),
        None => match rename_rule {
            Some(rename_fields) => rename_fields.rename(ident_name),
            None => RenameTarget::Argument.rename(ident_name),
        },
    }
}

/// Calculate the name of a type.
/// @arg name: The name of the type, specified by the user.
/// @arg type_name: The name of the type, extracted from the code.
pub fn calc_type_name(name: Option<&str>, type_name: &str) -> String {
    match name {
        Some(name) => name.to_owned(),
        None => RenameTarget::Type.rename(type_name),
    }
}

pub fn calc_enum_item_name(
    name: Option<&str>,
    item_name: &str,
    rename_rule: Option<&RenameRule>,
) -> String {
    match name {
        Some(name) => name.to_owned(),
        None => match rename_rule {
            Some(rename_items) => rename_items.rename(item_name),
            None => RenameTarget::EnumItem.rename(item_name),
        },
    }
}

pub fn calc_arg_name(
    name: Option<&str>,
    ident_name: &str,
    rename_rule: Option<&RenameRule>,
) -> String {
    match name {
        Some(name) => name.to_owned(),
        None => {
            let ident_name = ident_name.strip_prefix('_').unwrap_or(ident_name);
            rename_rule.rename(ident_name, RenameTarget::Argument)
        }
    }
}
