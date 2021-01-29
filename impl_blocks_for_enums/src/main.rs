enum PaymentMode {
    Debit,
    Credit,
    Paypal,
}

fn pay_by_credit(amt: u64) {
    println!("processing payment by credit, {}", amt)
}
fn pay_by_debit(amt: u64) {
    println!("processing payment by debit, {}", amt)
}
fn pay_by_paypal(amt: u64) {
    println!("processing payment by paypal, {}", amt)
}

impl PaymentMode {
    fn pay(&self, amount: u64) {
        match self {
            PaymentMode::Debit => pay_by_debit(amount),
            PaymentMode::Credit => pay_by_credit(amount),
            PaymentMode::Paypal => pay_by_paypal(amount),
        }
    }
}

fn main() {
    PaymentMode::Paypal.pay(1000000);
    PaymentMode::Credit.pay(1000000);
    PaymentMode::Debit.pay(1000000);
}
