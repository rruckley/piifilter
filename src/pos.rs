use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};
use tokio::{sync::oneshot, task};

use rust_bert::pipelines::pos_tagging::POSModel;
use rust_bert::pipelines::pos_tagging::POSTag;

type Message = (Vec<String>,oneshot::Sender<Vec<Vec<POSTag>>>);
pub struct POSFilter {
    sender: mpsc::SyncSender<Message>
}

impl POSFilter {
    pub fn spawn() -> (JoinHandle<Result<String,String>>, POSFilter) {
        let (sender, receiver) = mpsc::sync_channel(100);
        let handle = thread::spawn(move || Self::runner(receiver));

        (handle, POSFilter { sender })
    }
    fn runner(receiver: mpsc::Receiver<Message>) -> Result<String,String> {
        let model = POSModel::new(Default::default()).expect("Couldnt create PoS model");

        while let Ok((context, sender)) = receiver.recv() {
            let result = model.predict(&context);
            let _send_result = sender.send(result);
        }

        Ok("POS Runner Done".to_owned())
    }
    pub fn get_style() -> String {
        "
        <style type=\"text/css\">
        span.NN {
            background-color: #9F9;
        }
        span.NNP {
            background-color: #99F;
        }
        span.JJ {
            background-color: #F99;
        }
        </style>
        ".to_owned()
    }
    pub async fn filter(&self, context : String) -> Result<String,String> {
    
        let input = vec![context];

        let (sender, receiver) = oneshot::channel();
        task::block_in_place(|| self.sender.send((input, sender))).expect("POS: Could not spawn task");
        let output = receiver.await.expect("POS: Could not receive message from thread");
    
        let mut result = "<div class=\"pos\">".to_owned();
        
        for row in output {
            for t in row {
                result.push_str(format!("<span class=\"{}\">{}</span>&nbsp;",t.label,t.word).as_str());
            };  
        }
        result.push_str("</div>");
    
        Ok(result)
    }
}