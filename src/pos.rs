use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};
use tokio::{sync::oneshot, task};

use rust_bert::pipelines::pos_tagging::POSModel;

type Message = (Vec<String>,oneshot::Sender<Vec<POSTag>>);
pub struct POSFilter {
    model : POSModel,
    sender: mpsc::SyncSender<Message>
}

impl POSFilter {
    pub fn spawn() -> (JoinHandle<Result<<String>,<String>>>, POSFilter) {
        let (sender, receiver) = mpsc::sync_channel(100);
        let model = POSModel::new(Default::default()).expect("Couldnt create PoS model");
        Self {
            model,
            sender,
        }
    }
    fn filter(context : String) -> Result<String,String> {
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
}