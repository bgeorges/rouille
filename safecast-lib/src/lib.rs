pub mod models;

#[cfg(test)]
mod test {
    use crate::models::Measurement;
    use crate::models::User;
    use std::fs;

    #[test]
    fn user_test() {
        let joe = User {
            id: 1,
            name: String::from("Joe"),
            measurements_count: None,
        };
        let bob = User {
            id: 2,
            name: String::from("Bob"),
            measurements_count: None,
        };
        assert!(joe == bob);
    }

    #[test]
    fn measurement_test() {
        let m1 = Measurement::new(String::from("Unite"), 3);
        let m2 = Measurement::new(String::from("Unite"), 3);
        assert!(m1.value == m2.value);
    }

    #[test]
    fn read_logs() {
        let logs = fs::read_to_string("data/sample-tracking-data.log").expect("Error reading file");
        assert!(logs.to_string().starts_with("Radius"));
    }
}
