use std::{fmt::Debug, sync::Arc};

use super::{handler::HttpRequestHandler, logging::Logging, parser::Parser, router::HttpRouter};

use tokio::net::TcpListener;

#[derive(Debug)]
pub struct HttpServer {
    logging_enabled: bool,
    router: Arc<HttpRouter>,
}

impl HttpServer {
    pub fn new(router: HttpRouter) -> Self {
        Self {
            logging_enabled: false,
            router: Arc::new(router),
        }
    }


    #[tokio::main]
    pub async fn listen(&self, port: u32) {
        let listen = TcpListener::bind(format!("127.0.0.1:{}", port))
            .await
            .unwrap();

        log::info!("Starting server...");
        log::info!("Server started! Listening on port... {}", port);

        loop {
            match listen.accept().await {
                Ok((mut socket, _)) => {
                    let s_router = self.router.clone();
                    let mut handler = HttpRequestHandler::new(s_router);
                    // pin!(socket);
                    if self.logging_enabled() {
                        handler.enable_logging();
                    }
                    tokio::spawn(async move {
                        let _ = handler.handle_incoming_request(socket).await;
                    });
                }
                Err(err) => {
                    if self.logging_enabled() {
                        log::info!("Request not processed! There was an error. Error: {}", err);
                    }
                }
            }
        }
    }
}

impl Logging for HttpServer {
    fn enable_logging(&mut self) {
        self.logging_enabled = true;
    }
    fn disable_logging(&mut self) {
        self.logging_enabled = true;
    }

    fn logging_enabled(&self) -> bool {
        self.logging_enabled
    }
}
