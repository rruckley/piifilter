// Question / Answer Module
use tokio::{sync::oneshot, task};
use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel,Answer};

type Message = (Vec<String>,oneshot::Sender<Vec<Vec<Answer>>>);
use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};


pub struct QAFilter {
    sender: mpsc::SyncSender<Message>
}

impl QAFilter {
pub fn spawn() -> (JoinHandle<Result<String,String>>, QAFilter) {
        let (sender, receiver) = mpsc::sync_channel(100);
        let handle = thread::spawn(move || Self::runner(receiver));

        (handle, QAFilter { sender })
    }

    fn runner(receiver: mpsc::Receiver<Message>) -> Result<String,String> {
        let model = QuestionAnsweringModel::new(Default::default())
            .expect("Could not create QA Model");

        while let Ok((context, sender)) = receiver.recv() {
                let question = context.get(0).unwrap().to_string();
                println!("Question: {:?}",context);
                let context = context.get(1..).unwrap().join("\n");
                let output = model.predict(&[QaInput { question, context }], 1 , 32);
                let _send_result = sender.send(output);
            }
        Ok("QA Runner Done".to_owned())
    }

    pub async fn filter(&self, question: String, context : String) -> Result<String,String> {

        let input = vec![question,context];

        let (sender, receiver) = oneshot::channel();
        task::block_in_place(|| self.sender.send((input, sender))).expect("QA: Could not spawn task");
        let output = receiver.await.expect("QA: Could not receive message from thread");
    
        let mut result = "<div class=\"qa\">".to_owned();
        
        for summary in output {
            for a in summary {
                result.push_str(format!("Answer:{} Score: {} Start: {} End: {}",a.answer.as_str(),a.score,a.start,a.end).as_str());
            };   
        };
        result.push_str("</div>");
        Ok(result)
    }
}