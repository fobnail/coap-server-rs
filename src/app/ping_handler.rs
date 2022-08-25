use alloc::boxed::Box;
use async_trait::async_trait;
use dyn_clone::DynClone;
use futures::Future;

#[async_trait]
pub trait PingHandler<Endpoint>: DynClone + 'static {
    async fn handle(&self, endpoint: Endpoint);
}

dyn_clone::clone_trait_object!(<Endpoint> PingHandler<Endpoint>);

#[async_trait]
impl<Endpoint, F, R> PingHandler<Endpoint> for F
where
    Endpoint: Send + Sync + 'static,
    F: Fn(Endpoint) -> R + Sync + Send + Clone + 'static,
    R: Future<Output = ()> + Send,
{
    async fn handle(&self, endpoint: Endpoint) {
        (self)(endpoint).await
    }
}
