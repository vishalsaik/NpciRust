# Loan Approval System

## Overview
The Loan Approval System is a Rust-based application designed to evaluate loan applications based on user inputs such as income, age, and loan amount. The system incorporates optional co-applicant checks and handles various eligibility errors using custom error types.

## Features
- Input handling for income, age, and loan amount.
- Optional co-applicant support.
- Custom error handling for age and income eligibility.
- Clear error propagation and handling using Rust's `Result` and `Option` types.

## Getting Started

### Prerequisites
- Rust programming language installed on your machine. You can download it from [rust-lang.org](https://www.rust-lang.org/).

### Running the Application
1. Clone the repository:
   ```
   git clone <repository-url>
   cd loan-approval-system
   ```

2. Build the project:
   ```
   cargo build
   ```

3. Run the application:
   ```
   cargo run
   ```

### Input Format
When prompted, enter the following details:
- **Income**: Your annual income.
- **Age**: Your age.
- **Loan Amount**: The amount of loan you wish to apply for.
- **Co-applicant**: (Optional) Provide details if you have a co-applicant.

### Error Handling
The application will provide feedback on eligibility based on the following criteria:
- Age must be above a certain threshold.
- Income must meet the minimum requirement for the requested loan amount.

## Contributing
Contributions are welcome! Please feel free to submit a pull request or open an issue for any enhancements or bug fixes.

## License
This project is licensed under the MIT License. See the LICENSE file for more details.