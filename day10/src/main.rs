fn main() {
    let mut x = 10;
    let r = &mut x;
    *r += 10;
    println!("value of x after reference: {x}");
    // println!("value of r: {r}");

    /*x owns a memory location
    r temporarily gets exclusive permission to modify that location
    *r = 10 modifies the memory
    Since x reads from the same memory, its value appears “updated” */

    let a = Box::new(-1);
    let x_abs1 = i32::abs(*a);
    let x_abs2 = a.abs();
    assert_eq!(x_abs1, x_abs2);

    let v = vec![0, 1, 2];
    let n = get_first(&v);
    println!("{} {}", n, v[1]);
    println!("--------------------------------------------------------");

    borrow_checker();
    println!("--------------------------------------------------------");
}

fn get_first(vr: &Vec<i32>) -> i32 {
    vr[0]
}

fn borrow_checker() {
    let mut v = vec![1,2,3];
    let num = &mut v[2];
    let num2 = &*num;
    // *num += 1;
    // println!("Third element is {}", *num);
    
    // v.push(4);
    println!("{} {}", *num, *num2);
}

