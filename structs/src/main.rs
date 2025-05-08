struct Account {
    ref_id: String,
    balance: f32,
    name: String,
    active: bool,
}
impl Account {
    fn create_account(&self) -> Account {
        Account {
            ref_id,
            balance,
            name,
            active,
        }
    }
}


fn main() {
    let _user_1: Account = create_account(String::from("#2efabc62"), 250.00, String::from("Ravi"), true);
    println!("User {} has ${} in their account", _user_1.name, _user_1.balance);
    println!("Active Status: {}", _user_1.active.to_string());
    println!("Account ID: {}", _user_1.ref_id);

    let _user_2: Account = create_account(String::from("#2efabc60"), 450.00, String::from("Raju"), true);
    println!("User {} has ${} in their account", _user_2.name, _user_2.balance);
    println!("Active Status: {}", _user_2.active.to_string());
    println!("Account ID: {}", _user_2.ref_id);
}

