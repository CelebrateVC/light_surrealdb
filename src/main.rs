
fn main(){
    let client = surrealdb_client::SurrealDBSocketClient::new("ws://localhost:8000/rpc");
    println!("ping: {}",client.ping().expect("ping failed"));
    println!("signin: {:?}",client.signin(Some("root".to_owned()),
                                        Some("root".to_owned()),
                                        None,
                                        None,
                                        None
                       ).expect("login error")
    );
    match client.use_ns("myapp".to_owned(), "myapp".to_owned()){
        Ok(x) =>{println!("use: {:?}",x)},
        Err(e)=>{println!("use error:{:?}",e)}
    }
    print!("GET: {:?}", client.select::<Account>("account".to_owned()).expect("get error"));
    client.close();
}


#[derive(serde::Deserialize,Debug,Clone)]
struct Empty{}


#[derive(serde::Serialize,serde::Deserialize, Debug,Clone)]
struct Account{
    // id: String,
    name: String,
    created_at: String
}