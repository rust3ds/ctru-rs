//! Power-Time service.
//!
//! This service manages user information such as registered playtime, step count (using the pedometer) and control to various
//! hardware and features to notify the user during play (such as the Notification/Info LED).
#![doc(alias = "led")]
#![doc(alias = "playtime")]
#![doc(alias = "step")]
#![doc(alias = "power")]

pub mod user;
