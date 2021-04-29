use uuid::Uuid;

use crate::net::client::Client;

pub struct Player {
    client: Client,
    uuid: Uuid,
}

impl Player {
    pub fn new(client: Client, uuid: Uuid) -> Self {
        Self { client, uuid }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn client_mut(&mut self) -> &mut Client {
        &mut self.client
    }
}
