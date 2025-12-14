use bitcoin::Network;

pub fn network() -> Network {
    let network = Network::Regtest;
    network
}
