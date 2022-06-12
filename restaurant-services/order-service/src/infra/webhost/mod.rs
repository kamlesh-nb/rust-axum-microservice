use std::net::{Ipv4Addr, SocketAddr};
use std::{sync::Arc};

use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing, Extension, Json, Router,
    Server,
};

use mediator::DefaultAsyncMediator;
use tokio::sync::Mutex;
use tower_http::{compression::CompressionLayer, cors::CorsLayer};

use super::{Settings, CosmosService};

#[derive(Clone)]
#[allow(unused)]
pub struct WebHost {
    app: Router,
}

#[derive(Clone)]
#[allow(unused)]
pub struct WebHostBuilder {
    app: Router,
}

#[allow(unused)]
impl WebHost {
    pub fn new(app: Router) -> Self {
        Self { app }
    }

    pub fn add_cors(mut self, cors: CorsLayer) -> Self {
        self.app = self.app.layer(cors);
        self
    }

    pub fn add_compression(mut self) -> Self {
        self.app = self.app.layer(CompressionLayer::new());
        self
    }

    pub fn add_mediator(mut self, mediator: DefaultAsyncMediator) -> Self {
      let mediatr = Arc::new(Mutex::new(mediator));
        self.app = self.app.layer(Extension(mediatr.clone()));
        self
    }

    pub fn add_cosmosdb(
        mut self,
        service: Arc<Mutex<CosmosService>>
    ) -> Self {
        
        self.app = self.app.layer(Extension(service));
        self
    }

    pub fn add_api_docs(mut self, apidoc: utoipa::openapi::OpenApi) -> Self {
        let cfg = Arc::new(utoipa_swagger_ui::Config::from("/swagger/swagger.json"));
        self.app = self
            .app
            .route(
                "/swagger/swagger.json",
                routing::get({
                    let doc = apidoc.clone();
                    move || async { Json(doc) }
                }),
            )
            .route(
                "/swagger-ui/*tail",
                routing::get(serve_swagger_ui).layer(Extension(cfg)),
            );
        self
    }

    pub async fn start(mut self, config: Settings)  {
        let cfg = Arc::new(Mutex::new(config));
        self.app = self.app.layer(Extension(cfg.clone()));
        let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
        let server = Server::bind(&address)
            .serve(self.app.into_make_service());

        if let Err(e) = server.await {
          eprintln!("server error: {}", e);
        }
    }
}

#[allow(unused)]
impl WebHostBuilder {
    pub fn new(app: Router) -> Self {
        Self { app }
    }

    pub fn build(self) -> WebHost {
        WebHost {
            app: self.app,
        }
    }
}

async fn serve_swagger_ui(
    Path(tail): Path<String>,
    Extension(state): Extension<Arc<utoipa_swagger_ui::Config<'static>>>,
) -> impl IntoResponse {
    match utoipa_swagger_ui::serve(&tail[1..], state) {
        Ok(file) => file
            .map(|file| {
                (
                    StatusCode::OK,
                    [("Content-Type", file.content_type)],
                    file.bytes,
                )
                    .into_response()
            })
            .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response()),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}
