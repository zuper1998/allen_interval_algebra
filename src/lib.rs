
//! Allen's interval algebra is a basic library for intervals and their comparison
//!
//! # Example
//!
//!
//! ``` rust
//!     use allen_interval_algebra::interval::Interval;
//!
//!     //Create 3 intervals
//!     let x = Interval::new(0,10);
//!     let y = Interval::new(10,14);
//!     let z = Interval::new(15,16);
//!
//!     //Test whether x interval meets z or y
//!     assert_eq!(x.meet(z),false);
//!     assert!(x.meet(y))
//!
//!
//! ```
//!


pub mod interval;




