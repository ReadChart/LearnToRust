//! expansive calculations
//!
//! Using closures to create abstract of behavior
//! This function is using [std::thread::sleep()] to sleep the thread in order to simulate a expansive calculation
//! ```rust
//! fn simulated_expansive_calculation(intensity: u32) ->u32{
//!     println!("Calculating slowly");
//!     thread::sleep(Duration::from_secs(2));
//!     intensity
//!}
//! ```
//! Using assigned variables as user inputs
//! ```rust
//! fn main() {
//!     let simulated_user_specified_value = 10;
//!     let simulated_random_number = 7;
//!
//!     generate_workout(
//!         simulated_user_specified_value,
//!         simulated_random_number
//!     );
//! }
//! ```
//! function generate workout details
//! ```rust
//! fn generate_workout(intensity: u32, random_number: u32) {
//!     if intensity < 25 {
//!         println!(
//!             "Today, do {} pushups!",
//!             simulated_expensive_calculation(intensity)
//!         );
//!         println!(
//!             "Next, do {} situps!",
//!             simulated_expensive_calculation(intensity)
//!         );
//!     } else {
//!         if random_number == 3 {
//!             println!("Take a break today! Remember to stay hydrated!");
//!         } else {
//!             println!(
//!                 "Today, run for {} minutes!",
//!                simulated_expensive_calculation(intensity)
//!             );
//!         }
//!     }
//! }
//! ```
//! Using functions to reconstruct generate workout detail :
//! ```rust
//!
//! #![allow(unused)]
//! fn main() {
//!     use std::thread;
//!     use std::time::Duration;
//!
//!     fn simulated_expensive_calculation(num: u32) -> u32 {
//!         println!("calculating slowly...");
//!         thread::sleep(Duration::from_secs(2));
//!         num
//!     }
//!
//!     fn generate_workout(intensity: u32, random_number: u32) {
//!         let expensive_result =
//!             simulated_expensive_calculation(intensity);
//!
//!         if intensity < 25 {
//!             println!(
//!                 "Today, do {} pushups!",
//!                 expensive_result
//!             );
//!             println!(
//!                 "Next, do {} situps!",
//!                 expensive_result
//!             );
//!         } else {
//!             if random_number == 3 {
//!                 println!("Take a break today! Remember to stay hydrated!");
//!             } else {
//!                 println!(
//!                     "Today, run for {} minutes!",
//!                     expensive_result
//!                 );
//!             }
//!         }
//!     }
//! }
//! ```
//! Follow up is the example using closure to reconstruct
//!

pub fn add_using_closure() {
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| x + 1;
    let add_one_v4 = |x: u32| x + 1;
}

struct Catchier<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Catchier<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Catchier<T> {
        Catchier {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
