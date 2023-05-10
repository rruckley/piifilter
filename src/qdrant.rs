// API into QDrant vector database

#[derive(Debug)]
pub struct QDrantPoint {
    pub id : i64,
    pub payload : String,
    pub vector  : Vec<f32>,
}

#[derive(Debug)]
pub struct QDrantPoints {
    pub points : Vec<QDrantPoint>,
}

#[derive(Debug)]
pub struct QDrant {
    pub instance : String,
}

impl QDrant {
    pub fn store(points : QDrantPoints) -> Result<String,String> {
        let count = points.points.len();
        info!("Storing {count} points");
        Ok("Not implemented".to_string())
    }
}