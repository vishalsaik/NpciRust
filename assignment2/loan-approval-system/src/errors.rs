// This file defines custom error types for the loan approval system.

#[derive(Debug)]
pub enum AgeError {
    Underage,
    Overage,
}

#[derive(Debug)]
pub enum IncomeError {
    InsufficientIncome,
}

impl std::fmt::Display for AgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgeError::Underage => write!(f, "Applicant is under the minimum age requirement."),
            AgeError::Overage => write!(f, "Applicant exceeds the maximum age limit."),
        }
    }
}

impl std::fmt::Display for IncomeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IncomeError::InsufficientIncome => write!(f, "Applicant's income is insufficient for the loan amount."),
        }
    }
}

impl std::error::Error for AgeError {}

impl std::error::Error for IncomeError {}