// main.rs

use clap::Parser;
use rent_calculator::rent_calculator;

/// CLI arguments for the rent calculator
#[derive(Parser)]
#[command(name = "Rent Calculator", about = "Calculate total and monthly rent expenses")]
struct Cli {
    /// Number of months to rent
    months: u32,
    /// Monthly rent
    monthly_rent: f64,
    /// Number of free months for rent
    free_months_rent: u32,
    /// Monthly utilities
    monthly_utilities: f64,
    /// Number of free months for utilities
    free_months_utilities: u32,
}

fn main() {
    let args = Cli::parse();

    // Call rent_calculator from lib.rs
    let (total_spent, net_monthly_expense) = rent_calculator(
        &args.months,
        &args.monthly_rent,
        &args.free_months_rent,
        &args.monthly_utilities,
        &args.free_months_utilities,
    );

    // Output the results
    println!("Total money spent: {:.2}", total_spent);
    println!("Net monthly expense: {:.2}", net_monthly_expense);
}
