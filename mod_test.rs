use realy_core::{Transaction, TransactionState, EvaEscrow}; // Importa do core
use pretty_assertions::assert_eq;

#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub valid: bool,
}

impl Transaction {
    pub fn validate(&mut self) {
        self.valid = !self.from.is_empty() && !self.to.is_empty() && self.amount > 0.0;
    }
}


    assert!(tx.is_valid());
    eva.register_transaction(tx.clone());

    assert_eq!(eva.ledger.len(), 1);
    assert_eq!(eva.reputation.get("alice").unwrap().completed, 1);
}

#[test]
fn test_invalid_transaction_blocked() {
    let mut eva = EvaEscrow::new();

    let tx = Transaction {
        id: "tx_test_02".to_string(),
        from: "bob".to_string(),
        to: "".to_string(), // inválido
        amount: 75.0,
        state: TransactionState::Pending,
    };

    assert!(!tx.is_valid());
    eva.register_transaction(tx.clone());

    assert_eq!(eva.ledger.len(), 0); // Não foi registrada
}
