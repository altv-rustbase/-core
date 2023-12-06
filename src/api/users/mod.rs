pub mod events;
// pub mod db_model;

use std::{collections::HashMap, cell::RefCell};
use altv::BaseObjectPoolFuncs;

mod vitality;
use crate::modules::utils::{Utils, terminal::Terminal};
use self::vitality::Vitality;

// CODE

thread_local! {
    pub static PLAYERS_LIST:RefCell<HashMap<usize, User>> = RefCell::new(HashMap::new());
}

pub struct Users {}
impl Users {
    pub fn has(dynamic_id:&String) -> bool {
        let _local_list = PLAYERS_LIST.take();
        let _dynamic_id = dynamic_id.to_string().parse::<usize>().unwrap();

        if let Some(_user) = _local_list.get(&_dynamic_id) { true }
        else { false }
    }

    pub fn add(dynamic_id:&String, user:User) {
        Terminal::debug_detailed(format!("[Users List] add({});", dynamic_id).as_str());
        let _dynamic_id = dynamic_id.to_string().parse::<usize>().unwrap();

        if !Users::has(dynamic_id) {
            PLAYERS_LIST.with(|_list| {
                let mut _local_list = _list.borrow_mut();
                _local_list.insert(_dynamic_id, user);
            });
        } else { Terminal::error(format!("[Users List] It is impossible to add a player with ID {} to the list because he already exists", dynamic_id).as_str()); }
    }

    pub fn remove(dynamic_id:&String) {
        Terminal::debug_detailed(format!("[Users List] remove({});", dynamic_id).as_str());
        let _dynamic_id = dynamic_id.to_string().parse::<usize>().unwrap();

        if Users::has(dynamic_id) {
            PLAYERS_LIST.with(|_list| {
                let mut _local_list = _list.borrow_mut();
                _local_list.remove(&_dynamic_id);
            });
        } else { Terminal::error(format!("[Users List] Cannot delete player with ID {} because he is not in the list", dynamic_id).as_str()); }
    }

    // pub fn get(dynamic_id:&String) -> Result<User, bool> {
    //     Terminal::debug_detailed(format!("[Users List] get({});", dynamic_id).as_str());
    //     let _dynamic_id = dynamic_id.to_string().parse::<usize>().unwrap();

    //     PLAYERS_LIST.with(|_list| {
    //         let mut _local_list = _list.borrow_mut();
            
    //         if let Some(_user) = _local_list.remove(&_dynamic_id) {
    //             Ok(_user)
    //         } else { Err(false) }
    //     })
    // }
}

// impl Users {
//     pub async fn _init() {
//         // MAccounts::_init().await;
//     }

//     fn has(dynamic_id:usize) -> bool {
//         if let Some(_user) = PLAYERS_LIST.take().get(&dynamic_id) { true }
//         else { false }
//     }

//     pub fn add(dynamic_id:String, user:User) {
//         let _dynamic_id = dynamic_id.to_string().parse::<usize>().unwrap();

//         if Users::has(_dynamic_id) == false {
//             Terminal::debug_detailed(format!("Player with ID {} added to users list", _dynamic_id).as_str());
//             PLAYERS_LIST.take().insert(_dynamic_id, user);
//         } else { Terminal::error(format!("Cannot add player with ID {} because he is already on the users list", dynamic_id).as_str()); }
//     }

//     pub fn remove(dynamic_id:String) {
//         let _dynamic_id = dynamic_id.to_string().parse::<usize>().unwrap();
        
//         if Users::has(_dynamic_id) {
//             Terminal::debug_detailed(format!("Player with ID {} removed from users list", _dynamic_id).as_str());
//             PLAYERS_LIST.take().remove(&_dynamic_id);
//         } else { Terminal::error(format!("Player with ID {} not found in users list", _dynamic_id).as_str()); }
//     }

//     // pub fn get(dynamic_id:String) -> Result<User, String> {
//     //     let _dynamic_id = dynamic_id.to_string().parse::<usize>().unwrap();
        
//     //     if Users::has(_dynamic_id) {
//     //         let _local_list = PLAYERS_LIST.take();
//     //         let _user = _local_list.get(&_dynamic_id).clone().unwrap();
//     //         Ok(_user.clone())
//     //     } else {
//     //         Err(format!("Player with ID {} not found in users list", dynamic_id))
//     //     }
//     // }
// }

#[derive(Clone)]
pub struct User {
    pub dynamic_id:String,
    pub health:Vitality,
    pub armour:Vitality
}

impl User {
    pub fn new(
        dynamic_id:&str
    ) -> User {
        User {
            dynamic_id: dynamic_id.to_string(),
            health: Vitality::new(100, 0, 100),
            armour: Vitality::new(50, 0, 100)
        }
    }

    pub fn _sync(&mut self) {
        let mut _players_all = altv::Player::all();

        let _dynamic_id = self.dynamic_id.to_string().parse::<usize>().unwrap();
        let _local_player = _players_all.get_mut(_dynamic_id).unwrap();

        let _local_health = Utils::convert_to::<u16>(self.health.get().to_string()).unwrap();
        _local_player.set_health(_local_health + 100).unwrap();

        let _local_armour = Utils::convert_to::<u16>(self.armour.get().to_string()).unwrap();
        _local_player.set_health(_local_armour).unwrap();
    }

    // SETTERS

    // GETTERS

    // OTHERS
}