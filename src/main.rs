// // #[derive(Debug, Clone)]
// // struct BankStruct {
// //     id: String, // random id
// //     balance: u32,
// //     account_number: usize,
// //     holder: String,
// // }

// // #[derive(Debug, Clone)]
// // struct Bank {
// //     back: Vec<BankStruct>,
// //     number: usize,
// //     total_balance: u32,
// //     amount_in: u32,
// //     amount_out: u32,
// // }

// // impl Bank {
// //     fn new(&mut self, holder: String) -> (BankStruct) {
// //         let id = uuid::Uuid::new_v4().to_string();
// //         let next_number_holder = self.number + 1;
// //         let bank = BankStruct { id, balance: 0, account_number: next_number_holder, holder };
        
// //         // Clone the bank struct before pushing it to the vector
// //         self.back.push(bank.clone());
// //         self.number = next_number_holder;
// //         bank
// //     }
    
// //     fn print_bank(&self) {
// //         for bank in self.back.iter() {
// //             println!("id: {}, balance: {}, account_number: {}, holder: {}", bank.id, bank.balance, bank.account_number, bank.holder);
// //         }
// //     }

// //     fn find_back_by_id(&mut self, id: &str) -> &mut BankStruct {
// //         self.back.iter_mut().find(|bank| bank.id == id).unwrap()
// //     }

// //     fn get_max_balance(&self) -> BankStruct {
// //         self.back.iter().max_by_key(|bank| bank.balance).unwrap().clone()
// //     }

// //     fn get_min_balance(&self) -> BankStruct {
// //         self.back.iter().min_by_key(|bank| bank.balance).unwrap().clone()
// //     }

// //     fn get_total_balance(&self) -> u32 {
// //         self.back.iter().map(|bank| bank.balance).sum()
// //     }

// //     fn get_amount_in(&self) -> u32 {
// //         self.back.iter().map(|bank| bank.balance).sum()
// //     }

// //     fn get_amount_out(&self) -> u32 {
// //         self.back.iter().map(|bank| bank.balance).sum()
// //     }   

// //     fn deposit(&mut self, id: String, amount: u32) -> BankStruct {
// //         {
// //             let bank = self.find_back_by_id(&id);
// //             bank.balance += amount;
// //         }
// //         self.total_balance += amount;
// //         self.amount_in += amount;
// //         self.find_back_by_id(&id).clone()
// //     }

// //     fn withdraw(&mut self, id: String, amount: u32) -> &mut BankStruct {
// //         let bank = self.find_back_by_id(&id);
// //         println!("bank: {}", bank.balance);
// //         println!("amount: {}", amount);

// //         if bank.balance < amount {
// //             panic!("Insufficient balance");
// //         } else {
// //             bank.balance -= amount;
// //             self.total_balance -= amount;
// //             self.amount_out += amount;
// //         }
// //         self.find_back_by_id(&id)
// //     }
// // }

// // fn print_value(value: u32) {
// //     println!("value: {}", value);
// // }

// // fn main() {
// //     let mut new_data = Bank {
// //         back: Vec::new(),
// //         number: 0,
// //         total_balance: 0,
// //         amount_in: 0,
// //         amount_out: 0,
// //     };

// //     let user1 = new_data.new("John Doe".to_string());
// //     let user2 = new_data.new("John Doe".to_string());
// //     let user3 = new_data.new("John Doe".to_string());

// //     new_data.deposit(user1.id.clone(), 1000);

// //     new_data.withdraw(user1.id.clone(), 100);

// //     // new_data.deposit(user2.id.clone(), 2000);
// //     // new_data.print_bank();

// //     // new_data.withdraw(user2.id.clone(), 1000);
// //     new_data.print_bank();

// //     let value = 1;
// //     print_value(value);
// //     print_value(value);
// // }

// #[derive(Debug)]
// struct Account {
//     id: u32,
//     balance: i32,
//     holder: String,
// }

// impl Account {
//     fn new(id: u32, holder: String) -> Self {
//         Account {
//             id,
//             holder,
//             balance: 0,
//         }
//     }
// }

// fn print_account(account: Account) {
//     println!("Account: {:?}", account);
// }


// fn main() {
//     let account = Account::new(1, String::from("me"));
    
//     // TODO: Write and call a function that will *take ownership* of the Account
//     // value, print it, and return nothing
//     print_account(account);
//     // Question: Will you be able to call the function twice with the 'account'
//     // variable?
//     print_account(account);

// }


#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn print_bank(bank: Bank) {
    println!("{:#?}", bank);
}

fn main() {
    let bank = Bank::new();
    
    // TODO: Write and call a function that will *take ownership* of 
    // the Banks's "accounts" field, print it, and return nothing
    print_bank(bank);
    // Once you've finished the to-do, uncomment the 'print_bank' call below
    print_bank(bank);
    // When your function + print_bank run, do you think you'll end up getting
    
    // an error?
    // If so, what error do you think you'd see?
    
    // UNCOMMENT THIS:
    // print_bank(bank);
}







