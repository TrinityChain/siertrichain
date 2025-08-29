use crate::core::triangle::Triangle;
use crate::wallet::address::Address;
use rust_decimal::Decimal;
use std::collections::HashMap;

/// Represents a loan taken out against a triangular asset.
pub struct Loan {
    /// The address of the borrower.
    pub borrower: Address,
    /// The triangle used as collateral.
    pub collateral_triangle: Triangle,
    /// The amount of tokens borrowed.
    pub loan_amount: Decimal,
    /// The value of the collateral at the time of the loan.
    pub collateral_value: Decimal,
    /// The block number when the loan was taken.
    pub start_block: u64,
    /// The interest rate of the loan.
    pub interest_rate: Decimal,
}

/// Manages the lending and borrowing of assets against triangular collateral.
pub struct LendingPlatform {
    /// A map from a borrower's address to a list of their loans.
    loans: HashMap<Address, Vec<Loan>>,
}

impl LendingPlatform {
    pub fn new() -> Self {
        Self { loans: HashMap::new() }
    }

    /// Creates a new loan, using a triangle as collateral.
    pub fn create_loan(&mut self, borrower: Address, collateral_triangle: Triangle, loan_amount: Decimal, collateral_value: Decimal, start_block: u64, interest_rate: Decimal) {
        let loan = Loan {
            borrower: borrower.clone(),
            collateral_triangle,
            loan_amount,
            collateral_value,
            start_block,
            interest_rate,
        };
        self.loans.entry(borrower).or_default().push(loan);
    }

    /// Repays a loan.
    pub fn repay_loan(&mut self, borrower: &Address, loan_index: usize, amount: Decimal) {
        if let Some(loans) = self.loans.get_mut(borrower) {
            if let Some(loan) = loans.get_mut(loan_index) {
                loan.loan_amount -= amount;
                // In a real implementation, we would handle full repayment and return of collateral.
            }
        }
    }

    /// Liquidates a loan if the collateral value falls below a certain threshold.
    pub fn liquidate_loan(&mut self, borrower: &Address, loan_index: usize) {
        // In a real implementation, we would check the current value of the collateral
        // and if it's below a certain loan-to-value ratio, we would liquidate the position.
        if let Some(loans) = self.loans.get_mut(borrower) {
            if loan_index < loans.len() {
                loans.remove(loan_index);
                // The collateral would be sold to cover the loan.
            }
        }
    }
}
