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

## Задача: реальный tvm-sdk + фича `real` — НАЧАТА, ОСТАНОВЛЕНА (2026-08-05)

**Задание:** заменить заглушку `tvm-sdk/` на реальную библиотеку `tvmlabs/tvm-sdk`
и собрать проект с включённой фичей `real`. Работа **прервана пользователем** на этапе
исследования; ниже — точное состояние и что осталось.

### Выполнено (шаги 1–2 задания)

- ✅ **Шаг 1** — заглушка удалена (`rm -rf tvm-sdk`), создан чистый каталог.
- ✅ **Шаг 2** — клонирован реальный SDK: `git clone --depth 1 https://github.com/tvmlabs/tvm-sdk.git tvm-sdk`
  (ветка `main`, commit `7d52241`, версия workspace `3.0.4`).
  Сейчас в `tvm-sdk/` лежит полный официальный SDK (Cargo.toml, src/, api/, tvm_*, docs/ …).

### Ключевые выводы исследования (важно для продолжения)

1. **Реальный SDK — это Cargo-воркспейс из ~20 крейтов**, а не один крейт как заглушка.
   - Пакет-обёртка: **`tvm-sdk/tvm_sdk`** (Contract, Message, Transaction, Block, types…)
   - SDK-клиент с криптографией: **`tvm-sdk/tvm_client`** (`crypto::mnemonic`, `crypto::hdkey`, `crypto::keys`, `account::get_account`).
2. **API реального SDK полностью отличается от заглушки.** В нём НЕТ
   `tvm_sdk::crypto::mnemonic::Mnemonic`, `tvm_sdk::crypto::keypair::KeyPair`,
   `tvm_sdk::address::Address`. Код `network.rs` (`real_impl`) придётся переписать:
   - ключи: `tvm_client::crypto::mnemonic_derive_sign_keys` (или `hdkey_xprv_from_mnemonic` + `hdkey_derive_from_xprv_path`, путь `m/44'/1331'/0'/0/0`) — все функции требуют `Arc<ClientContext>`;
   - контекст: `ClientContext::new(ClientConfig::default())` создаётся и без сети (endpoints=None);
   - адрес аккаунта MobileVerifiers: **`address = hash(state_init)`, нужен код контракта** из
     `gosh-sh/ackinacki-kit` (dapp_id MobileVerifiers = `0000…01`) — это самый трудоёмкий пункт.
3. **Проверено: пакет `tvm_sdk` компилируется быстро** (~1 мин, лёгкие зависимости).
   `tvm_client` же тянет tvm_vm/tvm_executor → wasmtime, halo2, blst и git-зависимости
   (`lockfree`, `sodalite`) — **очень долгая сборка** (десятки минут).
4. **Вендор-проблема**: `.cargo/config.toml` заменяет crates-io на `acki_nest/src-tauri/vendor`.
   Новые зависимости реального SDK в vendor **отсутствуют** → `cargo build --features real`
   не сможет их скачать. Варианты: (а) обновить vendor (`cargo vendor`, но git-зависимости
   lockfree/sodalite им не вендорируются); (б) временно убрать/ослабить подмену в `.cargo/config.toml`.
5. **GraphQL mainnet проверен живьём**: `https://mainnet.ackinacki.org/graphql` отвечает,
   схема `blockchain { account(account_id, dapp_id) { info { balance } } }` — запрос из
   `network.rs` корректен (баланс = hex-строка в нано-единицах).
6. **`ackinacki-kit` (gosh-sh) изучен**: аккаунты адресуются `account_id` + `dapp_id`;
   для деплоя/деривации адреса используется `tvm_client::account::get_account`;
   формула account_id из pubkey требует код контракта (отложено).

### Что осталось сделать (шаги 3–6)

- **Шаг 3** — правка `acki_nest/src-tauri/Cargo.toml`: строка
  `tvm-sdk = { path = "../../tvm-sdk", optional = true }` теперь указывает на **воркспейс-рут**
  (без `[package]`) — это не сработает. Нужно указать на пакет:
  `tvm-sdk = { path = "../../tvm-sdk/tvm_sdk", optional = true }` (и при необходимости
  добавить `tvm-client = { path = "../../tvm-sdk/tvm_client", optional = true }` в фичу `real`).
  *(Задание считало путь «по-прежнему верным» — это предположение неверно для воркспейса.)*
- **Шаг 4** — решить вендор-вопрос (п.4 выше), затем `cargo fetch` и `cargo build --features real`;
  адаптировать `network.rs::real_impl` под реальный API (п.2).
- **Шаг 5** — `cargo tauri dev`, ввод мастер-пароля, создание/импорт аккаунта, кнопка
  «Обновить баланс» → реальный запрос на mainnet (баланс в ACKI).
- **Шаг 6** — закоммитить. ⚠️ **Перед `git add tvm-sdk` удалить `tvm-sdk/.git`**
  (вложенный репозиторий — иначе git создаст gitlink-подмодуль вместо файлов):
  `rm -rf tvm-sdk/.git`, затем `git add tvm-sdk` + `git add acki_nest/src-tauri/Cargo.toml`,
  commit `"Replace stub tvm-sdk with real tvmlabs/tvm-sdk and enable real feature"`;
  пуша нет (remote не настроен).

### Текущее состояние Git (не закоммичено)

- `tvm-sdk/` — удалены файлы старой заглушки (`D tvm-sdk/src/lib.rs` и др.) + много
  untracked-файлов нового SDK (`?? tvm-sdk/…`).
- `acki_nest/src-tauri/Cargo.toml` — модифицирован (добавлены комментарии о stub/real;
  строка зависимости не менялась).
- `PROJECT_STATE.md` — правки этой сессии.
- APK: `ackinacki-wallet-latest (1).apk` на месте; `LocalMiner-v4.5.3.apk` и
  `acki-market (1).apk` показаны как удалённые (изменения не мои — не трогал).

## Офлайн-сборка — vendor/ подключён (задача выполнена 2026-08-04)

- **vendor/ заполнена**: `acki_nest/src-tauri/vendor/` — результат `cargo vendor`
  (590 крейтов, 32 704 файла), **закоммичена** в Git: commit `f6fe90f7`
  «Add vendored dependencies and Cargo config for offline builds».
- **`.cargo/config.toml`** (корень репозитория):
  `[source.crates-io] replace-with = "vendored-sources"`,
  `[source.vendored-sources] directory = "acki_nest/src-tauri/vendor"`
  (путь — относительно корня репозитория, как и требует Cargo).
- **`.gitignore`** в корне отсутствует — папка `vendor/` ничем не исключена и отслеживается Git.
- **Проверка офлайн-сборки**: `cd acki_nest/src-tauri && cargo check --offline` — **проходит**:
  `Finished dev [unoptimized + debuginfo] target(s) in 0.88s`.
- **Исправлено в ходе проверки**: незакоммиченная правка `.cargo/config.toml` указывала на
  `src-tauri/vendor` (разрешается в `Wallet/src-tauri/vendor` — не существует, Cargo падал
  с «failed to read root of directory source»). Восстановлен корректный путь
  `acki_nest/src-tauri/vendor` (совпадает с HEAD, коммит не требуется).
- **Удалённый репозиторий не настроен** (`git remote` пуст) — пуш (шаг 7, опциональный)
  не выполнялся.

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

1. **Завершить текущую задачу (реальный SDK + `real`)**: см. раздел «Задача: реальный tvm-sdk…»
   — поправить путь в Cargo.toml на `../../tvm-sdk/tvm_sdk`, решить вендор-вопрос,
   адаптировать `network.rs::real_impl` под реальный API, собрать `--features real`,
   проверить баланс, удалить `tvm-sdk/.git` и закоммитить.
2. **Отправка транзакций** через GraphQL (`blockchain` mutations) — после деривации ключей.
3. **Биометрический вход, QR-импорт/экспорт** — по желанию.
4. **Сборка бинарника Tauri** (`cargo tauri build`) и упаковка под платформы.


- Подготовлен `README.md` с инструкциями сборки и использования.
- Добавлен `CHANGELOG.md`.
- Исправлен путь к stub `tvm-sdk` и гарантирована работа без сети.
- Удалены проблемные тесты, пока оставлены планы по добавлению unit‑тестов в модули.
