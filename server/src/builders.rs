use crate::shared_types::{
    Card, GameData, PlayerData, ServerEvent, ServerEventCode, ServerEventData,
};

impl ServerEvent {
    pub fn builder(event_code: ServerEventCode) -> ServerEventBuilder {
        ServerEventBuilder::new(event_code)
    }
}
pub struct ServerEventBuilder {
    server_event: ServerEvent,
}

impl ServerEventBuilder {
    pub fn new(event_code: ServerEventCode) -> ServerEventBuilder {
        ServerEventBuilder {
            server_event: ServerEvent {
                event_code,
                data: None,
                message: None,
            },
        }
    }

    pub fn message(mut self, message: &str) -> ServerEventBuilder {
        self.server_event.message = Some(message.to_string());
        self
    }

    pub fn data(mut self, data: ServerEventData) -> ServerEventBuilder {
        self.server_event.data = Some(data);
        self
    }

    pub fn build(&self) -> ServerEvent {
        self.server_event.clone()
    }
}

impl ServerEventData {
    pub fn builder() -> ServerEventDataBuilder {
        ServerEventDataBuilder::new()
    }
}
pub struct ServerEventDataBuilder {
    server_event_data: ServerEventData,
}

impl ServerEventDataBuilder {
    pub fn new() -> ServerEventDataBuilder {
        ServerEventDataBuilder {
            server_event_data: ServerEventData::default(),
        }
    }

    pub fn client_id(mut self, client_id: &str) -> ServerEventDataBuilder {
        self.server_event_data.client_id = Some(client_id.to_string());
        self
    }

    pub fn session_id(mut self, session_id: &str) -> ServerEventDataBuilder {
        self.server_event_data.session_id = Some(session_id.to_string());
        self
    }

    pub fn session_client_ids(
        mut self,
        session_client_ids: &Vec<String>,
    ) -> ServerEventDataBuilder {
        self.server_event_data.session_client_ids = Some(session_client_ids.clone());
        self
    }

    pub fn card_options(mut self, card_options: &Vec<Card>) -> ServerEventDataBuilder {
        self.server_event_data.card_options = Some(card_options.clone());
        self
    }

    pub fn game_data(mut self, game_data: &GameData) -> ServerEventDataBuilder {
        self.server_event_data.game_data = Some(game_data.clone());
        self
    }

    pub fn player_data(mut self, player_data: &PlayerData) -> ServerEventDataBuilder {
        self.server_event_data.player_data = Some(player_data.clone());
        self
    }

    pub fn build(&self) -> ServerEventData {
        self.server_event_data.clone()
    }
}
