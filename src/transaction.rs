
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionState {
    Initiated,
    AwaitingEscrow,
    InEscrow,
    Released,
    Disputed,
    Cancelled,
    Completed,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub state: TransactionState,
    pub escrow_agent: Option<String>,
    pub refunded: bool,
    pub expired: bool,
}

impl Transaction {
    pub fn new(id: &str, from: &str, to: &str, amount: f64) -> Self {
        Self {
            id: id.to_string(),
            from: from.to_string(),
            to: to.to_string(),
            amount,
            state: TransactionState::Initiated,
            escrow_agent: None,
            refunded: false,
            expired: false,
        }
    }

    pub fn enable_escrow(&mut self, agent: &str) {
        self.escrow_agent = Some(agent.to_string());
        self.state = TransactionState::AwaitingEscrow;
    }

    pub fn deposit_escrow(&mut self) {
        if self.escrow_agent.is_some() {
            self.state = TransactionState::InEscrow;
        }
    }

    pub fn release(&mut self) {
        self.state = TransactionState::Released;
    }

    pub fn complete(&mut self) {
        self.state = TransactionState::Completed;
    }

    pub fn dispute(&mut self) {
        self.state = TransactionState::Disputed;
    }

    pub fn cancel(&mut self) {
        self.state = TransactionState::Cancelled;
    }

    pub fn refund(&mut self) {
        if matches!(self.state, TransactionState::Cancelled | TransactionState::Disputed) && !self.refunded {
            self.refunded = true;
            println!("🔁 Transação {} reembolsada para {}", self.id, self.from);
        } else {
            println!("⚠️ Transação {} não pode ser reembolsada", self.id);
        }
    }

    pub fn force_complete(&mut self) {
        if matches!(self.state, TransactionState::Released | TransactionState::InEscrow) {
            self.state = TransactionState::Completed;
            println!("🛠️ Transação {} forçada para 'Completed'", self.id);
        }
    }

    pub fn expire(&mut self) {
        if matches!(self.state, TransactionState::Initiated | TransactionState::AwaitingEscrow) {
            self.expired = true;
            self.state = TransactionState::Cancelled;
            println!("⏰ Transação {} expirada", self.id);
        }
    }
}
