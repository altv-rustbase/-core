pub mod events;
use std::collections::HashMap;

// CODE

static mut PLAYERS_LIST:Vec<HashMap<usize, User>> = Vec::new();

pub struct Users {}
impl Users {
    pub fn _init() {
        unsafe { PLAYERS_LIST.push(HashMap::new()); }
    }

    pub fn add(dynamic_id:String, user:User) {
        unsafe {
            let _dynamic_id = dynamic_id.to_string().parse::<usize>().unwrap();
            PLAYERS_LIST[0].insert(_dynamic_id, user);
        }
    }

    pub fn remove(dynamic_id:String) {
        unsafe {
            let _dynamic_id = dynamic_id.to_string().parse::<usize>().unwrap();
            PLAYERS_LIST[0].remove(&_dynamic_id);
        }
    }

    pub fn get(dynamic_id:String) -> &'static User {
        unsafe {
            let _dynamic_id = dynamic_id.to_string().parse::<usize>().unwrap();
            let _return_handle = PLAYERS_LIST[0].get_mut(&_dynamic_id).unwrap();
            _return_handle
        }
    }
}

pub struct User {
    pub dynamic_id:String
}

impl User {
    pub fn new(
        dynamic_id:&str
    ) -> User {
        User {
            dynamic_id: dynamic_id.to_string()
        }
    }
}