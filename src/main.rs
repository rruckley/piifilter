extern crate anyhow;

use rust_bert::pipelines::{ner::NERModel};
use rust_bert::pipelines::question_answering::{QaInput,QuestionAnsweringConfig, QuestionAnsweringModel};
use rust_bert::pipelines::pos_tagging::POSModel;
use rust_bert::resources::RemoteResource;
use rust_bert::longformer::{
    LongformerConfigResources, LongformerMergesResources, LongformerModelResources,
    LongformerVocabResources,
};
use rust_bert::pipelines::common::ModelType;

fn main() -> anyhow::Result<()> {

    /***
    let input = [
        "My name is Amy. I live in Paris.",
        "Paris is a city in France.",
        "My Medicare number is 0234903 and my email is ryan.ruckley@optus.com.au",
        "I want to cancel my phone 042349839, my passport is 2340934",
        "We have given soluiton walkthrough (Sukumar) and then Mona has shared the reasoning for the http used within the docker",
        "Richard Branson, founder of Virgin Galactic, is now offering manned space flights for as little as $200,000."
    ];

    for line in input {
        println!("Input: {}",line);
    }

    let ner_model = NERModel::new(Default::default())?;

    let output = ner_model.predict_full_entities(&input);

    for entity in output {
        println!("{:?}",entity);
    }
    ***/

    // Question / Answer section
    let config = QuestionAnsweringConfig::new(
        ModelType::Longformer,
        RemoteResource::from_pretrained(LongformerModelResources::LONGFORMER_BASE_SQUAD1),
        RemoteResource::from_pretrained(LongformerConfigResources::LONGFORMER_BASE_SQUAD1),
        RemoteResource::from_pretrained(LongformerVocabResources::LONGFORMER_BASE_SQUAD1),
        Some(RemoteResource::from_pretrained(
            LongformerMergesResources::LONGFORMER_BASE_SQUAD1,
        )),
        false,
        None,
        false,
    );

    let qa_model = QuestionAnsweringModel::new(config)?;

    let question = String::from("Where does Ryan live ?");
    let context = String::from("Ryan lives in Sydney, Australia");

    let context2 = String::from("Richard Branson, founder of Virgin Galactic, is now offering manned space flights for as little as $200,000.");
    let question2 = String::from("What is Virgin Galactic?");

    let answers = qa_model.predict(&[QaInput { question, context }, QaInput { question: question2,context:  context2}], 1, 32);
    println!("{:?}",answers);

    // PoS tagging
    let pos_model = POSModel::new(Default::default())?;

    let input = ["My name is Ryan Ruckley and I am an architect."];
    let output = pos_model.predict(&input);

    for (pos, pos_tag) in output[0].iter().enumerate() {
        println!("{} - {:?}",pos, pos_tag);
    }

    Ok(())
}
