mod transaction;

use transaction::Transaction;
use std::collections::HashMap;

fn simulate_network() {
    let mut ledger: HashMap<String, Transaction> = HashMap::new();

    let mut tx1 = Transaction::new("tx001", "Alice", "Bob", 100.0);
    tx1.enable_escrow("Eva");
    tx1.deposit_escrow();
    tx1.release();
    tx1.complete();

    let mut tx2 = Transaction::new("tx002", "Carlos", "Diana", 50.0);
    tx2.enable_escrow("Eva");
    tx2.deposit_escrow();
    tx2.dispute();
    tx2.refund();

    let mut tx3 = Transaction::new("tx003", "Elena", "Felipe", 80.0);
    tx3.enable_escrow("Eva");
    tx3.expire();

    let mut tx4 = Transaction::new("tx004", "Gustavo", "Helena", 120.0);
    tx4.enable_escrow("Eva");
    tx4.deposit_escrow();
    tx4.release();
    tx4.force_complete();

    ledger.insert(tx1.id.clone(), tx1.clone());
    ledger.insert(tx2.id.clone(), tx2.clone());
    ledger.insert(tx3.id.clone(), tx3.clone());
    ledger.insert(tx4.id.clone(), tx4.clone());

    for (id, tx) in ledger.iter() {
        println!(
            "ID: {}, From: {}, To: {}, Amount: {}, State: {:?}, Refunded: {}, Expired: {}",
            id, tx.from, tx.to, tx.amount, tx.state, tx.refunded, tx.expired
        );
    }
}

fn main() {
    println!("--- EVA - Escrow Voluntário da Rede ---");
    simulate_network();
}
