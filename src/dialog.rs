
use rust_bert::pipelines::conversation::{ConversationModel, ConversationManager};

use tokio::{sync::oneshot, task};
use std::collections::HashMap;
use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};

type Message = (Vec<String>, oneshot::Sender<HashMap<& 'static uuid::Uuid,& 'static str>>);
pub struct DialogFilter {
    sender: mpsc::SyncSender<Message>,
}

impl DialogFilter {
    pub fn spawn() -> (JoinHandle<Result<String,String>>, DialogFilter) {
        let (sender, receiver) = mpsc::sync_channel(100);
        let handle = thread::spawn(move || Self::runner(receiver));

        (handle, DialogFilter { sender })
    }
    fn runner(receiver: mpsc::Receiver<Message>) -> Result<String,String> {
        let conversation_model = ConversationModel::new(Default::default()).expect("Could not create dialog model");
        let mut conversation_manager = ConversationManager::new();

        let (context, _sender) = receiver.recv().unwrap();
            // Nothing to be done here for now
        let _conversation_id = conversation_manager.create(context.first().unwrap());
        

        Ok("Finished.".to_owned())
    }

    pub async fn filter(&self, context : String) -> Result<String,String> {
        let mut mangle = String::from(&context);
        let input = vec![context];

        let (sender, receiver) = oneshot::channel();
        task::block_in_place(|| self.sender.send((input, sender))).expect("Dialog:Could not spawn task");
        let output = receiver.await.expect("Dialog: Could not get message from thread");
        for (_id,conv )in output.iter() {
            mangle.push_str(conv);
        }
        mangle.replace_range(3..5, "xxx");
        Ok(mangle)
    }

}