extern crate anyhow;

use rocket::form::Form;
use rocket::http::ContentType;
use rocket::State;
use rocket::fs::{FileServer,relative};

mod ner;
mod pos;
mod regex;
mod dialog;
mod summary;
mod docs;
mod qa;
mod embed;
mod qdrant;

use docs::Document;
use ner::NERFilter;
use pos::POSFilter;
use embed::EmbedFilter;
use summary::SummaryFilter;
use crate::regex::RegexFilter;
use dialog::DialogFilter;
use qa::QAFilter;
use qdrant::{QDrant, QDrantPoints, QDrantPoint};


#[macro_use] extern crate rocket;

#[derive(FromForm)]
struct InputData {
    text : String,
    question : String,
    action : String,
}

fn get_style() -> String {
"
<link rel=\"stylesheet\" href=\"/static/style.css\" type=\"text/css\" />
".to_owned()
}

#[post("/process", data = "<form_data>")]
async fn process(
        pos : &State<POSFilter>,
        ner : &State<NERFilter>,
        regex : &State<RegexFilter>,
        dialog : &State<DialogFilter>, 
        summary : &State<SummaryFilter>,
        qa : &State<QAFilter>,
        embed : &State<EmbedFilter>,
        form_data : Form<InputData>) -> (ContentType,String) {
    let action = &form_data.action;
    let style = get_style();
    let result = match action.as_str() {
        "regex" => {
            process_regex(regex, form_data.text.clone()).await
            //Ok(format!("<html><head><title>Regular Expressions</title>{}</head><body><h2>Regular Expressions</h2>{}</body></html>",style,reg))
        },
        "nep" => {
            let ner = process_ner(ner, form_data.text.clone()).await.unwrap();
            let ns = NERFilter::get_style();
            Ok(format!("<html><head><title>NER</title>{}{}</head><h2>NER Output</h2><p>{}</p></html>",style,ns,ner))
        },
        "pos" => {
            let pos = process_pos(pos, form_data.text.clone()).await.unwrap();
            let ps = POSFilter::get_style();
            Ok(format!("<html><head>{}{}</head><body>{}</body></html>",style,ps,pos))
        },
        "sum" => {
            let sum = process_summary(summary, form_data.text.clone()).await.expect("Could not call summary filter");
            let ss = SummaryFilter::get_style();
            Ok(format!("<html><head>{}{}</head><body>{}</html>",style,ss,sum))
        },
        "dialog" => {
            let log = process_dialog(dialog, form_data.text.clone()).await.unwrap();
            Ok(format!("<html><head>{}</head><body>{}</html>",style,log))
        },
        "qa" => {
            let qa = process_qa(qa, 
                form_data.question.clone(),
                form_data.text.clone(),
            ).await.unwrap();
            Ok(format!("<html><head>{}</head><body>{}</body></html>",style,qa))
        },
        "embed" => {
            let embed = process_embed(embed,
                form_data.text.clone(),
            ).await.unwrap();
            Ok(format!("<html><head>{}</head><body>{}</body></html>",style,embed))
        },
        "all" => {
            let ner = process_ner(ner, form_data.text.clone()).await.unwrap();
            let ns = NERFilter::get_style();
            let pos = process_pos(pos, form_data.text.clone()).await.unwrap();
            let ps = POSFilter::get_style();
            let reg = process_regex(regex, form_data.text.clone()).await.unwrap();
            
            Ok(format!("<html><head>{}{}{}</head><body>{}<br />{}<br />{}</body>",style,ns,ps,ner,pos,reg))
        },
        _ => Ok(format!("Invalid Action: {}",action))
    };
    (ContentType::HTML,result.unwrap())
}

///! Function to execute regex against supplied text to find PII identifiers
async fn process_regex(regex : &State<RegexFilter>,context : String) -> Result<String,String> {
    Ok(regex.filter(context).unwrap())
}

async fn process_ner(ner : &State<NERFilter>, context : String) -> Result<String,String> {
    ner.filter(context).await
}

async fn process_pos(pos :&State<POSFilter>, context : String) -> Result<String,String> {
    // PoS tagging
    pos.filter(context).await
}

async fn process_dialog(dialog: &State<DialogFilter>, context : String) -> Result<String,String> {
    let result = dialog.filter(context).await?;

    Ok(result)
}

async fn process_summary(summary: &State<SummaryFilter>, context: String) -> Result<String,String> {
    let result = summary.filter(context).await?;

    Ok(result)
}

async fn process_qa(qa: &State<QAFilter>,question : String, context: String) -> Result<String,String> {
    let result = qa.filter(question, context).await?;
    Ok(result)
}

async fn process_embed(embed: &State<EmbedFilter>, content : String) -> Result<String,String> {
    // Turn content into a Vec<String>
    let lines : Vec<String> = content.lines().map(|l| l.to_string()).collect();
    let result = embed.filter(lines).await;
    match result {
        Ok(r) => {
            info!("Found parent vector of size {}",r.len());
            let mut points : Vec<QDrantPoint> = vec![];
            for v in r {
                points.push(QDrantPoint {
                    id : 5,
                    payload : "{}".to_string(),
                    vector: v,    
                });
            }
            let vector = QDrantPoints {
                points
            };
            match QDrant::store(vector) {
                Ok(m) => {
                    info!("Stored vector to QDrant");
                    Ok(m)
                },
                Err(e) => {
                    error!("Could not store vector: {e}");
                    Err(e)
                },
            }
            
        },
        Err(e) => Err(e),    
    }
    
}

#[get("/")]
async fn index() -> (ContentType, &'static str) {
    (ContentType::HTML, "
    <html>
    <head>
        <title>PII Experiments</title>
        <link rel=\"stylesheet\" href=\"/static/style.css\" type=\"text/css\" />
    </head>
    <body>
    <h2>PII Filter</h2>
    <div>
        <form method=\"post\" action=\"/process\">
            <textarea name=\"text\" rows=\"10\" cols=\"64\" autofocus=\"true\" required=\"true\"></textarea>
            <br />
            <select name=\"action\">
                <option value=\"regex\">Regular Expressions</option>
                <option value=\"nep\">Named Entity Parsing</option>
                <option value=\"pos\">Parts of Speech Tagging</option>
                <option value=\"sum\">Summary</option>
                <option value=\"dialog\">Dialog</option>
                <option value=\"qa\">Question / Answer</option>
                <option value=\"embed\">Sentence Embedding</option>
                <option value=\"all\">All</option>
            </select>
            <br />
            <input name=\"question\" />
            <button type=\"submit\">Process Text</button>
        </form>
    </div>
    </body>
    </html>
    ")
}

#[launch]
async fn rocket() -> _ {
    env_logger::init();
    let version = env!("CARGO_PKG_VERSION");
    info!("Starting piifilter {version}");
    let _passport = Document::new(docs::DocType::CurrentPassport,70);
    let (_handle1, pos_filter) = POSFilter::spawn();
    let (_handle2, ner_filter) = NERFilter::spawn();
    let regex_filter= RegexFilter::new();
    let (_handle3,dialog_filter) = DialogFilter::spawn();
    let (_handle4, summary_filter) = SummaryFilter::spawn();
    let (_handle5, qa_filter) = QAFilter::spawn();
    let (_handle6, embed_filter) = EmbedFilter::spawn();
    rocket::build()
        .manage(pos_filter)
        .manage(ner_filter)
        .manage(regex_filter)
        .manage(dialog_filter)
        .manage(summary_filter)
        .manage(qa_filter)
        .manage(embed_filter)
        .mount("/static", FileServer::from(relative!("static")))
        .mount("/", routes![index,process])
}
