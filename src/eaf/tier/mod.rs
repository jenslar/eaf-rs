//! Tier.
//! 
//! ELAN tiers can be either a main tier,
//! or a referred tier (refers to a parent tier).
//! 
//! Note that a referred tier may refer to another
//! referred tier.

pub mod tier;
pub mod builder;

pub use tier::Tier;
pub use builder::TierBuilder;