type ID = String;

#[derive(serde::Serialize)]
pub struct SIGNUP{
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "user"))]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "pass"))]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "NS"))]
    pub namespace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "DB"))]
    pub database: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "SC"))]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interests: Option<Vec<String>>
}

#[derive(serde::Serialize)]
pub struct LOGIN{
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "user"))]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "pass"))]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "NS"))]
    pub namespace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "DB"))]
    pub database: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

fn generic(verb:&str,arg:&str) -> String {
    let ret = "{\"method\":\"".to_owned()+verb+"\",\"params\":"+arg+",\"id\":\""+&uuid::Uuid::new_v4().simple().to_string()+"\"}";
    println!("websocket message [OUT]: {}",ret);
    ret
}

macro_rules! define_verb {

    (
        $func_name:ident,
        $common:expr
    ) => {
        pub fn $func_name()-> String{
            let pre: [String;0] = [];
            let arg = &serde_json::to_string(&pre).unwrap();
            generic($common,arg)
        }
    };

    (
        $func_name:ident,
        $common:expr,
        $($v:ident: $t:ty),*
    ) => {
        pub fn $func_name($($v: $t),*)-> String{
            let pre = [$(serde_json::to_value($v).unwrap()),*,];
            let arg = &serde_json::to_string(&pre).unwrap();
            generic($common,arg)
        }
    };  
}

macro_rules! typed_verb_def {
    (
        $func_name:ident,
        $common:expr,
        $($v:ident: $t:ty),+
    ) => {
        pub fn $func_name<T: serde::Serialize>($($v: $t),*)-> String{
            let pre = [$(serde_json::to_value($v).unwrap()),*,];
            let arg = &serde_json::to_string(&pre).unwrap();
            generic($common,arg)
        }
    };  
}


define_verb!(ping,"ping");
define_verb!(info,"info");
define_verb!(r#use,"use", ns: String,db: String);
define_verb!(signup,"signup", sign: SIGNUP);
define_verb!(signin,"signin", login:LOGIN);
define_verb!(invalidate,"invalidate");
define_verb!(authenticate,"authenticate",auth: Option<String>);
define_verb!(kill,"kill",id:ID);
define_verb!(live, "live", table: String);
define_verb!(set, "set", key: String, value: String);
typed_verb_def!(query,"query",sql:String,replace:T);
define_verb!(select,"select",id:ID);
typed_verb_def!(create,"create",id:ID,data: Option<T>);
typed_verb_def!(update,"update",id:ID,data: Option<T>);
typed_verb_def!(change,"change",id:ID,data: Option<T>);
typed_verb_def!(modify,"modify",id:ID,data: Option<T>);
define_verb!(delete,"delete",id:ID);

