use day26::banking::{accounts, transactions};

fn main() {
    let mut acc1 = accounts::open_account(2876437, String::from("skarekroe"));
    dbg!(&acc1);
    let mut acc2 = accounts::open_account(2936425, String::from("sanjana"));
    dbg!(&acc2);

    transactions::deposit(&mut acc1, 45983.74);
    transactions::withdraw(&mut acc1, 423.53);
    dbg!(&acc1);

    println!();
    transactions::transfer(&mut acc1, &mut acc2, 876.32);
}
