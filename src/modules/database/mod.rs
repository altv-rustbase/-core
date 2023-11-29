use std::cell::RefCell;
use mongodb::{Client, options::ClientOptions, Collection};
use crate::modules::utils::terminal::Terminal;

// CODE

pub const DATABASE_NAME:&str = "rustbase";

thread_local! {
    pub static DATABASE_HANDLE:RefCell<Option<Client>> = RefCell::new(None);
}

pub struct DataBase {}
impl DataBase {
    pub async fn _load(host:&str, username:&str, password:&str, name:&str) {
        Terminal::debug_detailed("[DataBase] _load();");
        let mut _string_connect:String = String::new();
        
        if host == "localhost" {
            _string_connect = "mongodb://localhost:27017/rustbase".to_string();
        } else {
            let _mongo_str = format!("mongodb+srv://{:?}:{:?}@{:?}.mongodb.net/{:?}", username, password, host, name);
            _string_connect = _mongo_str;
        }

        let mut _options = ClientOptions::parse_async(_string_connect).await;
        let _client = Client::with_options(_options.unwrap()).unwrap();
        DATABASE_HANDLE.set(Some(_client));

        Terminal::done("[DataBase] Connection is successful!");
    }

    pub async fn create_table<T>(name:&str) -> Collection<T> {
        Terminal::debug_detailed(format!("[DataBase] Table `{name}` is init").as_str());
        
        let _handle = DATABASE_HANDLE.take().clone().unwrap();
        let _db = _handle.database(DATABASE_NAME);
        _db.create_collection(name, None).await.unwrap();

        let _table = _db.collection(&name);
        return _table;
    }
}