use crate::constants::MAX_PLAYERS;
use crc32fast::Hasher;
use ggpo::{ggpo::Event, player::Player};
use std::net::{IpAddr, SocketAddr};

pub const FRAME_DELAY: usize = 2;
// TODO: Synctest during testing

enum Inputs {
    Thrust,
    Break,
    Rotate_Left,
    Rotate_Right,
    Fire,
    Bomb,
}

pub fn init(local_port: u16, num_players: i32, players: &[Player], num_spectators: i32) {
    unimplemented!()
}

pub fn init_spectator(local_port: u16, num_players: i32, host_ip: IpAddr, host_port: u16) {
    unimplemented!()
}

pub fn draw_current_frame() {
    unimplemented!()
}

pub fn advance_frame(inputs: Vec<i32>, disconnect_flags: i32) {
    unimplemented!()
}

pub fn run_frame(input: i32) -> i32 {
    unimplemented!()
}

pub fn idle(time: i32) {}

pub fn disconnect_player(player: i32) {}

pub fn exit() {}

/*
 * The begin game callback.  We don't need to do anything special here,
 * so just return true.
 */
// TODO: Deprecated, remove later
fn begin_game_callback(name: &str) -> bool {
    true
}

/*
 * Notification from GGPO that something has happened.  Update the status
 * text at the bottom of the screen to notify the user.
 */
fn on_event_callback(info: &Event) -> bool {
    let progress: i32;
    // match info {
    //     Event::ConnectedToPeer =>
    // }
    unimplemented!()
}
