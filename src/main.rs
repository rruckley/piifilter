extern crate anyhow;

use rocket::form::Form;
use rocket::http::ContentType;
use rocket::State;

mod ner;
mod pos;
mod regex;
mod dialog;
mod docs;
use docs::Document;

use ner::NERFilter;
use pos::POSFilter;
use crate::regex::RegexFilter;
use dialog::DialogFilter;


#[macro_use] extern crate rocket;

#[derive(FromForm)]
struct InputData {
    text : String,
    action : String,
}


#[post("/process", data = "<form_data>")]
async fn process(pos : &State<POSFilter>,ner : &State<NERFilter>,regex : &State<RegexFilter>,dialog : &State<DialogFilter>, form_data : Form<InputData>) -> (ContentType,String) {
    let action = &form_data.action;
    let result = match action.as_str() {
        "regex" => process_regex(regex, form_data.text.clone()).await,
        "nep" => process_ner(ner, form_data.text.clone()).await,
        "pos" => process_pos(pos, form_data.text.clone()).await,
        "dialog" => process_dialog(dialog, form_data.text.clone()).await,
        "all" => {
            let ner = process_ner(ner, form_data.text.clone()).await.unwrap();
            let ns = NERFilter::get_style();
            let pos = process_pos(pos, form_data.text.clone()).await.unwrap();
            let ps = POSFilter::get_style();
            let reg = process_regex(regex, form_data.text.clone()).await.unwrap();
            Ok(format!("<html><head>{}{}</head><body>{}<br />{}<br />{}</body>",ns,ps,ner,pos,reg))
        }
        _ => Ok(format!("Invalid Action: {}",action))
    };
    (ContentType::HTML,result.unwrap())
}

///! Function to execute regex against supplied text to find PII identifiers
async fn process_regex(regex : &State<RegexFilter>,context : String) -> Result<String,String> {
    Ok(regex.filter(context).unwrap())
}

async fn process_ner(ner : &State<NERFilter>, context : String) -> Result<String,String> {
    let result = ner.filter(context).await?;
    let style = NERFilter::get_style();
    Ok(format!("<html><head><title>NER</title>{}</head><h2>NER Output</h2><p>{}</p></html>",style,result))
}

async fn process_pos(pos :&State<POSFilter>, context : String) -> Result<String,String> {
    // PoS tagging
    let result = pos.filter(context).await?;
    let style = POSFilter::get_style();

    Ok(format!("<html><head>{}</head><body>{}</body></html>",style,result))
}

async fn process_dialog(dialog: &State<DialogFilter>, context : String) -> Result<String,String> {
    let result = dialog.filter(context).await?;

    Ok(format!("<h2>Regular Expressions</h2>{}",result))
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
    <option value=\"all\">All</option>
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
    let (_handle1, pos_filter) = POSFilter::spawn();
    let (_handle2, ner_filter) = NERFilter::spawn();
    let regex_filter= RegexFilter::new();
    let (_handle3,dialog_filter) = DialogFilter::spawn();
    rocket::build()
        .manage(pos_filter)
        .manage(ner_filter)
        .manage(regex_filter)
        .manage(dialog_filter)
        .mount("/", routes![index,process])
}
