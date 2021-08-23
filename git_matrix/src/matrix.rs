use futures_util::stream::TryStreamExt;
use ruma_client::{
    api::r0,
    events::EventType,
    identifiers::{RoomAliasId, RoomId, UserId},
    HttpsClient,
};
use std::convert::TryFrom;

use crate::error::Error;

pub use ruma_client::Session;

mod events;

pub struct Builder {
    url: String,
    room: String,
}

impl Builder {
    pub fn new(url: String, room: String) -> Self {
        Builder { url, room }
    }

    pub async fn guest(&self) -> Result<(Matrix, Session), Error> {
        let client = create_client(&self.url, None)?;
        let session = client.register_guest().await?;
        let room_id = self.resolve_room_alias(&client).await?;

        Ok((Matrix { client, room_id }, session))
    }

    pub async fn session(
        self,
        username: &str,
        access_token: String,
        device_id: String,
    ) -> Result<Matrix, Error> {
        let client = create_client(
            &self.url,
            Some(Session {
                access_token,
                user_id: UserId::try_from(username).unwrap(),
                device_id,
            }),
        )?;

        let room_id = self.resolve_room_alias(&client).await?;

        Ok(Matrix { client, room_id })
    }

    async fn resolve_room_alias(&self, client: &HttpsClient) -> Result<RoomId, Error> {
        let response = client
            .request(r0::alias::get_alias::Request {
                room_alias: RoomAliasId::try_from(&self.room[..])?,
            })
            .await?;

        Ok(response.room_id)
    }
}

pub fn create_client(url: &str, session: Option<Session>) -> Result<HttpsClient, Error> {
    Ok(HttpsClient::https(url.parse()?, session))
}

#[derive(Clone)]
pub struct Matrix {
    client: HttpsClient,
    room_id: RoomId,
}

impl Matrix {
    pub async fn send_custom_event(
        &self,
        event_type: &str,
        content: serde_json::Value,
    ) -> Result<(), ruma_client::Error> {
        self.client
            .request(events::custom::Request {
                room_id: self.room_id.clone(),
                event_type: EventType::Custom(event_type.to_owned()),
                txn_id: uuid::Uuid::new_v4().to_simple().to_string(),
                content,
            })
            .await?;

        Ok(())
    }

    pub async fn create_content(
        &self,
        filename: &str,
        content_type: &str,
        file: Vec<u8>,
    ) -> Result<r0::media::create_content::Response, ruma_client::Error> {
        let response = self
            .client
            .request(r0::media::create_content::Request {
                filename: Some(filename.to_owned()),
                content_type: content_type.to_owned(),
                file,
            })
            .await?;

        Ok(response)
    }

    pub async fn get_content(
        &self,
        media_id: String,
        server_name: String,
    ) -> Result<r0::media::get_content::Response, ruma_client::Error> {
        self.client
            .request(r0::media::get_content::Request {
                media_id,
                server_name,
            })
            .await
    }

    pub async fn send_state_event_for_key(
        &self,
        event_type: &str,
        state_key: &str,
        data: crate::RefEventContent,
    ) -> Result<r0::state::create_state_event_for_key::Response, ruma_client::Error> {
        let data = serde_json::to_value(data).unwrap();
        let response = self
            .client
            .request(r0::state::create_state_event_for_key::Request {
                room_id: self.room_id.clone(),
                event_type: EventType::Custom(event_type.to_owned()),
                state_key: state_key.to_owned(),
                data,
            })
            .await?;

        Ok(response)
    }

    pub async fn sync(
        &self,
        types: Vec<String>,
    ) -> Result<r0::sync::sync_events::IncomingResponse, ruma_client::Error> {
        let filter =
            r0::sync::sync_events::Filter::FilterDefinition(r0::filter::FilterDefinition {
                event_fields: None,
                event_format: None,
                account_data: Some(r0::filter::Filter {
                    not_types: Vec::new(),
                    limit: None,
                    senders: None,
                    types: Some(Vec::new()),
                    not_senders: Vec::new(),
                }),
                room: Some(r0::filter::RoomFilter {
                    include_leave: None,
                    account_data: Some(r0::filter::RoomEventFilter {
                        not_types: Vec::new(),
                        not_rooms: Vec::new(),
                        limit: None,
                        rooms: Some(Vec::new()),
                        not_senders: Vec::new(),
                        senders: None,
                        types: Some(Vec::new()),
                        contains_url: None,
                    }),
                    timeline: Some(r0::filter::RoomEventFilter {
                        not_types: Vec::new(),
                        not_rooms: Vec::new(),
                        limit: None,
                        rooms: Some(vec![self.room_id.clone()]),
                        not_senders: Vec::new(),
                        senders: None,
                        types: Some(types.clone()),
                        contains_url: None,
                    }),
                    ephemeral: Some(r0::filter::RoomEventFilter {
                        not_types: Vec::new(),
                        not_rooms: Vec::new(),
                        limit: None,
                        rooms: Some(Vec::new()),
                        not_senders: Vec::new(),
                        senders: None,
                        types: Some(Vec::new()),
                        contains_url: None,
                    }),
                    state: Some(r0::filter::RoomEventFilter {
                        not_types: Vec::new(),
                        not_rooms: Vec::new(),
                        limit: None,
                        rooms: Some(vec![self.room_id.clone()]),
                        not_senders: Vec::new(),
                        senders: None,
                        types: Some(types.clone()),
                        contains_url: None,
                    }),
                    not_rooms: Vec::new(),
                    rooms: Some(vec![self.room_id.clone()]),
                }),
                presence: Some(r0::filter::Filter {
                    not_types: Vec::new(),
                    limit: None,
                    senders: None,
                    types: Some(Vec::new()),
                    not_senders: Vec::new(),
                }),
            });

        let mut sync_stream = Box::pin(self.client.sync(Some(filter), None, false));
        Ok(sync_stream.try_next().await?.unwrap())
    }
}
