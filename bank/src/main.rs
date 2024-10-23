#[derive(Debug)]
struct Account {
    balance: i32,
    id: u32,
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

fn print_account(account: Account) {
    print!("{:#?}", account);
}
fn main() {
    let bank = Bank::new();
    let account = Account::new(1, String::from("Monkey D. Luffy"));

    print_account(account);
    print_account(account);
}
