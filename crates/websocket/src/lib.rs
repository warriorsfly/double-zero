
pub mod handlers;
pub mod messages;
pub mod session;
pub mod server;

// pub ROOM_NAME:&str = "room";

use actix_web::web::ServiceConfig;
use server::TransAmSystem;

pub fn config_trans_am_system(cfg:&mut ServiceConfig){
    let tas =  TransAmSystem{
        sessions: HashMap::new(),
        rooms: HashMap::new(),
        rng: rand::thread_rng(),
    };
    cfg.app_data(Data::new(tas));
}
