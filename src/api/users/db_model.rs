use mongodb::bson::{Document, doc, oid::ObjectId};
use serde_derive::{Serialize, Deserialize};
use crate::modules::database::{DataBase, DATABASE_HANDLE, DATABASE_NAME};

// CODE

const TABLE_NAME:&str = "accounts";

#[derive(Serialize, Deserialize)]
#[serde(rename = "_id")]
struct Schema {
    pub id:ObjectId,
    pub test:bool
}

pub struct MAccounts {}
impl MAccounts {
    pub async fn _init() {
        DataBase::create_table::<MAccounts>(TABLE_NAME).await;
    }

    pub async fn create_document(document:Document) {
        unsafe {
            let _db_handle = DATABASE_HANDLE.clone().unwrap();
            let _table_handle = _db_handle.database(DATABASE_NAME).collection::<Schema>(TABLE_NAME);

            let _doc = doc!{
                "test": false
            };
            
            _table_handle.insert_one(_doc, None).await.unwrap();
        }
    }
}