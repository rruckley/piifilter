///! Document types
/// 
/// 
/// 
/// 
use regex::Regex;

#[derive(Debug)]
pub enum DocType {
    CurrentPassport,
    ExpiredPassport,
    BirthCertificate,
    CitizenCertificate,
    DriverLicense,
    ForeignPassport,
    Medicare,
}

impl std::fmt::Display for DocType {
    fn fmt(&self, f: & mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CurrentPassport => write!(f,"Passport"),
            Self::ExpiredPassport => write!(f,"Expired Passport"),
            Self::BirthCertificate => write!(f,"Birth Certificate"),
            Self::CitizenCertificate => write!(f,"Citizen Certificate"),
            Self::DriverLicense => write!(f,"Drivers License"),
            Self::ForeignPassport => write!(f,"Foreign Passport"),
            Self::Medicare => write!(f,"Medicare Cart"),

        }
    }
}

pub struct Document {
    pub doc_type : DocType,
    pub points : u32 ,
    pub pattern : regex::Regex,
}

impl Document {
    pub fn new(doc_type : DocType, points : u32) -> Self {
        let pattern = Document::get_regex(&doc_type);
        Self {
            doc_type,
            points,
            pattern,
        }
    }
    fn get_regex(doc_type: &DocType) -> regex::Regex {
        match doc_type {
            DocType::CurrentPassport => Regex::new("[A-Z][A-Z]?\\d{7}").unwrap(),
            DocType::ExpiredPassport => Regex::new("EPASS").unwrap(),
            DocType::BirthCertificate => Regex::new("BCERT").unwrap(),
            DocType::CitizenCertificate => Regex::new("CCERT").unwrap(),
            DocType::DriverLicense => Regex::new("DRIVER").unwrap(),
            DocType::ForeignPassport  => Regex::new("FPASS").unwrap(),
            DocType::Medicare => Regex::new("\\d{4}-\\d{5}-\\d").unwrap(),
        }
    }
}