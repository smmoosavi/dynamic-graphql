mod content;

use crate::content::{after, str, until, ws, Content};
use std::fs;
use std::path::Path;

fn is_normalize_schema(lookaheads: &mut content::Lookaheads) -> bool {
    let res = lookaheads
        .seek(&after("("))
        .unwrap()
        .seek(&ws())
        .unwrap()
        .seek(&str("normalize_schema"));
    res.is_ok()
}
fn transform_assert_eq(content: &mut Content) -> Result<(), String> {
    println!("Transforming assert_eq");
    let mut lookaheads = content.lookaheads();
    if !is_normalize_schema(&mut lookaheads) {
        content.seek(&after("assert_eq!"))?;
        return Ok(());
    }

    // content.delete(&str("assert_eq!"))?;
    // content.insert("insta::assert_snapshot!");
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
