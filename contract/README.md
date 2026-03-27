# 🚀 Crypto Subscriptions (Permissionless DApp)

## 📌 Project Description

Crypto Subscriptions is a fully permissionless decentralized application built on the Stellar network using Soroban smart contracts. It enables anyone to create, manage, and subscribe to recurring payment plans without relying on centralized intermediaries.

This project demonstrates how recurring payment systems (like Netflix, SaaS tools, or memberships) can be implemented in a decentralized and trustless way.

---

## ⚙️ What it does

- Allows any user to create a subscription plan with:
  - Price
  - Billing interval (in seconds)

- Allows any user to subscribe to any plan

- Tracks subscription cycles and next payment timestamps

- Enables recurring payments via on-chain logic

- Eliminates the need for centralized subscription platforms

---

## ✨ Features

### 🔓 Fully Permissionless
- No admin controls
- No ownership restrictions
- Anyone can create or join subscription plans

### 🔁 Recurring Payments
- Time-based subscription model
- Automatic next payment tracking

### 🌐 Decentralized
- Built on Stellar using Soroban smart contracts
- No centralized authority or gatekeeping

### ⚡ Lightweight & Efficient
- Minimal contract logic
- Gas-efficient storage usage

### 🧩 Extensible
Future upgrades can include:
- Token-based payments
- Auto-payment bots
- Subscription NFTs
- Multi-token support

---

## 🔗 Deployed Smart Contract Link

Crypto Subscriptions:  
👉 (Add your Stellar contract deployment link here)

---

## 🛠 Tech Stack

- Stellar Blockchain
- Soroban Smart Contracts (Rust)
- (Optional Frontend) Next.js + Stellar Wallet Integration

---

## 🚀 How to Use

1. Deploy the smart contract on Stellar Soroban
2. Call `create_plan(price, interval)`
3. Users call `subscribe(plan_id)`
4. Call `pay(subscription_id)` periodically to process payments

---

## 📌 Future Improvements

- Automated payment execution (cron bots)
- Token transfers using Stellar asset contracts
- UI dashboard for managing subscriptions
- Wallet integration (Freighter)

---

## 🤝 Contribution

This is an open, permissionless project — contributions are welcome!

