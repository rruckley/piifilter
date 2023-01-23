/// Generate summary from input text
/// 
use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};
use tokio::{sync::oneshot, task};

use rust_bert::pipelines::summarization::SummarizationModel;

type Message = (Vec<String>,oneshot::Sender<Vec<String>>);

pub struct SummaryFilter {
    sender: mpsc::SyncSender<Message>
}

impl SummaryFilter {
    pub fn spawn() -> (JoinHandle<Result<String,String>>, SummaryFilter) {
        let (sender, receiver) = mpsc::sync_channel(100);
        let handle = thread::spawn(move || Self::runner(receiver));

        (handle, SummaryFilter { sender })
    }
    fn runner(receiver: mpsc::Receiver<Message>) -> Result<String,String> {
        let mut model = SummarizationModel::new(Default::default()).expect("Could not create Summary Model");

        while let Ok((context, sender)) = receiver.recv() {
            let input = [context.first().unwrap().as_str()];
            let output = model.summarize(&input);
            let _send_result = sender.send(output);
        }

        Ok("Summary Runner Done".to_owned())
    }
    pub fn get_style() -> String {
        // Generate some style
        // 
        "
        ".to_owned()
    }
    pub async fn filter(&self, context : String) -> Result<String,String> {
    
        let input = vec![context];

        let (sender, receiver) = oneshot::channel();
        task::block_in_place(|| self.sender.send((input, sender))).expect("Summary: Could not spawn task");
        let output = receiver.await.expect("Summary: Could not receive message from thread");
    
        let mut result = "<div class=\"summary\">".to_owned();
        
        for summary in output {
                result.push_str(summary.as_str());
        };
        result.push_str("</div>");
    
        Ok(result)
    }
}