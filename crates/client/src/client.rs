use std::net::{Ipv6Addr, SocketAddrV6};

use game::{
    ecs::systems::rendering::rendering_system,
    game::{state::State, texture_manager::TextureManager},
};
use macroquad::prelude::*;
use protocol::{
    packet::{MAX_DATAGRAM_SIZE, Packet},
    packets::ping::PacketPing,
    packets::spawn_entity::PacketSpawnEntity,
};
use tokio::{
    net::UdpSocket,
    sync::mpsc::{Receiver, Sender},
};

use crate::spawn_network_entity::spawn_network_entity;

pub struct Client {}
