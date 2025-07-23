#[cfg(test)]
mod run {
    use crate::features::imperial::v1::info::handler::calculate_td2;

    #[test]
    fn test_calculate_td2_class_1_a() {
        // Test data for 2.0625-12-UNS-2A
        let d = 2.0625;
        let tpi = 12.0;
        let le = 9.0 / tpi;
        let class = "2A";
        let p = 1.0 / tpi;

        println!("\nTest Input Parameters:");
        println!("Basic major diameter (d) = {}", d);
        println!("Threads per inch (tpi) = {}", tpi);
        println!("Length of engagement (le) = {}", le);
        println!("Pitch (p) = {}", p);
        println!("Class = {}", class);

        let expected = 0.006070;
        let calculated = calculate_td2(d, le, p, class);

        println!("\nResults Comparison:");
        println!("Expected TD2 = {}", expected);
        println!("Calculated TD2 = {}", calculated);
        println!("Difference = {}", (calculated - expected).abs());

        assert!(
            (calculated - expected).abs() < 1e-6,
            "Expected {}, but got {}",
            expected,
            calculated
        );
    }

    #[test]
    fn test_calculate_td2_class_2_a() {
        // Test data for 0.5-28-UNEF-2A
        let d = 0.5; // Basic major diameter
        let tpi = 28.0; // Threads per inch
        let le = 9.0 / tpi; // Length of engagement
        let class = "2A"; // Thread class
        let p = 1.0 / tpi; // Pitch

        println!("\nTest Input Parameters:");
        println!("Basic major diameter (d) = {}", d);
        println!("Threads per inch (tpi) = {}", tpi);
        println!("Length of engagement (le) = {}", le);
        println!("Pitch (p) = {}", p);
        println!("Class = {}", class);

        let expected = 0.003668;
        let calculated = calculate_td2(d, le, p, class);

        println!("\nResults Comparison:");
        println!("Expected TD2 = {}", expected);
        println!("Calculated TD2 = {}", calculated);
        println!("Difference = {}", (calculated - expected).abs());

        assert!(
            (calculated - expected).abs() < 1e-6,
            "Expected {}, but got {}",
            expected,
            calculated
        );
    }

    #[test]
    fn test_calculate_td2_class_2_b() {
        // Test data for 0.5-28-UNEF-2B
        let d = 0.5; // Basic major diameter
        let tpi = 28.0; // Threads per inch
        let le = 9.0 / tpi; // Length of engagement
        let class = "2B"; // Thread class
        let p = 1.0 / tpi; // Pitch

        println!("\nTest Input Parameters:");
        println!("Basic major diameter (d) = {}", d);
        println!("Threads per inch (tpi) = {}", tpi);
        println!("Length of engagement (le) = {}", le);
        println!("Pitch (p) = {}", p);
        println!("Class = {}", class);
        // https://fpg-co.com/Standards/ASME%20B1.1%202008.pdf 91
        let expected = 0.004768;
        let calculated = calculate_td2(d, le, p, class);

        println!("\nResults Comparison:");
        println!("Expected TD2 = {}", expected);
        println!("Calculated TD2 = {}", calculated);
        println!("Difference = {}", (calculated - expected).abs());

        assert!(
            (calculated - expected).abs() < 1e-6,
            "Expected {}, but got {}",
            expected,
            calculated
        );
    }
}
