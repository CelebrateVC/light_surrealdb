
pub use reqwest::Error;


#[derive(serde::Deserialize,Debug)]
struct QueryResponse<T>{
    time: String,
    status: String,
    result: T
}

#[derive(serde::Deserialize,Debug,Clone)]
struct DataResponse<T>{
    #[serde(flatten)]
    data: T,
    #[serde(flatten)]
    extra: std::collections::HashMap<String,serde_json::Value>
}

pub async fn runquery< T: serde::de::DeserializeOwned+std::fmt::Debug+Clone>(query_string: String) -> Result<T,reqwest::Error>{
    let cli = reqwest::Client::new();
    let post = cli.post("http://localhost:8000/sql")
        .header("NS", "myapp")
        .header("DB", "myapp")
        .basic_auth("root", Some("root"))
        .header("Content-Type","application/json")
        .body(query_string)
        .send()
        .await?
        .json::<Vec<QueryResponse<T>>>()
        .await?;
    
    println!("body = {:?}", post);
    Ok(post[0].result.clone())
}


pub async fn run_create< T: serde::de::DeserializeOwned+std::fmt::Debug+Clone>(query_string: String)
 -> Result<Vec<T>,reqwest::Error>
//  -> Result<(),reqwest::Error>
 {
    let cli = reqwest::Client::new();
    let post = cli.post("http://localhost:8000/sql")
        .header("NS", "myapp")
        .header("DB", "myapp")
        .basic_auth("root", Some("root"))
        .header("Content-Type","application/json")
        .body(query_string)
        .send()
        .await?
        .json::<[QueryResponse<Vec<DataResponse<T>>>; 1]>()
        // .text()
        .await?;
    
    println!("body = {:?}", post);
    Ok(post[0].result.clone().into_iter().map(|x|{x.data}).collect())
    // Ok(())
}