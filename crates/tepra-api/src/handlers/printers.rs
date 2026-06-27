//! Handlers for `/api/printer*` endpoints — one-to-one facade over Creator `WebAPI`.
// Stubs only: todo!() removed when handlers are implemented (T14b).
#![allow(
    clippy::todo,
    clippy::module_name_repetitions,
    clippy::missing_errors_doc
)]

use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tepra_core::{
    client::traits::TepraClient,
    dto::{
        printer::{
            AutoselectResponse, LwStatusResponse, OnlineStatusResponse, PrinterInfoResponse,
            PrinterListItem, VersionResponse,
        },
        template::{GetMarginRequest, GetMarginResponse},
    },
};

/// `GET /api/printer` — list all connected printers.
#[axum::debug_handler]
pub async fn list_printers(
    State(_client): State<Arc<dyn TepraClient>>,
) -> Json<Vec<PrinterListItem>> {
    todo!()
}

/// `GET /api/printer/version` — `WebAPI` module and driver versions.
#[axum::debug_handler]
pub async fn version(State(_client): State<Arc<dyn TepraClient>>) -> Json<VersionResponse> {
    todo!()
}

/// `GET /api/printer/autoselect` — currently auto-selected printer name.
#[axum::debug_handler]
pub async fn autoselect(State(_client): State<Arc<dyn TepraClient>>) -> Json<AutoselectResponse> {
    todo!()
}

/// `GET /api/printer/info/{name}` — printer capabilities and tape list.
#[axum::debug_handler]
pub async fn printer_info(
    State(_client): State<Arc<dyn TepraClient>>,
    Path(_name): Path<String>,
) -> Result<Json<PrinterInfoResponse>, StatusCode> {
    todo!()
}

/// `GET /api/printer/onlinestatus/{name}` — printer online/offline state.
#[axum::debug_handler]
pub async fn online_status(
    State(_client): State<Arc<dyn TepraClient>>,
    Path(_name): Path<String>,
) -> Json<OnlineStatusResponse> {
    todo!()
}

/// `GET /api/printer/lwstatus/{name}` — detailed tape and device status.
#[axum::debug_handler]
pub async fn lw_status(
    State(_client): State<Arc<dyn TepraClient>>,
    Path(_name): Path<String>,
) -> Json<LwStatusResponse> {
    todo!()
}

/// `POST /api/printer/getmargin/{name}` — compute print margins.
#[axum::debug_handler]
pub async fn get_margin(
    State(_client): State<Arc<dyn TepraClient>>,
    Path(_name): Path<String>,
    Json(_req): Json<GetMarginRequest>,
) -> Json<GetMarginResponse> {
    todo!()
}
