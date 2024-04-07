mod content;

use crate::content::end;
use crate::content::paren;
use crate::content::str;
use crate::content::until;
use crate::content::Content;
use crate::content::{after, ws};
use std::fs;
use std::path::Path;

fn transfer_assert_eq_args(content: &str) -> Option<String> {
    let mut content = Content::new(content.to_string());
    // remove whitespaces
    content.seek(&after("(")).unwrap();
    content.delete(&ws()).unwrap();
    // find first argument
    content.seek(&after("normalize_schema")).unwrap();
    content.seek(&paren()).unwrap();
    content.seek(&after(",")).unwrap();
    content.delete(&ws()).unwrap();
    if content.is_done() {
        return None;
    }
    // find second argument
    content.seek(&until("normalize_schema")).unwrap();
    if content.is_done() {
        return None;
    }

    content.delete(&str("normalize_schema")).unwrap();
    content.delete(&paren()).unwrap();
    content.insert(r#" @"""#);
    content.delete(&ws()).unwrap();
    content.seek(&until(",")).unwrap();
    content.delete(&str(",")).ok();
    content.delete(&ws()).unwrap();
    content.seek(&end()).unwrap();
    let output = content.finish().unwrap();
    Some(output)
}
fn transform_assert_eq(content: &mut Content) -> Result<(), String> {
    let mut lookaheads = content.lookaheads();
    let args = lookaheads.seek(&until("("))?.take(&paren())?;
    let args = transfer_assert_eq_args(args);

    if let Some(args) = args {
        content.delete(&str("assert_eq!"))?;
        content.insert("insta::assert_snapshot!");
        content.seek(&until("("))?;
        content.delete(&paren())?;
        content.insert(&args);
        return Ok(());
    }

    content.seek(&after("assert_eq!"))?;
    Ok(())
}
fn transform_logic(content: &str) -> String {
    let mut content = Content::new(content.to_string());

    loop {
        content.seek(&until("assert_eq!")).unwrap();
        if content.is_done() {
            break;
        }
        transform_assert_eq(&mut content).unwrap();
    }

    content.finish().unwrap()
}
fn transform_file(file: &Path) {
    let content = fs::read_to_string(file).unwrap();
    let transformed_content = transform_logic(&content);
    fs::write(file, transformed_content).unwrap();
}

fn main() {
    let files = glob::glob("derive/tests/**/*.rs")
        .unwrap()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    for file in files {
        transform_file(&file);
    }
}
