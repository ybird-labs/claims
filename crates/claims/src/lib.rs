//! Claims Engine core crate.
//!
//! This crate starts intentionally small. Keep the domain pure and add concrete
//! implementation only when the domain model is ready.

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod projection;
