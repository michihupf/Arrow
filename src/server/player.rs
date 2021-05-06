use uuid::Uuid;
use log::debug;

use crate::{
    net::{
        client::Client,
        error::NetError
    },
    serde::play::{
        serverbound::ClientSettings,
        clientbound::{
            HeldItemChange, PlayerAbilities
        }
    }
};

pub struct Player {
    client: Client,
    uuid: Uuid,
    client_settings: Option<ClientSettings>,
    player_abilities: PlayerAbilities,
    active_slot: i8,
}

impl Player {
    pub fn new(client: Client, uuid: Uuid) -> Self {
        Self { client, uuid, client_settings: None, active_slot: 0, player_abilities: PlayerAbilities::default() }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn client_mut(&mut self) -> &mut Client {
        &mut self.client
    }

    pub fn client_settings(&self) -> &Option<ClientSettings> {
        &self.client_settings
    }

    pub fn set_client_settings(&mut self, client_settings: ClientSettings) {
        self.client_settings = Some(client_settings);
    }

    pub async fn set_player_abilites(&mut self, player_abilities: PlayerAbilities) -> Result<(), NetError> {
        self.player_abilities = player_abilities;

        self.client_mut()
            .send_packet(0x30, player_abilities)
            .await
    }

    pub async fn set_active_slot(&mut self, slot: i8) -> Result<(), NetError> {
        debug!("Called `set_active_slot` in `player.rs`");
        self.active_slot = slot;
        let held_item_change = HeldItemChange {
            slot
        };

        let response = self.client_mut()
            .send_packet(0x3F, held_item_change)
            .await;
        debug!("Sent a packet with id 0x3F");
        response
    }
}
