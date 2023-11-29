pub mod events;
// pub mod db_model;

use std::{collections::HashMap, cell::RefCell};
use altv::BaseObjectPoolFuncs;

mod vitality;
use crate::modules::utils::Utils;
use self::vitality::Vitality;

// CODE

thread_local! {
    pub static PLAYERS_LIST:RefCell<HashMap<usize, User>> = RefCell::new(HashMap::new());
}

pub struct Users {}
impl Users {
    pub async fn _init() {
        // MAccounts::_init().await;
    }

    pub fn add(dynamic_id:String, user:User) {
        let _dynamic_id = dynamic_id.to_string().parse::<usize>().unwrap();
        let mut _local_players_list = PLAYERS_LIST.take();
        _local_players_list.insert(_dynamic_id, user);
    }

    pub fn remove(dynamic_id:String) {
        let _dynamic_id = dynamic_id.to_string().parse::<usize>().unwrap();
        let mut _local_players_list = PLAYERS_LIST.take();
        _local_players_list.remove(&_dynamic_id);
    }

    pub fn get(dynamic_id:String) -> User {
        let _dynamic_id = dynamic_id.to_string().parse::<usize>().unwrap();
        let mut _local_players_list = PLAYERS_LIST.take();
        let _return_handle = _local_players_list.remove(&_dynamic_id).unwrap();
        _return_handle
    }
}

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
            armour: Vitality::new(0, 0, 100)
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