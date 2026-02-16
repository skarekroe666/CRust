pub mod banking {
    pub mod accounts {
        #[derive(Debug)]
        pub struct Account {
            pub id: u32,
            pub name: String,
            pub balance: f64,
        }

        pub fn open_account(id: u32, name: String) -> Account {
            println!("Accout opened: {id}");
            Account {
                name,
                id,
                balance: 0.0,
            }
        }
    }

    pub mod transactions {
        use super::accounts::Account;

        pub fn deposit(acc: &mut Account, amount: f64) {
            acc.balance += amount;
            println!(
                "[TRANSACTION] Deposited: ${} into {}. New balance: ${}",
                amount, acc.id, acc.balance
            )
        }

        pub fn withdraw(acc: &mut Account, amount: f64) {
            if acc.balance >= amount {
                println!(
                    "[TRANSACTION] Withdrew ${} from account: {}, New balace: {}",
                    amount, acc.id, acc.balance
                );
            } else {
                println!("[TRANSACTION] ERROR: Insufficient balance");
            }
        }
    }
}
