extern crate anyhow;

use rust_bert::pipelines::{ner::NERModel};
use rust_bert::pipelines::pos_tagging::POSModel;

use rocket::form::Form;
use rocket::response::{content, status};
use rocket::http::ContentType;

mod docs;
use docs::Document;


#[macro_use] extern crate rocket;

enum ActionType {
    Regex,
    NamedEntityRecognition,
    Both,
}

#[derive(FromForm)]
struct InputData {
    text : String,
    action : String,
}


#[post("/process", data = "<data>")]
fn process(data : Form<InputData>) -> std::io::Result<()> {

    let input = [
        "My name is Amy. I live in Paris.",
        "Paris is a city in France.",
        "My Medicare number is 0234903 and my email is ryan.ruckley@optus.com.au",
        "I want to cancel my phone 042349839, my passport is 2340934",
        "We have given soluiton walkthrough (Sukumar) and then Mona has shared the reasoning for the http used within the docker",
        "Richard Branson, founder of Virgin Galactic, is now offering manned space flights for as little as $200,000."
    ];

    for line in input {
        println!("Input: {}",line);
    }

    let ner_model = NERModel::new(Default::default()).expect("Could not create NER model");

    let output = ner_model.predict_full_entities(&input);

    for entity in output {
        println!("{:?}",entity);
    };
    // PoS tagging
    let pos_model = POSModel::new(Default::default()).expect("Couldnt create PoS model");

    let input = ["My name is Ryan Ruckley and I am an architect."];
    let output = pos_model.predict(&input);

    for (pos, pos_tag) in output[0].iter().enumerate() {
        println!("{} - {:?}",pos, pos_tag);
    }

    Ok(())
}

#[get("/")]
fn index() -> (ContentType, &'static str) {
    (ContentType::HTML, "
    <html>
    <head><title>PII Experiments</title></head>
    <body>
    <h2>PII Filter</h2>
    <form method=\"post\" action=\"/process\">
    <textarea name=\"text\"></textarea>
    <input name=\"action\" />
    <input type=\"submit\" />
    </form>
    </body>
    </html>
    ")
}

#[launch]
fn rocket() -> _ {
    let _passport = Document::new(docs::DocType::CurrentPassport,70);
    rocket::build().mount("/", routes![index,process])
}
