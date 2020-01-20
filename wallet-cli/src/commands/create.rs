use clap::ArgMatches;
use hex::ToHex;
use keys::KeyPair;
use proto::api::EmptyMessage;
use proto::api_grpc::Wallet;
use serde_json::json;

use crate::error::Error;
use crate::utils::client;
use crate::utils::jsont;

fn create_key() -> Result<(), Error> {
    let kp = KeyPair::generate();
    let address = kp.address();

    println!("Address(Base58): {:}", address);
    println!("Address(hex):    {:}", address.encode_hex::<String>());
    println!("Public:          {:}", kp.public());
    println!("Private:         {:}", kp.private());
    Ok(())
}

pub fn create_zkey() -> Result<(), Error> {
    let (_, payload, _) = client::GRPC_CLIENT
        .get_new_shielded_address(Default::default(), EmptyMessage::new())
        .wait()?;
    let mut addr_info = serde_json::to_value(&payload)?;

    // sk: spending key => ask, nsk, ovk
    // ask: spend authorizing key, 256 => ak
    // nsk: proof authorizing key, 256 => nk
    // ovk: outgoing viewing key, 256
    // ivk: incoming viewing key, 256 => pkD
    // d: diversifier, 11bytes
    // pkD: the public key of the address, g_d^ivk
    // pkD + d => z-addr
    for key in &["sk", "ask", "nsk", "ovk", "ak", "nk", "ivk", "d", "pkD"] {
        addr_info[key] = json!(jsont::bytes_to_hex_string(&addr_info[key]));
    }
    println!("{}", serde_json::to_string_pretty(&addr_info)?);
    Ok(())
}

pub fn main(matches: &ArgMatches) -> Result<(), Error> {
    match matches.subcommand() {
        ("key", _) => create_key(),
        ("zkey", _) => create_zkey(),
        _ => unreachable!("checked by cli.yml; qed"),
    }
}
