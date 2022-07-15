use serde_json::Value;
use gdk_electrum::{ElectrumSession, NativeNotif};
use gdk_common::wally::{make_str, read_str, bip39_mnemonic_to_seed};
use std::ffi::CString;
use std::os::raw::c_char;
use gdk_common::network::Network;
use std::string::String;
use gdk_common::error::Error;
use  std::ffi;
use gdk_electrum::interface::ElectrumUrl;
use std::fmt::Debug;
use gdk_common::session::Session;
pub use flutter::worker;

pub const GA_ERROR: i32 = -1;

pub fn init() {
    
    /*let mut rng = rand::rngs::OsRng::new().expect("creating OsRng failed");
    let mne = generate_mnemonic12_from_rng(&mut rng);
    println!("{:?}", mne);
    let mut session = create_session();
    //session.connect(&serde_json::Value::Null).unwrap();
    let mnemonic = gdk_common::mnemonic::Mnemonic::from(mne.to_owned());
    println!("{:?}", &mnemonic);
    let login_data = session.login(&mnemonic, None).unwrap();
    //println!("{:?}", login_data);
    //let c = "";
    //bip39_mnemonic_to_seed(&mne, &c);
    return Ok(session);*/
    let c = unsafe { worker::init()};
}

fn generate_mnemonic12_from_rng<R: rand::RngCore + rand::CryptoRng>(rng: &mut R) -> String {
    const ENTROPY_SIZE_MNEMONIC12: usize = 16;
    let mut key: [u8; ENTROPY_SIZE_MNEMONIC12] = [0; ENTROPY_SIZE_MNEMONIC12];
    rng.try_fill_bytes(&mut key)
        .expect("generating random bytes failed");
    let mnemonic = bip39::Mnemonic::from_entropy(&key).expect("generating mnemonic failed");
    mnemonic.to_string()
}

fn create_session() -> ElectrumSession{
    const GA_ERROR: i32 = -1;
    let network_string = r#""#;//r#" { "age": 43, "name": "John Doe" } "#;
    let c_str = CString::new(network_string).unwrap();
    
    println!("{:?}", read_str(c_str.as_ptr()));

    let network: Network = gdk_common::network::Network {
        name: "testnet".to_string(),
        network: "electrum-testnet-liquid".to_owned(),
        development: false,
        liquid: true,
        mainnet: false,
        tx_explorer_url: "https://blockstream.info/liquidtestnet/liquidtestnet/tx/".to_string(),
        address_explorer_url:  "https://blockstream.info/liquidtestnet/address/".to_string(),
        electrum_tls: Some(true),
        electrum_url:  Some("blockstream.info:465".to_string()),
        //electrum_onion_url: Some("".to_string()),
        validate_domain: Some(false),
        policy_asset: Some("144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49".to_string()),
        sync_interval: None,
        ct_bits: Some(52),
        ct_exponent: Some(0),
        ct_min_value: None,
        spv_enabled: Some(false),
        asset_registry_url:  Some("https://assets-testnet.blockstream.info".to_string()),
        asset_registry_onion_url: None,
        spv_multi: Some(false),
        spv_servers: None,
        //max_reorg_blocks : None,
        //pin_server_onion_url: "".to_string(),
        //pin_server_public_key: "".to_string(),
        //pin_server_url:"".to_string(),
        //use_tor:Some(false),
        //proxy: Some("".to_string()),
        //state_dir:"".to_string()
        taproot_enabled_at: Some(0xffffffff),
    };
    let url = ElectrumUrl::Plaintext("blockstream.info:465".to_string());
    println!("{:?}", url);
    let mut session = ElectrumSession::create_session(network, "", None, url);
    println!("balbal");
    return session
}