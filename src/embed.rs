/// Sentence Embeddings
/// 

use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder,SentenceEmbeddingsModelType};
use tokio::{sync::oneshot, task};
use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};

type Message = (Vec<String>, oneshot::Sender<Vec<Vec<f32>>>);
pub struct EmbedFilter {
    sender: mpsc::SyncSender<Message>,
}

impl EmbedFilter {
    pub async fn filter(&self, sentences : Vec<String>) -> Result<Vec<Vec<f32>>,String> {
        let (sender, receiver) = oneshot::channel();

        task::block_in_place(|| self.sender.send((sentences, sender))).expect("Ember: Could not spawn task");
        
        let output = receiver.await.expect("Embed: Could not get message from thread");

        
        Ok(output)
    }

    pub fn spawn() -> (JoinHandle<Result<String,String>>, EmbedFilter) {
        let (sender, receiver) = mpsc::sync_channel(100);
        let handle = thread::spawn(move || Self::runner(receiver));

        (handle, EmbedFilter { sender })
    }

    fn runner(receiver: mpsc::Receiver<Message>) -> Result<String,String> {
        let model = SentenceEmbeddingsBuilder::remote(
            SentenceEmbeddingsModelType::AllMiniLmL12V2
        )
        .create_model().map_err(|e| e.to_string()).expect("Could not create model");

        while let Ok((context, sender)) = receiver.recv() {
            //let output = ner_model.predict_full_entities(&input);
            let output = model.encode(context.as_slice()).map_err(|e| e.to_string()).unwrap();
            let _result = sender.send(output.to_owned());
        }
        // Generate embeddings for all returned sentences

        Ok("Finished.".to_owned())
    }}