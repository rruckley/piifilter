extern crate anyhow;

use rocket::form::Form;
use rocket::http::ContentType;
use rocket::State;

mod ner;
mod pos;
mod regex;
mod docs;
use docs::Document;

use ner::NERFilter;
use pos::POSFilter;


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
async fn process(pos : &State<POSFilter>,ner : &State<NERFilter>, form_data : Form<InputData>) -> (ContentType,String) {
    let action = &form_data.action;
    let result = match action.as_str() {
        "regex" => process_regex(form_data.text.clone()).await,
        "nep" => process_ner(ner, form_data.text.clone()).await,
        "pos" => process_pos(pos, form_data.text.clone()).await,
        _ => Ok(format!("Invalid Action: {}",action).to_owned())
    };
    (ContentType::HTML,result.unwrap())
}

///! Function to execute regex against supplied text to find PII identifiers
async fn process_regex(_context : String) -> Result<String,String> {
    Ok("Regex Not implemented".to_owned())
}

async fn process_ner(ner : &State<NERFilter>, context : String) -> Result<String,String> {
    let result = ner.filter(context).await?;
    Ok(format!("<html><h2>NER All Good</h2><p>{}</p></html>",result.to_owned()))
}

async fn process_pos(pos :&State<POSFilter>, context : String) -> Result<String,String> {
    // PoS tagging
    let result = pos.filter(context).await?;

    Ok(format!("Process POS: {}",result))
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
    let (_handle,pos_filter) = POSFilter::spawn();
    let (_handle2, ner_filter) = NERFilter::spawn();
    rocket::build()
        .manage(pos_filter)
        .manage(ner_filter)
        .mount("/", routes![index,process])
}
