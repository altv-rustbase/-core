pub mod events;

use std::collections::HashMap;
use altv::BaseObjectPoolFuncs;

mod vitality;
use crate::modules::utils::Utils;
use self::vitality::Vitality;

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

    pub fn get(dynamic_id:String) -> &'static mut User {
        unsafe {
            let _dynamic_id = dynamic_id.to_string().parse::<usize>().unwrap();
            let _return_handle = PLAYERS_LIST[0].get_mut(&_dynamic_id).unwrap();
            _return_handle
        }
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