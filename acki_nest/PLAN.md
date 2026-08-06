
## Next steps (after stub implementation)
- Подготовить `vendor/` с полным набором зависимостей (запуск `cargo vendor` на машине с интернетом).
- При желании подключить реальный `tvm-sdk` и собрать с `--features real`.
- Реализовать функции деплоя мультисиг‑контракта и подписи транзакций.
- Перейти к мобильной сборке (Android/iOS) через Tauri.

## ЗАДАЧА: реальный tvm-sdk + фича `real` (в процессе, остановлено 2026-08-05)

Полное описание — в корневом `PROJECT_STATE.md` (раздел «Задача: реальный tvm-sdk + фича real»).

### Статус
- [x] Шаг 1: заглушка `tvm-sdk/` удалена
- [x] Шаг 2: склонирован `https://github.com/tvmlabs/tvm-sdk.git` в `tvm-sdk/` (workspace 3.0.4, commit 7d52241)
- [ ] Шаг 3: поправить путь в `src-tauri/Cargo.toml` на `../../tvm-sdk/tvm_sdk` (сейчас `../../tvm-sdk` — воркспейс-рут, не пакет!)
- [ ] Шаг 4: решить вендор-вопрос + `cargo fetch` + `cargo build --features real` + адаптация `network.rs::real_impl`
- [ ] Шаг 5: `cargo tauri dev` — проверка реального баланса на mainnet
- [ ] Шаг 6: `rm -rf tvm-sdk/.git` (иначе gitlink!) + git add + commit

### Что нужно знать продолжающему
- Реальный API НЕ совпадает со заглушкой: нет `Mnemonic/KeyPair/Address`.
  Ключи: `tvm_client::crypto::mnemonic_derive_sign_keys(context, params)` (или hdkey-функции,
  путь `m/44'/1331'/0'/0/0`), контекст `ClientContext::new(ClientConfig::default())` (работает без сети).
- Адрес аккаунта MobileVerifiers = hash(state_init) — нужен код контракта из `gosh-sh/ackinacki-kit`.
- `tvm_client` тянет tvm_vm/tvm_executor (wasmtime/halo2/blst + git-зависимости lockfree, sodalite) — долгая сборка;
  сам пакет `tvm_sdk` компилируется быстро.
- Вендор: `.cargo/config.toml` заменяет crates-io на `acki_nest/src-tauri/vendor` — новых крейтов там нет.
  Варианты: обновить vendor (git-зависимости не вендорируются!) или временно ослабить конфиг.
- GraphQL mainnet работает: `blockchain { account(account_id, dapp_id) { info { balance } } }`.

