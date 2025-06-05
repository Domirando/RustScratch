//struct - a data structure that allows you to group multiple fields together under one name.

fn main(){
    let mut bank_account = BankAccount{
        owner: "Alice".to_string(),
        balance: 150.55,
    };
    bank_account.check_balance();
    bank_account.withdraw(50.48);
    bank_account.check_balance();
}
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount{
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }
}