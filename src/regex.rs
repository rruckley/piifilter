use rocket::form::prelude::NameView;

use crate::docs::{Document,DocType};

pub struct RegexFilter {
    docs : Vec<Document>,
}

impl RegexFilter {
    pub fn new() -> Self {
        let cpass = Document::new(DocType::CurrentPassport,70);
        let epass = Document::new(DocType::ExpiredPassport,50);
        let fpass = Document::new(DocType::ForeignPassport,50);
        let bcert = Document::new(DocType::BirthCertificate,70);
        let ccert = Document::new(DocType::CitizenCertificate, 50);
        let driver = Document::new(DocType::DriverLicense, 50);
        let medicare = Document::new(DocType::Medicare, 30);
        let docs = vec![cpass,epass,fpass,bcert,ccert,driver,medicare];
        Self {
            docs,
        }
    }

    /// Iterate through context looking for all document identifiers
    pub fn filter(&self, context : String) -> Result<String,String> {
        let mut output = context.clone();
        for doc in self.docs.iter() {
            // Look for regex matches
            let matches = doc.pattern.find_iter(context.as_str());
            for m in matches {
                //output.push_str(format!("{} : From {} to {}",doc.doc_type,m.start(),m.end()).as_str());
                let start = m.start() as usize;
                let end = m.end() as usize;
                let fill = "X".repeat(end-start);
                output.replace_range(start..end, &fill)
            }
        }
        
        Ok(output)
    }
}