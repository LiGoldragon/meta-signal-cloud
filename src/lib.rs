//! Meta Signal contract for the cloud component.
//!
//! This crate carries meta policy provider account, credential-handle, policy,
//! plan preparation, approval, and application records. It never carries secret
//! bytes.

use nota_next::{NotaDecode, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;

pub mod schema;

pub use signal_cloud::{
    Capability, DesiredState, DomainName, Plan, PlanIdentifier, Provider, ProviderAccount,
};
pub use signal_domain_criome::Projection;

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct CredentialHandle(String);

impl CredentialHandle {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Registration {
    pub provider: Provider,
    pub account: ProviderAccount,
    pub credential: CredentialHandle,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Rotation {
    pub provider: Provider,
    pub account: ProviderAccount,
    pub credential: CredentialHandle,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum CapabilityDirective {
    Enable,
    Disable,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct CapabilityPolicy {
    pub provider: Provider,
    pub account: ProviderAccount,
    pub capability: Capability,
    pub directive: CapabilityDirective,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ZonePolicy {
    pub provider: Provider,
    pub account: ProviderAccount,
    pub allowed_zones: Vec<DomainName>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Policy {
    pub zones: Vec<ZonePolicy>,
    pub capabilities: Vec<CapabilityPolicy>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct PlanPreparation {
    pub desired_state: DesiredState,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct ServerType(String);

impl ServerType {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct ImageName(String);

impl ImageName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct SshKeyName(String);

impl SshKeyName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum HostIntent {
    Create,
    Destroy,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DesiredHostState {
    pub provider: Provider,
    pub host_name: DomainName,
    pub server_type: ServerType,
    pub image_name: ImageName,
    pub ssh_key_name: SshKeyName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct HostPlanPreparation {
    pub desired_host_state: DesiredHostState,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct HostPlan {
    pub identifier: PlanIdentifier,
    pub provider: Provider,
    pub host_name: DomainName,
    pub server_type: ServerType,
    pub image_name: ImageName,
    pub ssh_key_name: SshKeyName,
    pub intent: HostIntent,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ProjectionPreparation {
    pub provider: Provider,
    pub projection: Projection,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Approval {
    pub plan: PlanIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Application {
    pub plan: PlanIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Retirement {
    pub provider: Provider,
    pub account: ProviderAccount,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct AccountRegistered {
    pub provider: Provider,
    pub account: ProviderAccount,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct CredentialRotated {
    pub provider: Provider,
    pub account: ProviderAccount,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct PolicySet {
    pub capability_policy_count: u64,
    pub zone_policy_count: u64,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct PlanApproved {
    pub plan: PlanIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct PlanApplied {
    pub plan: PlanIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct AccountRetired {
    pub provider: Provider,
    pub account: ProviderAccount,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum RejectionReason {
    CredentialHandleUnknown,
    ProviderNotConfigured,
    AccountUnknown,
    PlanUnknown,
    PlanNotApproved,
    PlanGenerationFailed,
    CapabilityUnauthorized,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RequestRejected {
    pub reason: RejectionReason,
}

signal_channel! {
    channel Meta {
        operation RegisterAccount(Registration),
        operation RotateCredential(Rotation),
        operation SetPolicy(Policy),
        operation PreparePlan(PlanPreparation),
        operation PrepareHostPlan(HostPlanPreparation),
        operation PrepareProjection(ProjectionPreparation),
        operation ApprovePlan(Approval),
        operation ApplyPlan(Application),
        operation RetireAccount(Retirement),
    }
    reply Reply {
        AccountRegistered(AccountRegistered),
        CredentialRotated(CredentialRotated),
        PolicySet(PolicySet),
        PlanPrepared(Plan),
        HostPlanPrepared(HostPlan),
        PlanApproved(PlanApproved),
        PlanApplied(PlanApplied),
        AccountRetired(AccountRetired),
        RequestRejected(RequestRejected),
    }
}

pub type ChannelRequest = signal_frame::Request<Operation>;
pub type ChannelReply = signal_frame::Reply<Reply>;

impl Operation {
    pub fn operation_kind(&self) -> OperationKind {
        self.kind()
    }
}
