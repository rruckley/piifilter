///! Document types
/// 
/// 

enum DocType {
    CurrentPassport,
    ExpiredPassport,
    BirthCertificate,
    CitizenCertificate,
    DriverLicense,
    ForeignPassport,
}

struct Document {
    docType : DocType,
    points : u32 ,
    regex : String,
}

impl Document {
    fn new(docType, points) -> Self {
        Self {
            docType,
            points,
        }
    }
}