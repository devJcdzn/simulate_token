# 🪙 Token Simulado na Solana

Smart contract desenvolvido com [Anchor](https://book.anchor-lang.com/) que simula a criação e transferência de tokens na blockchain Solana, sem depender do padrão SPL. Ideal para fins educacionais e demonstrações técnicas.

---

## 📌 Objetivo

Permitir que qualquer usuário:

- Crie um token personalizado com nome, símbolo e supply inicial.
- Gere contas para armazenar saldo de um token específico.
- Transfira tokens entre usuários com verificação de saldo.
- Tenha controle de acesso sobre suas próprias contas.

---

## ⚙️ Tecnologias

- 🦀 Rust
- ⚓ Anchor Framework
- 🧬 Solana Program Library (sem SPL Token diretamente)
- 🧪 Anchor Test Suite (TypeScript)

---

## 🧠 Conceitos Demonstrados

- Criação de contas on-chain com tipos personalizados
- Assinatura e validação de propriedade (`Signer`, `has_one`)
- Controle de saldo e simulação de transferência
- Segurança contra transferências inválidas
- Arquitetura limpa com tipagem forte

---

## 🏗️ Estrutura do Contrato

### 1. TokenMetadata

Armazena os dados de um token criado:

```rust
pub struct TokenMetadata {
  pub name: String,
  pub symbol: String,
  pub total_supply: u64,
  pub creator: Pubkey,
}
```

---

### 2. UserTokenAccount

Representa o saldo de um usuário em um token específico:

```rust
pub struct UserTokenAccount {
  pub owner: Pubkey,
  pub token: Pubkey,
  pub balance: u64,
}
```

---

## 🚀 Funcionalidades

### 🔹 `register_token(name, symbol, supply)`
Cria um novo token e inicializa o saldo do criador com o `supply` definido.

### 🔹 `create_user_token_account(token)`
Cria uma conta de saldo zerado para qualquer usuário e token.

### 🔹 `transfer(amount)`
Transfere saldo entre duas contas de token com validação de saldo, dono e consistência.

---

## 🧪 Testes Locais

Para testar localmente:

```bash
solana-test-validator
anchor build
anchor test
```

---

## 📦 Requisitos de espaço nas contas

| Conta               | Tamanho aproximado |
|---------------------|--------------------|
| `TokenMetadata`     | ~96 bytes          |
| `UserTokenAccount`  | ~80 bytes          |

---

## 🧱 Extensões Futuras

- ✅ Mint e Burn controlados pelo criador
- ✅ Taxa de transferência opcional
- ✅ Indexador com feed de transferências
- ✅ Frontend com React + Phantom Wallet

---

## ✍️ Autor

> Desenvolvido como projeto técnico demonstrativo por Jean Carlos, com foco em clareza, segurança e arquitetura limpa.

---

## 📜 Licença

MIT
