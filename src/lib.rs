use std::{fs, path::Path};

use apollo_parser::{
    ast::{Definition, Document},
    Parser,
};
use eyre::{eyre, Result};

/// Collects all object types from the given schema.
pub fn get_object_types(path: impl AsRef<Path>) -> Result<Vec<ObjectType>> {
    let document = get_document(path)?;

    let object_types = document
        .definitions()
        .filter_map(|definition| match definition {
            Definition::ObjectTypeDefinition(object_type) => {
                let fields = object_type
                    .fields_definition()?
                    .field_definitions()
                    .filter_map(|field| Some(field.name()?.text().to_string()))
                    .collect();

                Some(ObjectType {
                    name: object_type.name()?.text().to_string(),
                    fields,
                })
            }
            _ => None,
        })
        .collect();

    Ok(object_types)
}

/// An object type from the schema.
pub struct ObjectType {
    pub name: String,
    pub fields: Vec<String>,
}

/// Parses the given GraphQL document.
fn get_document(path: impl AsRef<Path>) -> Result<Document> {
    let input = fs::read_to_string(path)?;
    let parser = Parser::new(&input);
    let ast = parser.parse();

    if ast.errors().len() != 0 {
        return Err(eyre!("AST contains errors"));
    }

    Ok(ast.document())
}
