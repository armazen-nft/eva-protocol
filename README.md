# EVA Protocol

**EVA** (Escrow Voluntário da Rede) é o núcleo lógico de uma wallet descentralizada P2P focada em anonimato, segurança e autonomia financeira.  
Foi concebida para operar fora da estrutura de vigilância estatal, facilitando trocas diretas entre indivíduos por meio de contratos simples e rastreamento mínimo.

---

## ✨ Visão

Inspirada por ideais libertários e tecnologias abertas, EVA busca oferecer:

- Transações anônimas e auditáveis
- Módulo de escrow voluntário embutido
- Lógica baseada em FSM (Finite State Machine)
- Operação segura e offline-ready
- Código modular, limpo e extensível

---

## 🧠 Funcionalidades

- [x] Criação de transações entre pares
- [x] Estados: `Initiated`, `InEscrow`, `Released`, `Completed`, `Disputed`, `Cancelled`, `Refunded`
- [x] Funções: `deposit_escrow()`, `release()`, `dispute()`, `refund()`, `force_complete()`, `expire()`
- [x] Registro local simulado com `HashMap`

---

## 🚀 Instruções rápidas

Requisitos:
- [Rust](https://www.rust-lang.org/tools/install)

Clone e execute:

```bash
git clone https://github.com/seu-usuario/eva-protocol.git
cd eva-protocol
cargo run
