use reqwest;
// Working from: https://blog.logrocket.com/making-http-requests-rust-reqwest/
async fn main(){
    let endpoint = "https://api.seer.cancer.gov/rest/disease/latest/"
    let SEER_API_SPEC = "X-SEERAPI-Key"
    let key = "" // TODO: read key from python module
    let client = reqwest::Client::new();

    let response = client
    .get(endpoint)
    .header(SEER_API_SPEC, key)
    .send()
    .await;

    match response.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to an APIResponse
            match response.json::<APIResponse>().await {
                Ok(parsed) => println!("Success! {:?}", parsed),
                Err(_) => println!("Hm, the response didn't match the shape we expected."),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };
}   
