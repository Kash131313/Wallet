# Официальные источники Acki Nacki (реестр проекта)

**Правило (по требованию владельца проекта):** единственный источник правды.
Никаких догадок и домыслов — любые технические факты (ABI, контракты, токены, сетевая
модель, SDK) брать только из этих источников. Составлено: 2026-08-04.

---

## 1. Основные ресурсы (официальные)

| Ресурс | URL | Что там |
|---|---|---|
| Сайт | https://ackinacki.com/ | Главный портал, обзор сети |
| Кошелёк | https://ackinacki.com/wallet | Официальный кошелёк (функции, ссылки) |
| Документация | https://docs.ackinacki.com/ | Вся документация сети |
| Токены (ECC) | https://docs.ackinacki.com/glossary#extra-currency-collection | NACKL, SHELL, VMSHELL, USDC — мультивалютная модель |
| Для разработчиков | https://dev.ackinacki.com/ | Гайды: Dapp ID, развёртывание контрактов, GraphQL |
| GOSH docs | https://docs.gosh.sh/ | Протокол, консенсус, архитектура |
| GitHub | https://github.com/ackinacki | Узел сети (C), acki-nacki-igniter (Rust) |
| GitHub | https://github.com/tvmlabs | tvm-sdk (Rust/JS), инструменты TVM |
| GitHub | https://github.com/gosh-sh | SDK, контракты, компиляторы, ноды, ackinacki-kit |

## 2. Ключевые репозитории (проверено 04.08.2026 через GitHub API)

### gosh-sh
- **`gosh-sh/ackinacki-kit`** — ⭐ официальный Rust SDK + исходники контрактов Acki Nacki.
  Активен (последний коммит 2026-07-28). Структура `contracts/src`:
  `account.rs`, `dapp.rs`, `authservice/` (profile, root), `multisig/`, `mvsystem/`
  (multifactor, miner, indexer, mirror, popcoin, popitgame, root), `token/`
  (root, wallet, transaction), `accumulator/` (shell, usdc), `exchange/`, `bksystem/`,
  `mvconfig/`, `giver/`, `traits.rs`, `error.rs`, `event.rs`. Крейты: `root/`, `shared/`.
- `gosh-sh/acki-nacki-public` — публичная конфигурация и инструменты сети.
- `gosh-sh/docs.ackinacki.com` — исходники документации.
- `gosh-sh/TVM-Solidity-Compiler` — компилятор Solidity → TVM (.tvc/.abi).
- `gosh-sh/tonos-cli` — CLI для TVM-сетей (у пользователя установлен как `tvm-cli`).
- `gosh-sh/ever-sdk` — клиенты на 13 языках для TVM-сетей.
- `gosh-sh/ever-sdk-js` / `gosh-sh/flex-sdk-js` — JS SDK.
- `gosh-sh/ever-node`, `ever-vm`, `ever-executor`, `ever-block`, `ever-abi`, `ever-types`,
  `ever-crypto`, `ever-adnl`, `ever-rldp`, `ever-overlay`, `ever-tl`, `ever-block-json`,
  `ever-node-tools` — компоненты ноды/ВМ на Rust.
- `gosh-sh/evernode-se` — локальный эмулятор сети (безопасная разработка).
- `gosh-sh/evernode-ds` — community supernode с GraphQL API.
- `gosh-sh/ton-q-server` — GraphQL API для TVM-сетей.
- `gosh-sh/gosh` — блокчейн GOSH.

### tvmlabs
- `tvmlabs/tvm-sdk` — «Client Libraries and CLI for Acki-Nacki, Venom, Everscale, TON» (Rust) —
  наш сетевой слой.
- `tvmlabs/tvm-sdk-js` — JS-версия.
- `tvmlabs/ever-node` — нода в Rust; `tvmlabs/ton-q-server` — GraphQL.
- `tvmlabs/sdk-examples` — примеры.

### ackinacki
- `ackinacki/ackinacki` — реализация ноды (Block Keeper/Block Manager), язык C.
- `ackinacki/acki-nacki-igniter` — Rust (инструменты сети).

## 3. Сеть (mainnet)

- GraphQL: **https://mainnet.ackinacki.org/graphql** — проверено: HTTP 200, запросы работают.
  Версия API 1.2.0. **Старый API отключён**: поле `account` в корне QueryRoot больше
  не работает («Deprecated API is disabled»). Использовать **API v2**:
  `QueryRoot → blockchain → account(account_id, dapp_id) → info { ... }`,
  где `account_id` = 64 hex без префикса `0:`, `dapp_id` = 64 hex.
  **Живая проверка (04.08.2026):** `{ blockchain { account(account_id: "2222…22",
  dapp_id: "0000…01") { info { balance } } } }` → `balance: "0x1a30f9250f32a0"`
  (hex, нано-единицы; 1 ACKI = 1e9). Поля `info`: `balance`, `address`, `dapp_id`,
  `code_hash`, `data_hash`, `last_paid`, `acc_type` и др.
- **Системные Dapp ID** (из gosh-sh/ackinacki-kit, contracts/src/dapp.rs):
  - System (BK/BM, giver): `0000000000000000000000000000000000000000000000000000000000000000`
  - **MobileVerifiers (официальный кошелёк): `0000000000000000000000000000000000000000000000000000000000000001`**
  - AuthService: `0000000000000000000000000000000000000000000000000000000000000002`
  - Dex: `0000000000000000000000000000000000000000000000000000000000000004`
- Корень MobileVerifiers: `0:2222222222222222222222222222222222222222222222222222222222222222`.
- Резервные: `mainnet-cf.ackinacki.org` (HTTP 200) + узлы `IP:8600` (из конфига майнера:
  94.156.178.13/.24/.25/.202/.210).
- tvm-sdk для acki nacki: tvmlabs/tvm-sdk, **tag v3.0.4.an**.
- **Формат адреса SDK 3 (tvm-cli/SDK): `dapp_id::account_id`** — два 64-hex через `::`, без
  workchain/`0x`; префикс `0:` не принимается. В GraphQL — аргументы `account_id` + `dapp_id`.
  Рабочая инструкция по балансу: research/проверка-баланса-инструкция.md.
- Тестнет `shellnet.ackinacki.org` — **не используем** (работаем только на mainnet).
- Живой тест аккаунта: research/живой-тест-mainnet.md.

## 4. Как применять

| Задача | Источник |
|---|---|
| ABI контрактов (multifactor, named-wallet, token, dapp, miner) | `gosh-sh/ackinacki-kit` (contracts/src) |
| Сетевой слой, подпись, GraphQL | `tvmlabs/tvm-sdk` (Rust) |
| Токены NACKL/SHELL/VMSHELL/USDC | docs.ackinacki.com/glossary#extra-currency-collection |
| Dapp ID, развёртывание | dev.ackinacki.com |
| Консенсус/архитектура | docs.gosh.sh, ackinacki.com |
| Проверка фактов | Всегда сверяться с таблицей §1–§2 перед использованием |

