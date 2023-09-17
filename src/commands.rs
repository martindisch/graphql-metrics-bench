use std::path::Path;

use eyre::Result;
use graphql_metrics_bench::FieldCoordinates;

pub fn stats(path: &Path) -> Result<()> {
    let object_types = graphql_metrics_bench::get_object_types(path)?;

    let object_count = object_types.len();
    let field_count = object_types
        .iter()
        .map(|object_type| object_type.fields.len())
        .sum::<usize>();

    println!("Schema has {object_count} object types with a total of {field_count} fields");

    Ok(())
}

pub fn sample(path: &Path) -> Result<()> {
    let object_types = graphql_metrics_bench::get_object_types(path)?;
    let field_coordinates = FieldCoordinates::from(object_types);
    let selection = field_coordinates.choose(450);

    println!("{selection:#?}");

    Ok(())
}
