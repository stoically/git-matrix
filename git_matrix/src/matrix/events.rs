pub mod custom {
    use ruma_api::ruma_api;
    use ruma_client::{
        events::EventType,
        identifiers::{EventId, RoomId},
    };

    ruma_api! {
        metadata {
            description: "Send a custom event to a room.",
            method: PUT,
            name: "send_custom_event",
            path: "/_matrix/client/r0/rooms/:room_id/send/:event_type/:txn_id",
            rate_limited: false,
            requires_authentication: true,
        }

        request {
            /// The room to send the event to.
            #[ruma_api(path)]
            pub room_id: RoomId,
            /// The type of event to send.
            #[ruma_api(path)]
            pub event_type: EventType,
            /// The transaction ID for this event.
            ///
            /// Clients should generate an ID unique across requests with the
            /// same access token; it will be used by the server to ensure
            /// idempotency of requests.
            #[ruma_api(path)]
            pub txn_id: String,
            /// The event's content.
            #[ruma_api(body)]
            pub content: serde_json::Value,
        }

        response {
            /// A unique identifier for the event.
            pub event_id: EventId,
        }
    }
}
