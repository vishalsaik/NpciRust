// This file contains the main logic for the loan approval system.

use crate::errors::{AgeError, IncomeError};
use std::num::ParseFloatError;

pub struct LoanApplication {
    pub income: f64,
    pub age: u32,
    pub loan_amount: f64,
    pub co_applicant_income: Option<f64>,
}

impl LoanApplication {
    pub fn new(income: f64, age: u32, loan_amount: f64, co_applicant_income: Option<f64>) -> Self {
        Self {
            income,
            age,
            loan_amount,
            co_applicant_income,
        }
    }

    pub fn is_eligible(&self) -> Result<bool, Box<dyn std::error::Error>> {
        self.check_age()?;
        self.check_income()?;
        Ok(true)
    }

    fn check_age(&self) -> Result<(), AgeError> {
        if self.age < 18 {
            Err(AgeError::Underage)
        } else {
            Ok(())
        }
    }

    fn check_income(&self) -> Result<(), IncomeError> {
        let required_income = self.loan_amount / 10.0; // Example condition
        if self.income < required_income {
            Err(IncomeError::InsufficientIncome)
        } else {
            Ok(())
        }
    }
}