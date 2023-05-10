// API into QDrant vector database
use serde::Serialize;

#[derive(Debug,Serialize)]
pub struct QDrantPayload {

}
#[derive(Debug,Serialize)]
pub struct QDrantPoint {
    pub id : String,
    pub payload : QDrantPayload,
    pub vector  : Vec<f32>,
}

#[derive(Debug,Serialize)]
pub struct QDrantPoints {
    pub points : Vec<QDrantPoint>,
}

#[derive(Debug)]
pub struct QDrant {
    pub instance : String,
    pub collection : String,
}

impl QDrant {
    pub fn new(instance : String,collection : String) -> Self {
        Self {
            instance,
            collection,
        }
    }
    pub async fn store(&self,points : QDrantPoints) -> Result<String,String> {
        let count = points.points.len();
        info!("Storing {count} points");
        let req = reqwest::Client::new();
        let body = serde_json::to_string(&points).unwrap();
        let url = format!("{}/collections/{}/points",self.instance,self.collection);
        match req.put(url).body(body).send().await {
            Ok(r) => {
                Ok(r.text().await.unwrap())
            },
            Err(e) => {
                error!("Could not save points: {e}");
                Err(e.to_string())
            },
        }
    }
}