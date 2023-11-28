use crate::utils::{terminal::Terminal, Utils};
use super::{User, Users};
use inline_colorization::*;
use crate::color_gray;


// CODE

pub struct UsersEvents {}
impl UsersEvents {
    pub fn _register() {
        altv::events::on_player_connect(|event| {
            let _dynamic_id = event.player.id().unwrap().to_string();
    
            let _local_user = User::new(_dynamic_id.as_str());
            Users::add(_dynamic_id.to_string(), _local_user);

            let _player_name = event.player.name().unwrap();
            let _player_ip = Utils::get_normalize_ip(event.player.ip().unwrap().to_string().as_str());

            Terminal::info(format!("Player {color_yellow}{_player_name} {color_gray}({_player_ip}) {color_reset}connected to the server.").as_str());
        });

        altv::events::on_player_disconnect(|event| {
            let _dynamic_id = event.player.id().unwrap().to_string();
            Users::remove(_dynamic_id);

            let _player_name = event.player.name().unwrap();
            Terminal::info(format!("Player {color_yellow}{_player_name} {color_reset}disconnected from server.").as_str());
        });
    }
}