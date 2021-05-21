use uuid::Uuid;

use crate::client::Client;

pub struct Player {
    uuid: Uuid,
    name: String,
    client: Client,
}

impl Player {
    pub fn new(uuid: Uuid, name: String, client: Client) -> Self {
        Self { uuid, name, client }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn client_mut(&mut self) -> &mut Client {
        &mut self.client
    }

    pub async fn recv(&mut self) {
        self.client.recv().await;
    }
}
