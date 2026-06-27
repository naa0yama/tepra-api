//! Axum router builder for tepra-api.
#![allow(clippy::module_name_repetitions)]

use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use tepra_core::client::traits::TepraClient;

use crate::handlers::printers;

/// Build the main API router, wiring all `/api/printer*` routes.
pub fn build_router(client: Arc<dyn TepraClient>) -> Router {
    Router::new()
        .route("/api/printer", get(printers::list_printers))
        .route("/api/printer/version", get(printers::version))
        .route("/api/printer/autoselect", get(printers::autoselect))
        .route("/api/printer/info/:name", get(printers::printer_info))
        .route(
            "/api/printer/onlinestatus/:name",
            get(printers::online_status),
        )
        .route("/api/printer/lwstatus/:name", get(printers::lw_status))
        .route("/api/printer/getmargin/:name", post(printers::get_margin))
        .with_state(client)
}
