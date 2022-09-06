use {
    serde::{Deserialize},
    native_tls::Identity
};

#[derive(Deserialize, Clone)]
pub struct ClientConnectionConfig {

}


#[derive(Clone)]
enum ProxyMode {
    Http,
    Https(Identity),
    TCP(String)
}

