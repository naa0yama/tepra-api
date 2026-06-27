//! HTTP implementation of [`TepraClient`] using `reqwest`.
//!
//! All methods are stubs returning `TepraError::Transport` until T9b
//! implements the actual HTTP calls.

use async_trait::async_trait;

use crate::{
    client::traits::TepraClient,
    dto::{
        job::{
            JobControlRequest, JobInfoResponse, JobProgressResponse, PrintRequest, PrintResponse,
        },
        printer::{
            AutoselectResponse, LwStatusResponse, OnlineStatusResponse, PrinterInfoResponse,
            PrinterListItem, VersionResponse,
        },
        template::{GetMarginRequest, GetMarginResponse, ImportFrameItem, ImportFrameRequest},
    },
    error::TepraError,
};

/// HTTP client for the KING JIM TEPRA Creator `WebAPI`.
///
/// Constructed with [`ReqwestTepraClient::new`]; inject `base_url` to point at
/// the Creator daemon (default `http://localhost:29108`) or a `WireMock` server
/// in tests.
#[derive(Debug)]
pub struct ReqwestTepraClient {
    base_url: String,
    client: reqwest::Client,
}

impl ReqwestTepraClient {
    /// Create a new client targeting `base_url` (e.g. `"http://localhost:29108"`).
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: reqwest::Client::new(),
        }
    }
}

fn not_implemented(endpoint: &str) -> TepraError {
    TepraError::Transport {
        source: anyhow::anyhow!("not yet implemented (T9b): {endpoint}"),
    }
}

#[async_trait]
impl TepraClient for ReqwestTepraClient {
    async fn list_printers(&self) -> Result<Vec<PrinterListItem>, TepraError> {
        let _ = (&self.base_url, &self.client);
        Err(not_implemented("GET /api/printer"))
    }

    async fn version(&self) -> Result<VersionResponse, TepraError> {
        let _ = (&self.base_url, &self.client);
        Err(not_implemented("GET /api/printer/version"))
    }

    async fn autoselect(&self) -> Result<AutoselectResponse, TepraError> {
        let _ = (&self.base_url, &self.client);
        Err(not_implemented("GET /api/printer/autoselect"))
    }

    async fn printer_info(&self, name: &str) -> Result<PrinterInfoResponse, TepraError> {
        let _ = (&self.base_url, &self.client, name);
        Err(not_implemented("GET /api/printer/info/:name"))
    }

    async fn online_status(&self, name: &str) -> Result<OnlineStatusResponse, TepraError> {
        let _ = (&self.base_url, &self.client, name);
        Err(not_implemented("GET /api/printer/onlinestatus/:name"))
    }

    async fn lw_status(&self, name: &str) -> Result<LwStatusResponse, TepraError> {
        let _ = (&self.base_url, &self.client, name);
        Err(not_implemented("GET /api/printer/lwstatus/:name"))
    }

    async fn print(&self, name: &str, req: PrintRequest) -> Result<PrintResponse, TepraError> {
        let _ = (&self.base_url, &self.client, name, req);
        Err(not_implemented("POST /api/printer/print/:name"))
    }

    async fn tapefeed(&self, name: &str) -> Result<(), TepraError> {
        let _ = (&self.base_url, &self.client, name);
        Err(not_implemented("GET /api/printer/tapefeed/:name"))
    }

    async fn job_progress(
        &self,
        name: &str,
        jobid: u64,
    ) -> Result<JobProgressResponse, TepraError> {
        let _ = (&self.base_url, &self.client, name, jobid);
        Err(not_implemented("GET /api/printer/job/progress/:name"))
    }

    async fn job_info(&self, name: &str, jobid: u64) -> Result<JobInfoResponse, TepraError> {
        let _ = (&self.base_url, &self.client, name, jobid);
        Err(not_implemented("GET /api/printer/job/info/:name"))
    }

    async fn job_control(&self, name: &str, req: JobControlRequest) -> Result<(), TepraError> {
        let _ = (&self.base_url, &self.client, name, req);
        Err(not_implemented("POST /api/printer/job/control/:name"))
    }

    async fn import_frame(
        &self,
        req: ImportFrameRequest,
    ) -> Result<Vec<ImportFrameItem>, TepraError> {
        let _ = (&self.base_url, &self.client, req);
        Err(not_implemented("POST /api/printer/template/importframe"))
    }

    async fn get_margin(
        &self,
        name: &str,
        req: GetMarginRequest,
    ) -> Result<GetMarginResponse, TepraError> {
        let _ = (&self.base_url, &self.client, name, req);
        Err(not_implemented("POST /api/printer/getmargin/:name"))
    }
}
