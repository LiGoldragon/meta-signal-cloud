//! `meta-signal-cloud` — the meta (owner-only policy) Signal contract for the
//! `cloud` component, schema-derived.
//!
//! This crate carries owner-only provider account, credential-handle, policy,
//! plan preparation, approval, and application authority. It never carries
//! secret bytes — credentials cross only as durable `CredentialHandle`
//! references.
//!
//! The public wire types are checked-in generated source from
//! `schema/meta-signal-cloud.schema`, lowered through `schema-next` into an
//! `Asschema` and emitted into Rust by `schema-rust-next`. `build.rs` verifies
//! the checked-in module is fresh by round-tripping the schema source,
//! comparing the assembled NOTA + rkyv artifacts, and re-emitting Rust from
//! both.
//!
//! The hand-written daemon — Signal admission, Nexus decisions, SEMA state,
//! and Cloudflare effects — lives in the `cloud` runtime repository, not here.
//! This crate is the policy contract leg of the triad and carries only wire
//! vocabulary.

#![forbid(unsafe_code)]

pub mod schema {
    #[rustfmt::skip]
    pub mod meta_signal_cloud;
}

pub use schema::meta_signal_cloud::*;
