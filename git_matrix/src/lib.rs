// #![warn(missing_docs)]

use error::Error;
use ruma_client::events::collections::all::RoomEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;

pub use git2;

pub mod error;
pub mod git;
pub mod matrix;

type Refs = HashMap<String, String>;

#[derive(Serialize, Deserialize)]
pub struct RefEventContent {
    sha: String,
}

#[derive(Serialize, Deserialize)]
pub struct PackFile {
    content: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct PackEventContent {
    content_uri: String,
}

/// Create a new GitMatrix
pub struct GitMatrixBuilder {
    remote_name: String,
    remote_url: String,
}

impl GitMatrixBuilder {
    pub fn new(remote_url: String) -> Self {
        Self {
            remote_name: "origin".to_owned(),
            remote_url,
        }
    }

    /// Set the remote name to be used
    pub fn remote_name(&mut self, name: String) -> &mut Self {
        self.remote_name = name;
        self
    }

    /// Build the GitMatrix
    pub async fn build(self) -> Result<GitMatrix, Error> {
        let (homeserver_url, room) = self.parse_remote_url()?;

        let credentials = self.credentials();

        let git = git::Git::new()?;

        let matrix = match credentials {
            Ok((url, username, access_token, device_id)) => {
                let homeserver_url = url;
                matrix::Builder::new(homeserver_url, room)
                    .session(&username, access_token, device_id)
                    .await?
            }
            Err(_) => {
                let (matrix, _) = matrix::Builder::new(homeserver_url, room).guest().await?;
                matrix
            }
        };

        Ok(GitMatrix { git, matrix })
    }

    fn credentials(&self) -> Result<(String, String, String, String), Error> {
        let config = git::get_config()?;
        let url = config.get_string("credential.matrix.url")?;
        let username = config.get_string("credential.matrix.username")?;
        let access_token = config.get_string("credential.matrix.access-token")?;
        let device_id = config.get_string("credential.matrix.device-id")?;

        Ok((url, username, access_token, device_id))
    }

    fn parse_remote_url(&self) -> Result<(String, String), Error> {
        let url: url::Url = self.remote_url.parse()?;
        let scheme = url.scheme();
        let host = match url.host() {
            Some(host) => host.to_string(),
            None => {
                return Err(Error {
                    message: "Invalid host in URL".to_owned(),
                })
            }
        };

        let homeserver_url = match scheme {
            "matrix" => format!("https://{}", host),
            _ => {
                let port = match url.port_or_known_default() {
                    Some(port) => port,
                    None => {
                        return Err(Error {
                            message: "Could not detect port in URL".to_owned(),
                        })
                    }
                };
                format!("{}://{}:{}", scheme, host, port)
            }
        };

        let room = match url.path_segments() {
            Some(segments) => {
                let segments: Vec<&str> = segments.collect();
                if segments.len() != 1 {
                    return Err(Error {
                        message: "Invalid path length in URL".to_owned(),
                    });
                }
                let room = format!("#{}:{}", segments[0], host);
                room
            }
            None => {
                return Err(Error {
                    message: "Invalid path in URL".to_owned(),
                })
            }
        };

        Ok((homeserver_url, room))
    }
}

pub struct GitMatrix {
    git: git::Git,
    matrix: matrix::Matrix,
}

impl GitMatrix {
    pub async fn push(&self, src: &str, dst: &str) -> Result<(), Error> {
        let pack = self.git.pack(src)?;

        let response = self
            .matrix
            .create_content("pack", "gitpack", pack.content)
            .await?;

        let pack_event = serde_json::to_value(PackEventContent {
            content_uri: response.content_uri,
        })?;

        self.matrix
            .send_custom_event("org.gitmatrix.pack", pack_event)
            .await?;

        self.matrix
            .send_state_event_for_key(
                "org.gitmatrix.refs",
                dst,
                RefEventContent {
                    sha: self.git.ref_id(src)?,
                },
            )
            .await?;

        Ok(())
    }

    pub async fn fetch(&self) -> Result<(), Error> {
        let response = self
            .matrix
            .sync(vec!["org.gitmatrix.pack".to_owned()])
            .await
            .unwrap();
        let odb = self.git.repo.odb().unwrap();

        for (_, room) in response.rooms.join {
            for event in room.timeline.events.into_iter().rev() {
                match event.into_result() {
                    Ok(event) => match event {
                        RoomEvent::CustomRoom(event) => {
                            let object: crate::PackEventContent =
                                serde_json::from_value(event.content).unwrap();
                            let uri = url::Url::parse(&object.content_uri).unwrap();
                            let server_name = uri.host().unwrap().to_string();
                            let media_id = uri.path_segments().unwrap().next().unwrap().to_string();
                            let response = self
                                .matrix
                                .get_content(media_id, server_name)
                                .await
                                .unwrap();

                            let mut packwriter = odb.packwriter().unwrap();
                            packwriter.write(&response.file).unwrap();
                            packwriter.commit().unwrap();
                        }
                        _ => (),
                    },
                    Err(_) => (),
                }
            }
        }

        Ok(())
    }

    pub async fn refs(&self) -> Result<Refs, Error> {
        let response = self
            .matrix
            .sync(vec!["org.gitmatrix.refs".to_owned()])
            .await
            .unwrap();

        let mut refs: Refs = HashMap::new();
        for (_, room) in response.rooms.join {
            for event in room.timeline.events.into_iter().rev() {
                match event.into_result() {
                    Ok(event) => match event {
                        RoomEvent::CustomState(event) => {
                            if event.event_type == "org.gitmatrix.refs" {
                                let git_ref: RefEventContent =
                                    serde_json::from_value(event.content).unwrap();
                                if !refs.contains_key(&event.state_key) {
                                    refs.insert(event.state_key, git_ref.sha);
                                }
                            }
                        }
                        _ => (),
                    },
                    Err(_) => (),
                }
            }
        }

        Ok(refs)
    }
}
