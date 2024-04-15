use anyhow::Context;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::sync::Arc;
use std::clone::Clone;
use mysql::Pool;

mod db_handler;
use db_handler::{establish_connection, execute_query, getOneUser};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "with_axum_htmx_askama=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("mysql pool...");

    let pool = establish_connection().await?;

    let shared_pool = Arc::new(pool);

    info!("initializing router...");

    let assets_path = std::env::current_dir().unwrap();

    let pool_clone = shared_pool.clone();
    let api_router = Router::new()
    .route("/hello", get(
        move |_request: axum::http::Request<axum::body::Body>| {
            async move {
                hello_from_the_server(pool_clone).await
            }
        }
    ));
    
    let pool_clone2 = shared_pool.clone();
    let router = Router::new()
    .nest("/api", api_router)
    .route("/", get(
        move |_request: axum::http::Request<axum::body::Body>| {
            async move {
                hello(pool_clone2).await
            }
        }
    ))

    .nest_service(        
        "/assets",        
        ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),    
    );

    let port = 8000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    info!("router initialized, now listening on port {}", port);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .context("error while starting server")?;

    Ok(())
}

async fn hello(pool: Arc<Pool>) -> impl IntoResponse {
    match getOneUser(&pool).await {
        Ok(rows) => {
            let res: String = rows;
            let template = HelloTemplate { name: res };
            Ok(HtmlTemplate(template).into_response())
        },
        Err(err) => {
            let template = ErrorTemplate { error: format!("Error: {:?}", err) };
            Err(HtmlTemplate(template).into_response())
        }
    }
}


async fn hello_from_the_server(pool: Arc<Pool>) -> Html<String> {
    match execute_query(&pool).await {
        Ok(rows) => {
            let res: String = rows.iter()
            .map(|&(i, ref s)| format!("{} {}", i, s))
            .collect::<Vec<String>>()
            .join(", ");
            Html(format!("{}", res))
        },
        Err(err) => {
            Html(format!("Error executing query: {:?}", err))
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {
    name: String,
}
#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    error: String,
}

struct HtmlTemplate<T>(T);
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}