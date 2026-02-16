use day26::banking::{accounts, transactions};

fn main() {
    let mut acc = accounts::open_account(2876437, String::from("skarekroe"));

    dbg!(&acc);

    transactions::deposit(&mut acc, 45983.74);
    transactions::withdraw(&mut acc, 423.53);
}
