
use rust_bert::pipelines::{ner::NERModel};
pub struct NERFilter {
    model : NERModel,
}

impl NERFilter {
    pub fn new() -> Self {
        let model = NERModel::new(Default::default()).expect("Could not create NER model");
        Self {
            model
        }
    }
    pub fn filter(context : String) -> Result<String,String> {
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
}