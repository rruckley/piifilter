extern crate anyhow;



use rocket::form::Form;
use rocket::response::{content, status};
use rocket::response::status::NotFound;
use rocket::http::ContentType;
use rocket::State;

use tokio::task;

mod ner;
mod pos;
mod regex;
mod docs;
use docs::Document;

use ner::NERFilter;
use pos::POSFilter;
use crate::regex::RegexFilter;


#[macro_use] extern crate rocket;

enum ActionType {
    Regex,
    NamedEntityRecognition,
    PartsOfSpeed,
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

async fn process_ner(ner : NERFilter, context : String) -> Result<String,String> {
    let result = task::spawn_blocking(move || {
        ner(context)
    }).await.unwrap();
    Ok(format!("<html><h2>NER All Good</h2><p>{}</p></html>",result.expect("Error with NER model")).to_owned())
}

async fn process_pos(pos : POSFilter, context : String) -> Result<String,String> {
    // PoS tagging
    let result = task::spawn_blocking(move || {
        pos(context)
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

struct Filters {
    ner : NERFilter,
    pos : POSFilter,
    regex : RegexFilter,
}

#[launch]
async fn rocket() -> _ {
    let _passport = Document::new(docs::DocType::CurrentPassport,70);
    let ner = NERFilter::new();
    let pos = POSFilter::new();
    let regex = RegexFilter::new();
    let filters = Filters {
        ner,
        pos,
        regex,
    };
    rocket::build()
        .manage(filters)
        .mount("/", routes![index,process])
}
