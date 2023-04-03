
use rust_bert::pipelines::{ner::NERModel, ner::Entity};
use tokio::{sync::oneshot, task};


use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};

type Message = (Vec<String>, oneshot::Sender<Vec<Vec<Entity>>>);
#[derive(Debug)]
pub struct NERFilter {
    sender: mpsc::SyncSender<Message>,
}

impl NERFilter {
    pub fn spawn() -> (JoinHandle<Result<String,String>>, NERFilter) {
        let (sender, receiver) = mpsc::sync_channel(100);
        let handle = thread::spawn(move || Self::runner(receiver));

        (handle, NERFilter { sender })
    }
    fn runner(receiver: mpsc::Receiver<Message>) -> Result<String,String> {
        let ner_model = NERModel::new(Default::default()).expect("Could not create NER model");

        while let Ok((context, sender)) = receiver.recv() {
            let input = [context.first().unwrap().as_str()];
            let output = ner_model.predict_full_entities(&input);
            let _result = sender.send(output.to_owned());
        }

        Ok("Finished.".to_owned())
    }
    pub fn get_style() -> String {
        "
        <style type=\"text/css\">
        </style>
        ".to_owned()
    }

    pub async fn filter(&self, context : String) -> Result<String,String> {
        let mut mangle = String::from(&context);
        let input = vec![context];

        let (sender, receiver) = oneshot::channel();
        task::block_in_place(|| self.sender.send((input, sender))).expect("NER:Could not spawn task");
        let output = receiver.await.expect("NER: Could not get message from thread");
        
        let mut ev = vec![];
        for row in output {
            for t in row {
                ev.push(t);
            };
        }
        for e in ev.iter().rev() {
            let start = e.offset.begin as usize;
            let finish = e.offset.end as usize;

            let span = format!("<span class=\"{}\" start=\"{}\" finish=\"{}\">{}</span>",e.label,start,finish,e.word);
            mangle.replace_range(start..finish, span.as_str());
        } 
        let result =format!("<div class=\"ner\">{}</div>",mangle);
        Ok(result)
    }
}