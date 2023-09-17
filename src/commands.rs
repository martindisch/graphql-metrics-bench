use std::path::Path;

use apollo_parser::ast::Definition;
use eyre::Result;

pub fn stats(path: &Path) -> Result<()> {
    let document = graphql_metrics_bench::get_document(path)?;

    let (object_count, field_count) = document.definitions().fold((0, 0), |acc, e| match e {
        Definition::ObjectTypeDefinition(object_type) => (
            acc.0 + 1,
            acc.1
                + object_type
                    .fields_definition()
                    .unwrap()
                    .field_definitions()
                    .count(),
        ),
        _ => acc,
    });

    println!("Schema has {object_count} object types with a total of {field_count} fields");

    Ok(())
}
