use crate::modules::utils::{terminal::Terminal, Utils};
use super::{User, Users, PLAYERS_LIST};
use altv::Vector3;
use inline_colorization::*;
use crate::color_gray;

// CODE

pub struct UsersEvents {}
impl UsersEvents {
    pub fn _register() {
        Terminal::debug_detailed("[UsersEvents] _register();");

        //? player connect
        altv::events::on_player_connect(|event| {
            let _dynamic_id = event.player.id().unwrap().to_string();
    
            if !Users::has(&_dynamic_id) {
                let _player = User::new(_dynamic_id.as_str());
                Users::add(&_dynamic_id, _player);
            }

            let _player_name = event.player.name().unwrap();
            let _player_ip = Utils::get_normalize_ip(event.player.ip().unwrap().to_string().as_str());
            Terminal::info(format!("Player {color_yellow}{_player_name} {color_gray}({_player_ip}){color_reset} connected to the server.").as_str());
        });

        //? player disconnect
        altv::events::on_player_disconnect(|event| {
            let _dynamic_id = event.player.id().unwrap().to_string();
            Users::remove(&_dynamic_id);

            let _player_name = event.player.name().unwrap();
            Terminal::info(format!("Player {color_yellow}{_player_name}{color_reset} disconnected from server.").as_str());
        });

        //? for delete
        altv::events::on_player_connect(|event| {
            let _player = &event.player;
            _player.spawn(
                altv::hash("MP_M_Freemode_01"),
                Vector3::new(0, 0, 71)
            ).unwrap();

            let _veh = altv::Vehicle::new(
                altv::hash("oracle"),
                Vector3::new(15, 15, 70),
                Vector3::default()
            ).unwrap();
        });
    }
}