use dyn_clone::DynClone;

pub trait PingHandler<Endpoint>: DynClone + 'static {
    fn handle(&self, endpoint: Endpoint);
}

dyn_clone::clone_trait_object!(<Endpoint> PingHandler<Endpoint>);

impl<Endpoint, F> PingHandler<Endpoint> for F
where
    F: Fn(Endpoint) + Sync + Send + Clone + 'static,
{
    fn handle(&self, endpoint: Endpoint) {
        (self)(endpoint)
    }
}
