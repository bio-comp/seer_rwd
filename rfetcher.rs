use reqwest;

async fn main(){
    let endpoint = "https://api.seer.cancer.gov/rest/disease/latest/"
    let SEER_API_SPEC = "X-SEERAPI-Key"
    let key = ""
    let client = reqwest::Client::new();

    let response = client
    .get(endpoint)
    .header(SEER_API_SPEC, key)
    .send()
}   
