//! Helpers for spawning async work from the synchronous egui main thread.

use std::sync::Arc;
use tokio::runtime::Runtime;

/// Spawn an async task on the shared Tokio runtime and forget about it.
/// Any panic / error is logged but not propagated — GUI callers should not
/// block on completion.
pub fn fire_and_forget<F>(runtime: &Arc<Runtime>, fut: F)
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    runtime.spawn(fut);
}
