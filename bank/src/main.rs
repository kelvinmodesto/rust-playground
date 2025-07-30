#[derive(Debug)]
struct Account {
    balance: i32,
    id: u32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            balance: 0,
            id,
            holder,
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

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn print_holder(holder: String) {
    println!("{:#?}", holder);
}

fn main() {
    // let bank = Bank::new();
    let account = Account::new(1, String::from("Monkey D. Luffy"));

    // let other_bank = bank;

    // let accounts_list = vec![account];
    // let accounts = bank.accounts;

    // println!("{:#?}", bank.accounts);
    print_account(&account);

    println!("{:#?}", account);
}
