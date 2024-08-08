use anchor_client::{anchor_lang::pubkey, Client, Cluster};
use finternet_core::{
    primitives::core::{FinternetUID, PublicKey},
    primitives::user::User,
    smartcontract_interface::user_manager::UserManager,
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey,
    signature::{read_keypair_file, Keypair},
};
use squads_multisig::{client}
use std::{collections::HashMap, sync::Arc};

pub fn get_client() -> Client<Arc<Keypair>> {
    let url = Cluster::Custom(
        "http://localhost:8899".to_string(),
        "ws://127.0.0.1:8900".to_string(),
    );

    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Example requires a keypair file");

    let payer = Arc::new(payer);
    let client =
        Client::new_with_options(url.clone(), payer.clone(), CommitmentConfig::confirmed());

    client
}

pub struct SolanaUserManager;

impl UserManager for SolanaUserManager {
    fn create_user(&self, _user: User) -> Result<String, String> {
        let anchor_client = get_client();
        let program = client.program(my_program::ID)?;

        Ok("Success".to_string())
    }

    fn update_user(&self, _user: User) -> Result<String, String> {
        Ok("Success".to_string())
    }

    fn delete_user(&self, _user_id: FinternetUID) -> Result<String, String> {
        Ok("Success".to_string())
    }

    fn rotate_key(&self, _user_id: FinternetUID, _new_key: PublicKey) -> Result<String, String> {
        Ok("Success".to_string())
    }

    fn accept_incoming_asset(
        &self,
        _user_id: FinternetUID,
        _asset_id: FinternetUID,
    ) -> Result<String, String> {
        Ok("Success".to_string())
    }

    fn update_asset_units(
        &self,
        _user_id: FinternetUID,
        _asset_id: FinternetUID,
        _new_units: u64,
    ) -> Result<String, String> {
        Ok("Success".to_string())
    }

    fn update_asset_config(
        &self,
        _user_id: FinternetUID,
        _asset_id: FinternetUID,
        _config: HashMap<String, String>,
    ) -> Result<String, String> {
        Ok("Success".to_string())
    }
}
