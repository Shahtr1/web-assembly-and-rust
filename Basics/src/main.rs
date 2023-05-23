fn add(num_one: i32, num_two: i32) -> i32 {
    num_one + num_two
}

struct BankAccount {
    balance: i32,
    verified: bool
}

fn print_balance(account: &BankAccount){
    println!("{:?}", account.balance);
}

fn print_verified(account: &BankAccount){
    println!("{:?}", account.verified);
}

fn is_verified(account: &BankAccount) -> Result<bool, bool> {
    match account.verified {
        true => Ok(true),
        false => Err(false)
    }
}

fn main() {
    let mut total = add(20, 5);
    let mut free_shipping = false;

    if total > 50 {
        println!("You qualify for free shipping!");
        free_shipping = true
    }else if total > 20 {
        println!("If you add more items, you can qualify for free shipping")
    }else { println!("No free shipping") }

    total = match free_shipping {
        true => total + 0,
        false => total + 5
    };

    match total {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("No match found")
    }

    // println!("{}", foo);
    // println!("{0} {0}", foo);

    // we can manipulate a value before it gets outputted
    println!("Total: {:?}", total);

    let items:[i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", items);

    let vector_items = vec![1, 2, 3, 4, 5];
    let mut vector_items_2 = Vec::new();
    vector_items_2.push(1);
    vector_items_2.push(2);
    vector_items_2.push(3);
    vector_items_2.push(4);
    vector_items_2.push(5);

    println!("{:?}", vector_items);
    println!("{:?}", vector_items_2);


    let my_account = BankAccount{
        balance: 20,
        verified: false,
    };

    println!("{:?}", my_account.balance);
    println!("{:?}", my_account.verified);

    // Ownership gone wrong

    // during this process rust gives ownership to the print_balance function
    // so my_account variable in the main function has lost ownership
    print_balance(&my_account);
    // after the above function has been finished running rust will release memory allocated for the variable

    print_verified(&my_account); // not been able to get the value as its been dropped,
    // we can borrow the variable, use & symbol for borrowing variables


    let verification_status = is_verified(&my_account).expect("Unable to unwrap result");

    println!("{:?}", verification_status);
}
