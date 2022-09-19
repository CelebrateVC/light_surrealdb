use crate::verbs;
use crate::socket;
use std::collections::HashMap;


pub type Null = Option<String>;

pub struct SurrealDBSocketClient{
    interface: socket::ClientInterface
}

#[derive(serde::Deserialize, Debug)]
struct Response<T: std::fmt::Debug>{
    #[serde(rename = "id")]
    _id: String,
    result: T
}

#[derive(serde::Deserialize,Debug)]
pub struct QueryResponse<T>{
    pub result: Vec<RecordResponse<T>>,
    pub status: String,
    pub time: String
}

#[derive(serde::Deserialize, Debug)]
pub struct RecordResponse<T>{
    #[serde(flatten)]
    pub result: T,
    #[serde(flatten)]
    pub extra: HashMap<String,serde_json::Value>
}

#[derive(Debug)]
pub enum Error{
    WSSndErr(ws::Error),
    WSRcvErr(ws::Error),
    JSErr(serde_json::Error)
}

macro_rules! chain {
    ([$p:expr ,$ok:ident, $e:ident, $err:expr] $($rest:tt)*) 
    => {

        match $p{
            Ok($ok) => {chain!($($rest)*)},
            Err($e) => {
                Err($err)
            }
        } 
    };
    ([$prnt:ident=> $p:expr ,$ok:ident, $e:ident, $err:expr] $($rest:tt)*) 
    => {
        {

            {println!("{:?}",$prnt);}
            match $p{
                Ok($ok) => {
                    chain!($($rest)*)
                },
                Err($e) => {
                    Err($err)
                }
            } 
        }
    };
    ($b:expr) => {
        $b
    };
}

macro_rules! process {
    ($a:ident,$b:ident) => {
        chain!(
            [$a, _i, e, Error::WSSndErr(e)]
            $b.receive()
        )
    };
}


impl SurrealDBSocketClient{
    pub fn new(url: &str)-> Self{
        SurrealDBSocketClient { interface: socket::ClientInterface::new(url.to_owned()) }
    }



    fn receive<T: serde::de::DeserializeOwned+std::fmt::Debug>(&self)->Result<T,Error>{

         chain!(
            [self.interface.recv(),                         txt,     e, Error::WSRcvErr(e)]
            [txt=> serde_json::de::from_str::<Response<T>>(&txt), decoded, e, Error::JSErr(e)]
            Ok(decoded.result))

    }

    pub fn close(self){
        self.interface.close().unwrap();
    }

    pub fn ping(&self)->Result<bool,Error>{

        let send = self.interface.send(verbs::ping());

        process!(send,self)
    }
    pub fn live(&self, table_or_id: String)->Result<bool,Error>{

        let send = self.interface.send(verbs::live(table_or_id));

        process!(send,self)
    }
    pub fn kill(&self, id: String)->Result<bool,Error>{

        let send = self.interface.send(verbs::kill(id));

        process!(send,self)
    }
    pub fn info(&self)->Result<bool,Error>{

        let send = self.interface.send(verbs::info());

        process!(send,self)
    }
    pub fn use_ns(&self,ns:String,db:String)->Result<Null,Error>{
        let send = self.interface.send(verbs::r#use(ns, db));

        process!(send,self)
    }

    pub fn signup(&self,username:Option<String>,password:Option<String>,namespace:Option<String>,
                  database:Option<String>,email:Option<String>,scope: Option<String>,interests: Option<Vec<String>>
                ) -> Result<Null,Error>{
        let sign = verbs::SIGNUP{username,password,namespace,database,email,scope,interests};
        let send = self.interface.send(verbs::signup(sign));

        process!(send,self)
    }

    pub fn signin(&self,username:Option<String>,password:Option<String>,namespace:Option<String>,
                  database:Option<String>,email:Option<String>
                ) -> Result<String, Error>{
        let sign = verbs::LOGIN{username,password,namespace,database,email};
        let send = self.interface.send(verbs::signin(sign));

        process!(send,self)
    }

    pub fn invalidate(&self) -> Result<String, Error>{
        let send = self.interface.send(verbs::invalidate());
        process!(send,self)
    }

    pub fn authenticate(&self, auth: Option<String>)	 ->Result<String,Error>{
        let send = self.interface.send(verbs::authenticate(auth));
        process!(send,self)
    }

    pub fn set(&self, key: String, value: String)	 ->Result<Null,Error>{
        let send = self.interface.send(verbs::set(key, value));
        process!(send,self)
    }

    pub fn query<R: serde::Serialize, T: serde::de::DeserializeOwned+std::fmt::Debug>(&self, sql: String, replace: R)	 ->Result<Vec<QueryResponse<T>>,Error>
    {
        let send = self.interface.send(verbs::query(sql, replace));
        process!(send,self)

    }

    pub fn select<T: serde::de::DeserializeOwned+std::fmt::Debug>(&self, table_or_id: String)	 ->Result<Vec<RecordResponse<T>>,Error>{
        let send = self.interface.send(verbs::select(table_or_id));
        process!(send,self)

    }

    pub fn create<T: serde::Serialize+serde::de::DeserializeOwned+std::fmt::Debug>(&self, id:String, data: Option<T>) ->Result<[RecordResponse<T>;1],Error>{
        let send = self.interface.send(verbs::create(id, data));
        process!(send,self)
    }

    pub fn update<T: serde::Serialize+serde::de::DeserializeOwned+std::fmt::Debug>(&self, id:String, data:Option<T>)	 ->Result<Vec<RecordResponse<T>>,Error>{
        let send = self.interface.send(verbs::update(id, data));
        process!(send,self)
    }



    pub fn change<T: serde::Serialize+serde::de::DeserializeOwned+std::fmt::Debug>(&self, id: String, data: Option<T>)	 ->Result<Vec<RecordResponse<T>>,Error>{
        let send = self.interface.send(verbs::change(id, data));
        process!(send,self)
    }

    pub fn modify<T: serde::Serialize+serde::de::DeserializeOwned+std::fmt::Debug>(&self, id: String, data: Option<T>)	 ->Result<RecordResponse<T>,Error>{
        let send = self.interface.send(verbs::modify(id, data));
        process!(send,self)
    }

    pub fn delete(&self, id: String) ->Result<[String;0],Error>{
        let send = self.interface.send(verbs::delete(id));
        process!(send,self)
    }







}
