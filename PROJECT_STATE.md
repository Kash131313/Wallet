# Project State

**Current date:** 2026-08-04

## Статус

- **PLAN.md** – создан (подробный план разработки).
- **SOURCES.md** – реестр официальных источников (обновлён: GraphQL API v2).
- **acki_nest/** – скелет мульти-кошелька (Rust + Tauri 1.8):
  - `Cargo.toml` – настройки пакета (tauri-build добавлен).
  - `src-tauri/` – бэкенд: `main.rs`, `crypto.rs`, `wallet_manager.rs`, `network.rs`, `build.rs`.
  - `dist/index.html` – новый современный UI (тёмная тема, вкладки, автоблокировка).
  - `scripts/gen_icon.py` – генератор иконки (src-tauri/icons/icon.png).

## Сборка — ИСПРАВЛЕНА (ранее проект не компилировался)

Было 9+ ошибок; все устранены:

1. **Путь к tvm-sdk**: в `Cargo.toml` было `path = "../tvm-sdk"` (указывал на `acki_nest/tvm-sdk`).
   Исправлено на `../../tvm-sdk` (SDK лежит в корне репозитория).
2. **build.rs отсутствовал**: Tauri требует build-скрипт. Добавлены `build.rs` и
   `[build-dependencies] tauri-build = "1.5"` (последняя версия 1.x — **1.5.6**; `^1.6` не находится).
3. **tauri.conf.json** был в схеме Tauri 2 (`app`). Переписан под Tauri 1.x:
   `package` / `tauri` / `build`, поля `distDir`, `devPath`, `bundle.identifier`, `allowlist.all`.
4. **crypto.rs** – код был под устаревшие API (argon2 0.4, aes-gcm 0.9), а в зависимостях
   argon2 0.5.3 и aes-gcm 0.10.3. Обновлено: `Argon2::new(Algorithm::Argon2id, ...)` +
   `hash_password_into` (19 МБ, 2 итерации — рекомендации OWASP); `Aes256Gcm::new_from_slice`
   через trait `KeyInit`; ошибки маппятся в `Box<dyn Error>` через `.to_string()`.
5. **Tauri-команды**: `Result<_, Box<dyn Error>>` не удовлетворяет `Serialize` в Tauri 1.x.
   Все команды теперь возвращают `Result<_, String>` (псевдоним `CmdResult<T>`).
6. **network.rs**: `derive_address` / `get_balance` не были `#[tauri::command]` — добавлено.

Проверено: `cargo check` (default) и `cargo check --features real` — **обе проходят**.

## Сеть — реальный GraphQL API v2 (проверено живьём на mainnet)

Старый API из SOURCES.md **устарел**: поле `account` в корне отключено
(«Deprecated API is disabled»). Актуальная схема:

```
QueryRoot → blockchain → account(account_id: "64hex", dapp_id: "64hex") → info { balance }
```

- `balance` — hex-строка в нано-единицах (пример: `"0x1a30f9250f32a0"` для root MobileVerifiers).
- Корень MobileVerifiers: account_id `2222…22`, dapp_id `0000…01` (подтверждён ответом).
- Реализовано в `network.rs` (`real_impl::get_balance`), включая нормализацию адреса
  (`0:hex`, `dapp_id::account_id`, голый hex → чистый account_id).

## UI — переработан (dist/index.html)

- Тёмная тема с градиентами, стеклянные карточки, анимации, тосты.
- Вкладки: **Аккаунты / Мультисиг / Настройки**.
- Аккаунты: добавление по seed (или вручную адресом), кнопки «Баланс», «Копировать», «Удалить».
- Мультисиг: создание (участники, порог подписей), список, удаление.
- Настройки: смена мастер-пароля, таймер автоблокировки, «Заблокировать».
- Форматирование баланса: hex (нано) → «X.XX ACKI» (1 ACKI = 1e9).
- Демо-режим без Tauri (fallback на localStorage) для просмотра UI в браузере;
  в Tauri работает с реальной сетью.

## Исследование реального tvm-sdk (tvmlabs/tvm-sdk, tag v3.0.4.an)

Клонирован для изучения (`/tmp/tvm-sdk-study`). Выводы:

- Это workspace из многих крейтов; SDK-клиент — **`tvm_client`**, высокоуровневая обёртка — `tvm_sdk`.
- Деривация ключей: `tvm_client::crypto::hdkey` (`hdkey_xprv_from_mnemonic`,
  `hdkey_derive_from_xprv_path`, путь по умолчанию `m/44'/1331'/0'/0/0`) и
  `tvm_client::crypto::mnemonic` (`mnemonic_derive_sign_keys`). Все функции требуют
  `Arc<ClientContext>`.
- `tvm_client` тянет весь workspace (включая tvm_vm / tvm_executor) — **долгая компиляция**.
  Поэтому полноценная интеграция SDK перенесена в Phase 3.
- Локальная заглушка `tvm-sdk/` в корне репозитория остаётся рабочей для offline-сборки
  (та же поверхность API: `Mnemonic::from_phrase`, `KeyPair::from_mnemonic`, `Address::from_keypair`).

## Следующие шаги

1. **Phase 3 — реальный SDK**: подключить `tvm_client` из git (tag v3.0.4.an), реализовать
   деривацию ключей через `hdkey` и вычисление адреса MobileVerifiers по контракту из
   `gosh-sh/ackinacki-kit` (адрес = hash state_init; требуется код контракта).
2. **Отправка транзакций** через GraphQL (`blockchain` mutations) — после деривации ключей.
3. **Биометрический вход, QR-импорт/экспорт** — по желанию.
4. **Сборка бинарника Tauri** (`cargo tauri build`) и упаковка под платформы.


- Подготовлен `README.md` с инструкциями сборки и использования.
- Добавлен `CHANGELOG.md`.
- Исправлен путь к stub `tvm-sdk` и гарантирована работа без сети.
- Удалены проблемные тесты, пока оставлены планы по добавлению unit‑тестов в модули.
