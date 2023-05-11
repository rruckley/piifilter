// API into QDrant vector database
use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct QDrantPayload {
    pub phrase : Option<String>,
}

type QDrantVector = Vec<f32>;
#[derive(Debug,Serialize)]
pub struct QDrantPoint {
    pub id : String,
    pub payload : QDrantPayload,
    pub vector  : QDrantVector,
}

#[derive(Debug,Serialize)]
pub struct QDrantPoints {
    pub points : Vec<QDrantPoint>,
}

#[derive(Debug,Serialize)]
pub struct QDrantParams {
    hnsw_ef : i32,
    exact   : bool,
}

#[derive(Debug,Serialize)]
pub struct QDrantFilter {

}

#[derive(Debug,Serialize)]
pub struct QDrantSearch {
    filter  : QDrantFilter,
    params  : QDrantParams,
    vector  : QDrantVector,
    with_payload    : bool,
    limit   : i32,
}

#[derive(Debug,Deserialize)]
pub struct QDrantScoredVec {
    id  : String,
    score   : f32,
    payload : Option<QDrantPayload>,
}

#[derive(Debug,Deserialize)]
pub struct QDrantResponse {
    result : Vec<QDrantScoredVec>,
    status : String,
    time   : f32,
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

    pub async fn search(&self, vec : Vec<f32>) -> Result<Vec<String>,String> {
        let req = reqwest::Client::new();
        let url = format!("{}/collections/{}/points/search",self.instance,self.collection);
        let filter = QDrantSearch {
            filter : QDrantFilter {  },
            params : QDrantParams { hnsw_ef: 128, exact: false },
            vector : vec,
            with_payload : true,
            limit : 5,
        };
        let body = serde_json::to_string(&filter).expect("Could not parse QDrant response");
        match req.post(url).body(body).send().await {
            Ok(r) => {
                info!("Results from Qdrant!");
                let body = r.text().await.unwrap();
                let response : QDrantResponse = serde_json::from_str(body.as_str()).expect("Could not parse JSON");
                let mut output : Vec<String> = vec![];
                for v in response.result {
                    let phrase = match v.payload {
                        Some(p) => p.phrase,
                        None => Some("No phrase".to_string()),
                    };
                    output.push(format!("[{}] {}",v.score,phrase.unwrap()));
                }
                Ok(output)
            }
            Err(e) => {
                error!("Could not search DB: {e}");
                Err(e.to_string()) 
            },
        }
    }
}