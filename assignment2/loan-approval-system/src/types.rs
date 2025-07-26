// This file defines necessary types and structures used throughout the application.

pub struct LoanApplication {
    pub income: f64,
    pub age: u32,
    pub loan_amount: f64,
    pub co_applicant_income: Option<f64>,
}

pub struct LoanEligibility {
    pub is_eligible: bool,
    pub reason: Option<String>,
}