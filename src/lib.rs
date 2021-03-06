//! # Gooseberry - A Knowledge Base for the Lazy

/// Configuration of data directories and Hypothesis authorization
pub mod configuration;
/// "It claimed to have 15 functions, although it appeared that at least ten were apologizing for
/// the useless manner in which it performed the others." - [Dis-organizer](https://wiki.lspace.org/mediawiki/Dis-organiser)
pub mod errors;
/// Main gooseberry logic
pub mod gooseberry;
/// Utility functions
pub mod utils;

/// Name of the app, used for making project directories etc.
pub const NAME: &str = "gooseberry";
/// Minimum sync date, gooseberry starts sync by looking for all annotations created / updated after this date.
pub const MIN_DATE: &str = "1900-01-01T00:00:00.000Z";
/// Tag used to store untagged Hypothesis annotations
/// This shows up only in gooseberry and not in Hypothesis
pub const EMPTY_TAG: &str = "Untagged";
/// Tag used to tell gooseberry to ignore an annotation
/// This shows up only in Hypothesis and not in gooseberry
pub const IGNORE_TAG: &str = "gooseberry_ignore";
