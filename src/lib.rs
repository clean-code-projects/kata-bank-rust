use chrono::{Date, Utc};
use std::cell::RefCell;

pub struct DateService {
  today: RefCell<Date<Utc>>,
}

impl DateService {
  pub fn new() -> DateService {
    DateService {
      today: RefCell::new(Utc::today()),
    }
  }
  pub fn set_today(&self, today: Date<Utc>) {
    self.today.replace(today);
  }

  pub fn get(&self) -> Date<Utc> {
    self.today.borrow().clone()
  }
}

pub enum Transaction {
  Deposit(Date<Utc>, i32, i32),
  Withdraw(Date<Utc>, i32, i32),
}

pub struct AccountService<'a> {
  date_service: &'a DateService,
  balance: i32,
  transactions: Vec<Transaction>,
}

impl<'a> AccountService<'a> {
  pub fn new(date_service: &DateService) -> AccountService {
    AccountService {
      date_service,
      balance: 0,
      transactions: Vec::new(),
    }
  }

  pub fn deposit(&mut self, amount: i32) {
    self.balance += amount;
    self.transactions.push(Transaction::Deposit(
      self.date_service.get(),
      amount,
      self.balance,
    ));
  }

  pub fn withdraw(&mut self, amount: i32) {
    self.balance -= amount;
    self.transactions.push(Transaction::Withdraw(
      self.date_service.get(),
      amount,
      self.balance,
    ));
  }

  pub fn get_balance(&self) -> i32 {
    self.balance
  }

  pub fn print_statement(&self) {
    println!("{}", self.get_statement());
  }

  pub fn get_statement(&self) -> String {
    let header = String::from("Date | Amount | Balance");

    let rows: Vec<String> = self
      .transactions
      .iter()
      .rev()
      .map(format_transaction)
      .collect();

    header + "\n" + &rows.join("\n") + "\n"
  }
}

fn format_transaction(t: &Transaction) -> String {
  match t {
    Transaction::Deposit(d, a, b) => format!("{} | {} | {}", d.format("%d/%m/%Y"), a, b),
    Transaction::Withdraw(d, a, b) => format!("{} | {} | {}", d.format("%d/%m/%Y"), -a, b),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod account_service {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn new_balance_is_0() {
      let ds = DateService::new();

      let a = AccountService::new(&ds);

      assert_eq!(0, a.get_balance());
    }

    #[test]
    fn deposit_100() {
      let ds = DateService::new();
      let mut a = AccountService::new(&ds);

      a.deposit(100);

      assert_eq!(100, a.get_balance());
    }

    #[test]
    fn withdraw_100() {
      let ds = DateService::new();
      let mut a = AccountService::new(&ds);

      a.withdraw(100);

      assert_eq!(-100, a.get_balance());
    }

    #[test]
    fn print_statement_with_single_transaction() {
      let ds = DateService::new();
      let mut a = AccountService::new(&ds);

      ds.set_today(Utc.ymd(2012, 1, 10));
      a.deposit(1000);

      let statement = a.get_statement();
      assert_eq!(
        "Date | Amount | Balance\n10/01/2012 | 1000 | 1000\n",
        statement
      )
    }
    #[test]
    fn acceptance_test() {
      let ds = DateService::new();
      let mut a = AccountService::new(&ds);

      ds.set_today(Utc.ymd(2012, 1, 10));
      a.deposit(1000);

      ds.set_today(Utc.ymd(2012, 1, 13));
      a.deposit(2000);

      ds.set_today(Utc.ymd(2012, 1, 14));
      a.withdraw(500);

      let statement = a.get_statement();
      assert_eq!(
        [
          "Date | Amount | Balance",
          "14/01/2012 | -500 | 2500",
          "13/01/2012 | 2000 | 3000",
          "10/01/2012 | 1000 | 1000"
        ]
        .join("\n")
          + "\n",
        statement
      )
    }
  }

  mod date_service {
    use crate::DateService;
    use chrono::prelude::*;

    #[test]
    fn set_today() {
      let ds = DateService::new();
      ds.set_today(Utc.ymd(2015, 5, 15));

      assert_eq!("2015-05-15UTC", ds.get().to_string());
    }
  }
}
