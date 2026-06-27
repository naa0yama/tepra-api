//! `PrinterRegistry` — single-process map of printer name → `PrinterHandle`.

use std::{fmt, sync::Arc};

use dashmap::DashMap;
use tepra_core::client::TepraClient;

use super::PrinterHandle;

/// Single-process registry mapping printer names to their running actors.
///
/// Guarantees at most one `PrinterActor` per printer name.
#[allow(dead_code, clippy::module_name_repetitions)]
pub struct PrinterRegistry {
    client: Arc<dyn TepraClient>,
    actors: DashMap<String, Arc<PrinterHandle>>,
}

impl fmt::Debug for PrinterRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PrinterRegistry")
            .field("actors_count", &self.actors.len())
            .finish_non_exhaustive()
    }
}

#[allow(
    clippy::todo,
    clippy::unused_async,
    clippy::must_use_candidate,
    clippy::module_name_repetitions
)]
impl PrinterRegistry {
    /// Create a new empty registry backed by `client`.
    pub fn new(_client: Arc<dyn TepraClient>) -> Self {
        todo!("T13b: implement PrinterRegistry::new")
    }

    /// Return the handle for `name`, spawning a new actor if none exists yet.
    pub fn get_or_spawn(&self, _name: &str) -> Arc<PrinterHandle> {
        todo!("T13b: implement PrinterRegistry::get_or_spawn")
    }

    /// Shut down every registered actor and join their tasks.
    pub async fn shutdown_all(self) {
        todo!("T13b: implement PrinterRegistry::shutdown_all")
    }
}
