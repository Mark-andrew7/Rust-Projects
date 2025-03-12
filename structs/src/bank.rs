struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        if self.balance >= amount {
           self.balance -= amount;
           true 
        }
        else {
            false
        }
    }

    fn display_balance(&self) {
        println!("Current balance is {}", self.balance)
    }
}