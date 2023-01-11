fn main() {
    println!("Hello, world!");

    let ner_model = NERModel::new(default::default())?;

    let input = [
        "My name is Amy. I live in Paris.",
        "Paris is a city in France."
    ];

    let output = ner_model.predict(&input);
}
