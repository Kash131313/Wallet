use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use super::crypto;

// Encrypted storage file name – placed in the application working directory.
pub const STORE_FILE: &str = "wallet_store.enc";

/// Тип результата для Tauri‑команд: ошибки передаются как строки,
/// потому что Tauri требует, чтобы типы команд были Serialize.
pub type CmdResult<T> = Result<T, String>;

/// Обычный одно‑подписной кошелёк (как в оригинальном приложении).
#[derive(Serialize, Deserialize, Clone)]
pub struct WalletEntry {
    pub name: String,
    pub address: String,
    /// Plain seed phrase – stored only inside the encrypted blob.
    pub seed: String,
}

/// Мультисиг‑кошелёк. Поле `address` – адрес развернутого контракта multisig.
/// `participants` – список всех участников (адресов). `required` – число
/// подписей, необходимых для подтверждения транзакции.
#[derive(Serialize, Deserialize, Clone)]
pub struct MultisigWallet {
    pub name: String,
    pub address: String,
    pub participants: Vec<String>,
    pub required: u8,
}

#[derive(Serialize, Deserialize)]
pub struct WalletStore {
    pub salt: Vec<u8>,                // 16‑byte random salt
    pub accounts: Vec<WalletEntry>, // list of saved single‑sig wallets
    pub multisig_wallets: Vec<MultisigWallet>, // list of multisig wallets
}

/// Load the encrypted store from disk and decrypt it using the master password.
pub fn load_store(password: &str) -> Result<WalletStore, Box<dyn std::error::Error>> {
    let data = fs::read(STORE_FILE)?;
    // layout: [salt(16)][nonce(12)][ciphertext]
    if data.len() < 28 {
        return Err("Store file is corrupted".into());
    }
    let salt = &data[0..16];
    let nonce = &data[16..28];
    let ciphertext = &data[28..];
    let key = crypto::derive_key(password, salt)?;
    let plain = crypto::decrypt(ciphertext, nonce, &key)?;
    let store: WalletStore = serde_json::from_slice(&plain)?;
    Ok(store)
}

/// Save the store to disk, encrypting it with the master password.
pub fn save_store(store: &WalletStore, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure a salt is present – if the store is brand‑new we generate one.
    let salt_bytes: [u8; 16] = if store.salt.is_empty() {
        crypto::generate_salt()
    } else {
        let mut arr = [0u8; 16];
        arr.copy_from_slice(&store.salt);
        arr
    };
    let key = crypto::derive_key(password, &salt_bytes)?;
    let plain = serde_json::to_vec(store)?;
    let (nonce, ciphertext) = crypto::encrypt(&plain, &key)?;
    // Assemble final blob: salt + nonce + ciphertext
    let mut out = Vec::new();
    out.extend_from_slice(&salt_bytes);
    out.extend_from_slice(&nonce);
    out.extend_from_slice(&ciphertext);
    fs::write(STORE_FILE, out)?;
    Ok(())
}

/// Initialise a brand‑new empty store with a fresh random salt.
pub fn create_new_store(password: &str) -> Result<WalletStore, Box<dyn std::error::Error>> {
    let salt = crypto::generate_salt();
    let store = WalletStore {
        salt: salt.to_vec(),
        accounts: Vec::new(),
        multisig_wallets: Vec::new(),
    };
    save_store(&store, password)?;
    Ok(store)
}

/// Открыть хранилище: если файла нет — создать пустое, иначе проверить пароль.
fn open_store(password: &str) -> Result<WalletStore, Box<dyn std::error::Error>> {
    if Path::new(STORE_FILE).exists() {
        load_store(password)
    } else {
        create_new_store(password)
    }
}

/// Add a new wallet (name, address, seed) to the encrypted store.
#[tauri::command]
pub fn add_account(password: &str, name: &str, address: &str, seed: &str) -> CmdResult<()> {
    let mut store = open_store(password).map_err(|e| e.to_string())?;
    if store.accounts.iter().any(|a| a.name == name) {
        return Err(format!("Аккаунт с именем '{}' уже существует", name));
    }
    store.accounts.push(WalletEntry {
        name: name.to_string(),
        address: address.to_string(),
        seed: seed.to_string(),
    });
    save_store(&store, password).map_err(|e| e.to_string())
}

/// Remove a wallet by name.
#[tauri::command]
pub fn remove_account(password: &str, name: &str) -> CmdResult<()> {
    let mut store = open_store(password).map_err(|e| e.to_string())?;
    let before = store.accounts.len();
    store.accounts.retain(|a| a.name != name);
    if store.accounts.len() == before {
        return Err(format!("Аккаунт '{}' не найден", name));
    }
    save_store(&store, password).map_err(|e| e.to_string())
}

/// Create a new multisig wallet entry and store it encrypted.
#[tauri::command]
pub fn create_multisig_wallet(
    password: &str,
    name: &str,
    address: &str,
    participants: Vec<String>,
    required: u8,
) -> CmdResult<()> {
    let mut store = open_store(password).map_err(|e| e.to_string())?;
    // Ensure the name is unique among multisig wallets.
    if store.multisig_wallets.iter().any(|w| w.name == name) {
        return Err(format!("Мультисиг‑кошелёк с именем '{}' уже существует", name));
    }
    if required == 0 {
        return Err("Число требуемых подписей должно быть не меньше 1".into());
    }
    if (required as usize) > participants.len() {
        return Err("Требуемых подписей больше, чем участников".into());
    }
    store.multisig_wallets.push(MultisigWallet {
        name: name.to_string(),
        address: address.to_string(),
        participants,
        required,
    });
    save_store(&store, password).map_err(|e| e.to_string())
}

/// Remove a multisig wallet by name.
#[tauri::command]
pub fn remove_multisig_wallet(password: &str, name: &str) -> CmdResult<()> {
    let mut store = open_store(password).map_err(|e| e.to_string())?;
    let before = store.multisig_wallets.len();
    store.multisig_wallets.retain(|w| w.name != name);
    if store.multisig_wallets.len() == before {
        return Err(format!("Мультисиг‑кошелёк '{}' не найден", name));
    }
    save_store(&store, password).map_err(|e| e.to_string())
}

/// Return a vector with all stored multisig wallets.
#[tauri::command]
pub fn list_multisig_wallets(password: &str) -> CmdResult<Vec<MultisigWallet>> {
    let store = load_store(password).map_err(|e| e.to_string())?;
    Ok(store.multisig_wallets)
}

/// Return a vector of all stored accounts (after decryption).
#[tauri::command]
pub fn list_accounts(password: &str) -> CmdResult<Vec<WalletEntry>> {
    let store = load_store(password).map_err(|e| e.to_string())?;
    Ok(store.accounts)
}

/// Change the master password: decrypt with the old one, re‑encrypt with the new one.
#[tauri::command]
pub fn change_password(old_password: &str, new_password: &str) -> CmdResult<()> {
    if new_password.trim().is_empty() {
        return Err("Новый пароль не может быть пустым".into());
    }
    let store = load_store(old_password).map_err(|_| "Неверный текущий пароль".to_string())?;
    save_store(&store, new_password).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn init_store(password: &str) -> CmdResult<bool> {
    if Path::new(STORE_FILE).exists() {
        // Verify password by trying to load the store
        load_store(password).map_err(|e| e.to_string())?;
        Ok(true)
    } else {
        create_new_store(password).map_err(|e| e.to_string())?;
        Ok(true)
    }
}
