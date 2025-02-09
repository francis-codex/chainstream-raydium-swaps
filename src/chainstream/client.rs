//! Provides a simple async client for the ChainStream API.
#![allow(unused)]
use std::{sync::Arc, time::Duration};

use anyhow::Result;
use jsonrpsee::{
    core::client::{Subscription, SubscriptionClientT},
    http_client::HeaderMap,
    ws_client::{PingConfig, WsClient, WsClientBuilder},
};

use super::methods::SubscriptionMethod;

pub type ChainStreamSubscription<T> = Subscription<T>;

const CHAINSTREAM_API_URL: &'static str = "wss://chainstream.api.syndica.io";

pub struct ClientBuilder {
    token: String,
    ws_client_builder: WsClientBuilder,
}

#[allow(unused)]
pub type ClientError = jsonrpsee::core::ClientError;

impl ClientBuilder {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            ws_client_builder: WsClientBuilder::default(),
            token: Default::default(),
        }
    }

    /// Set the Syndica API token for the client.
    #[allow(unused)]
    pub fn token(self, token: &str) -> Self {
        let mut headers = jsonrpsee::http_client::HeaderMap::new();
        headers.insert("X-Syndica-Api-Token", token.parse().unwrap());
        Self {
            token: token.to_string(),
            ws_client_builder: self.ws_client_builder.set_headers(headers),
        }
    }

    /// See documentation [`WsTransportClientBuilder::max_request_size`] (default is 10 MB).
    #[allow(unused)]
    pub fn max_request_size(self, size: u32) -> Self {
        Self {
            ws_client_builder: self.ws_client_builder.max_request_size(size),
            ..self
        }
    }

    /// See documentation [`WsTransportClientBuilder::max_response_size`] (default is 10 MB).
    #[allow(unused)]
    pub fn max_response_size(self, size: u32) -> Self {
        Self {
            ws_client_builder: self.ws_client_builder.max_response_size(size),
            ..self
        }
    }

    /// See documentation [`ClientBuilder::request_timeout`] (default is 60 seconds).
    #[allow(unused)]
    pub fn request_timeout(self, timeout: Duration) -> Self {
        Self {
            ws_client_builder: self.ws_client_builder.request_timeout(timeout),
            ..self
        }
    }

    /// See documentation [`WsTransportClientBuilder::connection_timeout`] (default is 10 seconds).
    #[allow(unused)]
    pub fn connection_timeout(self, timeout: Duration) -> Self {
        Self {
            ws_client_builder: self.ws_client_builder.connection_timeout(timeout),
            ..self
        }
    }

    /// Enable websocket ping/pong heartbeat. This is important when using ChainStream API for
    /// low-traffic applications.
    #[allow(unused)]
    pub fn enable_ws_ping(self) -> Self {
        let ws_ping = PingConfig::new().ping_interval(Duration::from_secs(30));
        Self {
            ws_client_builder: self.ws_client_builder.enable_ws_ping(ws_ping),
            ..self
        }
    }

    /// See documentation [`ClientBuilder::disable_ws_ping`]
    #[allow(unused)]
    pub fn disable_ws_ping(self) -> Self {
        Self {
            ws_client_builder: self.ws_client_builder.disable_ws_ping(),
            ..self
        }
    }

    /// See documentation [`WsTransportClientBuilder::set_headers`] (default is none).
    #[allow(unused)]
    pub fn set_headers(self, headers: jsonrpsee::http_client::HeaderMap) -> Self {
        Self {
            ws_client_builder: self.ws_client_builder.set_headers(headers),
            ..self
        }
    }

    /// See documentation [`ClientBuilder::max_concurrent_requests`] (default is 256).
    #[allow(unused)]
    pub fn max_concurrent_requests(self, max: usize) -> Self {
        Self {
            ws_client_builder: self.ws_client_builder.max_concurrent_requests(max),
            ..self
        }
    }

    /// See documentation [`WsClientBuilder::max_buffer_capacity_per_subscription`] (default is
    /// 1024).
    #[allow(unused)]
    pub fn max_buffer_capacity_per_subscription(self, max: usize) -> Self {
        Self {
            ws_client_builder: self
                .ws_client_builder
                .max_buffer_capacity_per_subscription(max),
            ..self
        }
    }

    /// See documentation [`ClientBuilder::set_tcp_no_delay`] (default is true).
    #[allow(unused)]
    pub fn set_tcp_no_delay(self, no_delay: bool) -> Self {
        Self {
            ws_client_builder: self.ws_client_builder.set_tcp_no_delay(no_delay),
            ..self
        }
    }

    #[allow(unused)]
    pub async fn build(self) -> Result<ChainStreamClient, ClientError> {
        Ok(ChainStreamClient {
            inner: Arc::new(self.ws_client_builder.build(CHAINSTREAM_API_URL).await?),
            token: self.token,
        })
    }
}

#[derive(Debug)]
pub struct ChainStreamClient {
    inner: Arc<WsClient>,

    #[allow(dead_code)]
    token: String,
}

impl ChainStreamClient {
    /// Creates a new ChainStreamClient instance from a given URL and headers.
    ///
    /// The `url` parameter should be a valid URL to the ChainStream API. This is expected to be
    /// wss://chainstream.api.syndica.io
    pub async fn new(token: impl AsRef<str>) -> Result<Self> {
        let mut map = HeaderMap::new();
        map.insert("X-Syndica-Api-Token", token.as_ref().parse().unwrap());

        Ok(Self {
            inner: Arc::new(
                WsClientBuilder::new()
                    .set_headers(map)
                    .build(CHAINSTREAM_API_URL)
                    .await?,
            ),
            token: token.as_ref().to_string(),
        })
    }

    pub async fn subscribe<M>(&self, method: M) -> Result<ChainStreamSubscription<M::Output>>
    where
        M: SubscriptionMethod,
    {
        let inner = self.inner.clone();

        let subscription = inner
            .subscribe(
                method.subscribe_method(),
                method.params()?,
                method.unsubscribe_method(),
            )
            .await?;

        Ok(subscription)
    }
}
