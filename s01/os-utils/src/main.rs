use chrono::{DateTime, Utc};

// Learning Rust basics
/// Print current machine system info
fn main() {
    let host = gethostname::gethostname();
    println!("Hostname: {:?}", host);
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    // formatting prints
    println!("{} days", 365i32);
    //positioned arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Bob", "Bruno");
    // named arguments.
    println!(
        "{subject} {verb} {:?}",
        host,
        subject = "hostname",
        verb = "is"
    );
    println!("{} of {:x} people know Hex, the other half doesn't", 1, 20);
    println!("My name is {0}, {1} {0}", "G", "Ali");
    #[allow(dead_code)]
    let now: DateTime<Utc> = Utc::now();
    #[derive(Debug)]
    struct Structure(String, DateTime<Utc>);
    println!(
        "This struct {:#?} prints",
        Structure("Now".to_string(), now)
    );

    // let's do  decode sigfox paylod
    use std::fmt;

    struct TrackerMessage;
    struct Coordinates {
        lat: f64,
        long: f64,
    }

    impl Coordinates {
        fn latitude(&self) -> f64 {
            self.lat
        }

        pub fn longitude(&self) -> f64 {
            self.long
        }
    }

    trait Decode {
        fn sfdec(&self) -> Coordinates;
    }

    impl fmt::Display for Coordinates {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({},{})", self.latitude(), self.longitude())
        }
    }

    assert_eq!(
        "(2.983,1.987)",
        format!(
            "{}",
            Coordinates {
                long: 1.987,
                lat: 2.983,
            }
        )
    );


}
