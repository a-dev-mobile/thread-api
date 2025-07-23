#[cfg(test)]
mod run {
    use crate::features::imperial::v1::info::handler::unified_thread_allowance;

    #[test]
    fn test_unified_thread_allowance_1() {
        let d = 1.;
        let tpi = 20.0;
        let p = 1.0 / tpi;
        let le = 9.0 / tpi;
        let expected = 0.001363;
        let calculated = unified_thread_allowance(d, p, le, "2A").unwrap();

        println!("\nResults Comparison:");
        println!("Expected  = {}", expected);
        println!("Calculated  = {}", calculated);
        println!("Difference = {}", (calculated - expected).abs());
        assert!(
            (calculated - expected).abs() < 1e-6,
            "Expected {}, but got {}",
            expected,
            calculated
        );
    }

    #[test]
    fn test_unified_thread_allowance_2() {
        let d = 6.;
        let tpi = 14.0;
        let p = 1.0 / tpi;
        let le = 9.0 / tpi;
        let expected = 0.001953;
        let calculated = unified_thread_allowance(d, p, le, "2A").unwrap();

        println!("\nResults Comparison:");
        println!("Expected  = {}", expected);
        println!("Calculated  = {}", calculated);
        println!("Difference = {}", (calculated - expected).abs());
        assert!(
            (calculated - expected).abs() < 1e-6,
            "Expected {}, but got {}",
            expected,
            calculated
        );
    }

    #[test]
    fn test_unified_thread_allowance_3() {
        let d = 2.;
        let tpi = 14.0;
        let p = 1.0 / tpi;
        let le = 9.0 / tpi;
        let expected = 0.001702;
        let calculated = unified_thread_allowance(d, p, le, "2A").unwrap();

        println!("\nResults Comparison:");
        println!("Expected  = {}", expected);
        println!("Calculated  = {}", calculated);
        println!("Difference = {}", (calculated - expected).abs());
        assert!(
            (calculated - expected).abs() < 1e-6,
            "Expected {}, but got {}",
            expected,
            calculated
        );
    }

    #[test]
    fn test_unified_thread_allowance_4() {
        let d = 1.;
        let tpi = 8.0;
        let p = 1.0 / tpi;
        let le = 9.0 / tpi;
        let expected = 0.002052;
        let calculated = unified_thread_allowance(d, p, le, "2A").unwrap();

        println!("\nResults Comparison:");
        println!("Expected  = {}", expected);
        println!("Calculated  = {}", calculated);
        println!("Difference = {}", (calculated - expected).abs());
        assert!(
            (calculated - expected).abs() < 1e-6,
            "Expected {}, but got {}",
            expected,
            calculated
        );
    }

    #[test]
    fn test_unified_thread_allowance_invalid_class() {
        let d = 1.;
        let tpi = 8.0;
        let p = 1.0 / tpi;
        let le = 9.0 / tpi;
        let result = unified_thread_allowance(d, p, le, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_unified_thread_allowance_class_3a() {
        let d = 1.;
        let tpi = 8.0;
        let p = 1.0 / tpi;
        let le = 9.0 / tpi;
        let calculated = unified_thread_allowance(d, p, le, "3A").unwrap();
        assert_eq!(calculated, 0.0);
    }
}
