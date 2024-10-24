// lib.rs

pub fn rent_calculator(
    months: &u32,
    monthly_rent: &f64,
    free_months_rent: &u32,
    monthly_utilities: &f64,
    free_months_utilities: &u32,
) -> (f64, f64) {
    let total_rent = (*months - *free_months_rent) as f64 * *monthly_rent;
    let total_utilities = (*months - *free_months_utilities) as f64 * *monthly_utilities;
    let total_spent = total_rent + total_utilities;
    let net_monthly_expense = total_spent / *months as f64;

    (total_spent, net_monthly_expense)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rent_calculator() {
        let months = 12;
        let monthly_rent = 1000.0;
        let free_months_rent = 2;
        let monthly_utilities = 150.0;
        let free_months_utilities = 1;

        let (total_spent, net_monthly_expense) = rent_calculator(
            &months,
            &monthly_rent,
            &free_months_rent,
            &monthly_utilities,
            &free_months_utilities,
        );

        // Allow small differences in floating-point values (within 0.01)
        let tolerance = 0.01;

        assert!((total_spent - 11650.0).abs() < tolerance);
        assert!((net_monthly_expense - 970.83).abs() < tolerance);
    }
}
