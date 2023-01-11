extern crate anyhow;

use rust_bert::pipelines::ner::NERModel;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let ner_model = NERModel::new(Default::default())?;

    let input = [
        "My name is Amy. I live in Paris.",
        "Paris is a city in France."
    ];

    let output = ner_model.predict(&input);

    for entity in output {
        println!("{:?}",entity);
    }

    Ok(())
}
