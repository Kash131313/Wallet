#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod crypto;
mod wallet_manager;
use wallet_manager::{add_account, list_accounts, init_store, create_multisig_wallet, list_multisig_wallets, remove_account, remove_multisig_wallet, change_password};
mod network;
use network::{derive_address, get_balance};

fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![add_account, remove_account, list_accounts, init_store, derive_address, get_balance, create_multisig_wallet, remove_multisig_wallet, list_multisig_wallets, change_password])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
