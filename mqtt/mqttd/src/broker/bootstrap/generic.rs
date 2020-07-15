use mqtt_broker::{Broker, BrokerBuilder, BrokerConfig, BrokerSnapshot, Error, Server};
use mqtt_broker_core::auth::{
    authenticate_fn_ok, authorize_fn_ok, AuthId, Authorization, Authorizer,
};

pub async fn broker(
    config: &BrokerConfig,
    state: Option<BrokerSnapshot>,
) -> Result<Broker<impl Authorizer>, Error> {
    let broker = BrokerBuilder::default()
        .with_authorizer(authorize_fn_ok(|_| Authorization::Allowed))
        .with_state(state.unwrap_or_default())
        .with_config(config.clone())
        .build();

    Ok(broker)
}

pub async fn server<Z>(config: &BrokerConfig, broker: Broker<Z>) -> Result<Server<Z>, Error>
where
    Z: Authorizer + Send + 'static,
{
    let mut server = Server::from_broker(broker);

    if let Some(tcp) = config.transports().tcp() {
        let authenticator = authenticate_fn_ok(|_| Some(AuthId::Anonymous));
        server.tcp(tcp.addr(), authenticator);
    }

    if let Some(tls) = config.transports().tls() {
        let authenticator = authenticate_fn_ok(|_| Some(AuthId::Anonymous));
        server.tls(tls.addr(), tls.cert_path(), authenticator)?;
    }

    Ok(server)
}
