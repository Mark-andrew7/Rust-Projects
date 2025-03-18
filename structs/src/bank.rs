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
        println!("Current balance for {} is {}", self.owner, self.balance)
    }
}

fn main() {
    let mut account = BankAccount {
        owner: String::from("Mark"),
        balance: 100.0,
    };

    account.display_balance();

    account.deposit(50.0);
    println!("After depositing 50:");
    account.display_balance();

    let withdrawal_success = account.withdraw(30.0);
    println!("Withdrawal of 30 successful: {}", withdrawal_success);
    account.display_balance();


    let withdrawal_success = account.withdraw(200.0);
    println!("Withdrawal of 200 successful: {}", withdrawal_success);
    account.display_balance();


}