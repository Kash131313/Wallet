/// Нормализация адреса: из любого формата (`0:hex`, `dapp_id::account_id`, голый hex)
/// извлекаем чистый 64‑символьный account_id (без префикса воркчейна и dapp_id).
#[cfg_attr(not(feature = "real"), allow(dead_code))]
pub fn normalize_account_id(address: &str) -> String {
    let addr = address.trim();
    // Формат SDK 3: "dapp_id::account_id" – берём часть после "::".
    if let Some(idx) = addr.find("::") {
        return addr[idx + 2..].to_string();
    }
    // Формат "0:hex" / "-1:hex" – убираем префикс.
    let without_wc = if let Some(idx) = addr.find(':') {
        &addr[idx + 1..]
    } else {
        addr
    };
    // Убираем возможный префикс "0x".
    let hex = without_wc.strip_prefix("0x").unwrap_or(without_wc);
    hex.to_string()
}

// ---------- Реальная реализация (включается флагом "real") ----------
#[cfg(feature = "real")]
mod real_impl {
    use super::normalize_account_id;
    use reqwest::blocking::Client;
    use serde_json::json;
    use tvm_sdk::crypto::mnemonic::Mnemonic;
    use tvm_sdk::crypto::keypair::KeyPair;
    use tvm_sdk::address::Address;

    /// Вычисление адреса из seed‑фразы.
    /// ВАЖНО: сейчас используется локальная заглушка tvm-sdk. Подключение
    /// реального tvmlabs/tvm-sdk (tag v3.0.4.an) планируется в Phase 3 вместе
    /// с контрактами MobileVerifiers из gosh-sh/ackinacki-kit.
    #[tauri::command]
    pub fn derive_address(seed: &str) -> Result<String, String> {
        let mnemonic = Mnemonic::from_phrase(seed).map_err(|e| e.to_string())?;
        let keypair = KeyPair::from_mnemonic(&mnemonic).map_err(|e| e.to_string())?;
        let address = Address::from_keypair(&keypair);
        Ok(address.to_string())
    }

    /// Получение баланса аккаунта через официальный GraphQL API (v2):
    /// `QueryRoot -> blockchain -> account(account_id, dapp_id) -> info { balance }`.
    /// Баланс возвращается как hex‑строка в нано‑единицах (например "0x1a30f9250f32a0").
    #[tauri::command]
    pub fn get_balance(address: &str) -> Result<String, String> {
        const DAPP_ID: &str = "0000000000000000000000000000000000000000000000000000000000000001";
        const GRAPHQL_ENDPOINT: &str = "https://mainnet.ackinacki.org/graphql";

        let account_id = normalize_account_id(address);
        if account_id.len() != 64 {
            return Err(format!(
                "Некорректный account_id: ожидается 64 hex‑символа, получено {}",
                account_id.len()
            ));
        }

        let query = format!(
            "{{ blockchain {{ account(account_id: \"{account}\", dapp_id: \"{dapp}\") {{ info {{ balance }} }} }} }}",
            account = account_id,
            dapp = DAPP_ID,
        );
        let payload = json!({ "query": query });
        let client = Client::new();
        let response = client
            .post(GRAPHQL_ENDPOINT)
            .json(&payload)
            .send()
            .map_err(|e| format!("Сетевая ошибка: {}", e))?;
        let resp_json: serde_json::Value = response.json().map_err(|e| format!("Ответ сервера: {}", e))?;

        if let Some(balance) = resp_json["data"]["blockchain"]["account"]["info"]["balance"].as_str()
        {
            Ok(balance.to_string())
        } else {
            let msg = resp_json["errors"][0]["message"]
                .as_str()
                .unwrap_or("неизвестная ошибка")
                .to_string();
            Err(format!("GraphQL: {}", msg))
        }
    }
}

// ---------- Заглушка (используется, когда флаг "real" отключён) ----------
#[cfg(not(feature = "real"))]
mod stub_impl {
    /// Демонстрационная деривация: base64‑кодирование seed‑фразы.
    #[tauri::command]
    pub fn derive_address(seed: &str) -> Result<String, String> {
        use base64::Engine as _;
        Ok(base64::engine::general_purpose::STANDARD.encode(seed))
    }

    /// Нулевой баланс – реального запроса нет (офлайн‑режим).
    #[tauri::command]
    pub fn get_balance(_address: &str) -> Result<String, String> {
        Ok("0".to_string())
    }
}

// Публичные функции, которые выбирают реализацию в зависимости от флага.
#[cfg(feature = "real")]
pub use real_impl::{derive_address, get_balance};

#[cfg(not(feature = "real"))]
pub use stub_impl::{derive_address, get_balance};
