use day21::banking::{
    accounts::open_account,
    transactions::{deposit, transfer, withdraw},
};

fn main() {
    let mut skarekroe = open_account(367423);
    let mut sanjana = open_account(394852);

    dbg!(&skarekroe);
    dbg!(&sanjana);
    println!("---------------------------------------------");

    deposit(&mut skarekroe, 63234.32);
    withdraw(&mut skarekroe, 2342.54);

    deposit(&mut sanjana, 234.32);
    withdraw(&mut sanjana, 42.54);

    transfer(&mut skarekroe, &mut sanjana, 345.01);
    println!("---------------------------------------------");

    dbg!(&skarekroe);
    dbg!(&sanjana);
}
