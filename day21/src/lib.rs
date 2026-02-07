pub mod banking {
    pub mod accounts {
        #[derive(Debug)]
        pub struct Account {
            pub acc_number: i32,
            pub balance: f64,
        }

        pub fn open_account(id: i32) -> Account {
            println!("Account {} opened!", id);
            Account {
                acc_number: id,
                balance: 0.0,
            }
        }

        #[allow(dead_code)]
        fn close_account(acc: &mut Account) {
            println!("Account {} closed", acc.acc_number);
            acc.balance = 0.0;
        }
    }

    pub mod transactions {
        //super allows us to use functionality outside the module
        use super::accounts::Account;

        pub fn deposit(acc: &mut Account, amount: f64) {
            acc.balance += amount;
            println!(
                "Deposited ${:.2}, into Account {}. New balance: {}",
                amount, acc.acc_number, acc.balance
            );
        }

        pub fn withdraw(acc: &mut Account, amount: f64) {
            if acc.balance >= amount {
                acc.balance -= amount;
                println!(
                    "Withdrew {} from Account {}. New balance: {:.2}",
                    amount, acc.acc_number, acc.balance
                );
            }
        }

        pub fn transfer(from: &mut Account, to: &mut Account, amount: f64) {
            if from.balance >= amount {
                from.balance -= amount;
                to.balance += amount;
                println!(
                    "Transferred ${:.2} from Account {} to Account {}",
                    amount, from.acc_number, to.acc_number
                );
            } else {
                println!("Insufficient funds");
            }
        }
    }
}
