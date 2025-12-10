struct BankAccount{
    holder_name: String,
    balance: u32,
    is_active: bool
}

impl BankAccount{
    fn new(holder_name: String) ->Self{
        BankAccount{
            holder_name,
            balance:0,
            is_active:true
        }
    }
    fn deposit(&mut self,amount: u32) {
        self.balance += amount;
    }
    fn withdraw(&mut self,amount: u32)->bool {
        if self.balance<amount {
            false
        }else{
            self.balance -= amount;
            true
        }
    }
    fn info(&self){
        println!("Holder name: {}",self.holder_name);
        println!("Balance: {}",self.balance);
        println!("Active: {}",self.is_active);
    }
}


fn main(){
        
    let mut create_acc = BankAccount::new(String::from("alik"));
    create_acc.deposit(100);
    //тут можно додумать что делать если false возвращаеться
    create_acc.withdraw(50);
    create_acc.info();
}
