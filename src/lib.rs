use std::{fs, path::Path};

use apollo_parser::{ast::Document, Parser};
use eyre::{eyre, Result};

pub fn get_document(path: impl AsRef<Path>) -> Result<Document> {
    let input = fs::read_to_string(path)?;
    let parser = Parser::new(&input);
    let ast = parser.parse();

    if ast.errors().len() != 0 {
        return Err(eyre!("AST contains errors"));
    }

    Ok(ast.document())
}
