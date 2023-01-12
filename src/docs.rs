///! Document types
/// 
/// 

pub enum DocType {
    CurrentPassport,
    ExpiredPassport,
    BirthCertificate,
    CitizenCertificate,
    DriverLicense,
    ForeignPassport,
}

pub struct Document {
    doc_type : DocType,
    points : u32 ,
    regex : String,
}

impl Document {
    pub fn new(doc_type : DocType, points : u32) -> Self {
        Self {
            doc_type,
            points,
            regex: "sample".to_string(),
        }
    }
}