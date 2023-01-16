extern crate anyhow;

use rust_bert::pipelines::{ner::NERModel};
use rust_bert::pipelines::pos_tagging::POSModel;

use rocket::form::Form;
use rocket::response::{content, status};
use rocket::response::status::NotFound;
use rocket::http::ContentType;

mod docs;
use docs::Document;
use tokio::task;


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


#[post("/process", data = "<form_data>")]
async fn process(form_data : Form<InputData>) -> (ContentType,String) {
    let action = &form_data.action;
    let result = match action.as_str() {
        "regex" => process_regex(form_data.text.clone()).await,
        "nep" => process_ner(form_data.text.clone()).await,
        "pos" => process_pos(form_data.text.clone()).await,
        _ => Ok(format!("Invalid Action: {}",action).to_owned())
    };
    (ContentType::HTML,result.unwrap())
}

///! Function to execute regex against supplied text to find PII identifiers
async fn process_regex(context : String) -> Result<String,String> {
    Ok("Regex Not implemented".to_owned())
}

fn _holder(context : String) -> Result<String,String> {
    let ner_model = NERModel::new(Default::default()).expect("Could not create NER model");

    let input = [context.as_str()];
    let output = ner_model.predict_full_entities(&input);

    for entity in output {
        println!("{:?}",entity);
    };
    Ok("Filtering NER".to_owned())
}

fn filter_ner(context : String) -> Result<String,String> {
    let ner_model = NERModel::new(Default::default()).expect("Could not create NER model");

    let input = [context.as_str()];
    let output = ner_model.predict_full_entities(&input);

    let mut result = "<html><h2>POS Output</h2><body>".to_owned();
    result.push_str("<ul>");
    for tag in output {
        for t in tag {
            result.push_str("<li>");
            result.push_str(t.label.as_str());
            result.push_str(" : ");
            result.push_str(t.word.as_str());
            result.push_str("</li>");
        }
    }
    result.push_str("</ul></body></html>");
    Ok(result)
}

async fn process_ner(context : String) -> Result<String,String> {
    let result = task::spawn_blocking(move || {
        filter_ner(context)
    }).await.unwrap();
    Ok(format!("<html><h2>NER All Good</h2><p>{}</p></html>",result.expect("Error with NER model")).to_owned())
}

fn filter_pos(context : String) -> Result<String,String> {
    let pos_model = POSModel::new(Default::default()).expect("Couldnt create PoS model");

    let input = [context];
    let output = pos_model.predict(&input);

    let mut result = "<html><h2>POS Output</h2><body>".to_owned();
    result.push_str("<ul>");
    for tag in output {
        for t in tag {
            result.push_str("<li>");
            result.push_str(t.label.as_str());
            result.push_str(" : ");
            result.push_str(t.word.as_str());
            result.push_str("</li>");
        }
    }
    result.push_str("</ul></body></html>");

    Ok(result)
}

async fn process_pos(context : String) -> Result<String,String> {
  // PoS tagging
  let result = task::spawn_blocking(move || {
    filter_pos(context)
  }).await.unwrap();
  Ok(format!("Process POS: {}",result.unwrap()).to_owned())
}

#[get("/")]
async fn index() -> (ContentType, &'static str) {
    (ContentType::HTML, "
    <html>
    <head><title>PII Experiments</title></head>
    <body>
    <h2>PII Filter</h2>
    <form method=\"post\" action=\"/process\">
    <textarea name=\"text\" rows=\"10\" cols=\"64\"></textarea>
    <br />
    <select name=\"action\">
    <option value=\"regex\">Regular Expressions</option>
    <option value=\"nep\">Named Entity Parsing</option>
    <option value=\"pos\">Parts of Speech Tagging</option>
    </select>
    <br />
    <input type=\"submit\" />
    </form>
    </body>
    </html>
    ")
}

#[launch]
async fn rocket() -> _ {
    let _passport = Document::new(docs::DocType::CurrentPassport,70);
    rocket::build()
        .mount("/", routes![index,process])
}
