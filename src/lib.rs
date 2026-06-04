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
//! `Asschema` and emitted into Rust by `schema-rust-next`. The presence of
//! `Input` + `Output` roots alongside `NexusWork` / `NexusAction` /
//! `SemaWriteInput` / `SemaReadInput` in the namespace is what triggers the
//! emitter to write the three engine traits (`SignalEngine`, `NexusEngine`,
//! `SemaEngine`). `build.rs` verifies the checked-in module is fresh by
//! round-tripping the schema source, comparing the assembled NOTA + rkyv
//! artifacts, and re-emitting Rust from both.
//!
//! The hand-written daemon — the `SignalActor` / `Nexus` / `Store` impls plus
//! the Cloudflare `CommandEffect` handler — lives in the `cloud` runtime
//! repository, not here. This crate is the contract leg of the triad.

#![forbid(unsafe_code)]

pub mod schema {
    #[rustfmt::skip]
    pub mod meta_signal_cloud;
}

pub use schema::meta_signal_cloud::*;
