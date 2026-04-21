use std::sync::Arc;
use tokio::runtime::Runtime;

pub fn fire_and_forget<F>(runtime: &Arc<Runtime>, fut: F)
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    runtime.spawn(fut);
}
