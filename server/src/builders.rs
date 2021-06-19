use crate::shared_types::{ServerEvent, ServerEventCode, ServerEventData};

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

    pub fn build(&self) -> ServerEventData {
        self.server_event_data.clone()
    }
}
