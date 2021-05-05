use uuid::Uuid;

use crate::{net::client::Client, serde::play::serverbound::ClientSettings};

pub struct Player {
    client: Client,
    uuid: Uuid,
    client_settings: Option<ClientSettings>,
}

impl Player {
    pub fn new(client: Client, uuid: Uuid) -> Self {
        Self {
            client,
            uuid,
            client_settings: None,
        }
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
}
