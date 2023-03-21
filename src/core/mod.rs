//! Core module for the library, contains data types for the API responses
//!
//! Some of the most important types are [`api::events::Event`] and MediaPackage.
//!
//! The submodules are organized by the API path they are used for.  
//! The [`api`] module contains submodules itself, just like the server-side structure.
//!
//! All of the submodules contain the data types for the API responses; having one named
//! _api_ was a choice made by the Opencast developers (in their REST API structure),
//! which is modeled in this library using modules.

pub mod api;
pub mod info;
pub mod search;
pub mod series;
