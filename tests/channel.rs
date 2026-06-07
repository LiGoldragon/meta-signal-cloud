use meta_signal_cloud::{
    AccountRegistered, Application, Approval, Capability, CapabilityDirective, CapabilityPolicy,
    CredentialHandle, DesiredState, DomainName, Operation, OperationKind, Plan, PlanApplied,
    PlanIdentifier, PlanPreparation, Policy, ProjectionPreparation, Provider, ProviderAccount,
    Registration, RejectionReason, Reply, ReplyKind, RequestRejected, ZonePolicy,
};
use nota_next::{NotaEncode, NotaSource};
use signal_domain_criome::{Projection, ProjectionQuery, ProjectionScope};
use signal_frame::{RequestPayload, SignalOperationHeads};

fn encode_to_text<T: NotaEncode>(value: &T) -> String {
    value.to_nota()
}

fn desired_state() -> DesiredState {
    DesiredState {
        provider: Provider::Cloudflare,
        zone: DomainName::new("goldragon.criome"),
        records: vec![],
        redirects: vec![],
    }
}

#[test]
fn operations_are_meta_authority_verbs() {
    assert_eq!(
        <Operation as SignalOperationHeads>::HEADS,
        &[
            "RegisterAccount",
            "RotateCredential",
            "SetPolicy",
            "PreparePlan",
            "PrepareProjection",
            "ApprovePlan",
            "ApplyPlan",
            "RetireAccount",
        ]
    );

    let operation = Operation::RegisterAccount(Registration {
        provider: Provider::Cloudflare,
        account: ProviderAccount::new("primary"),
        credential: CredentialHandle::new("cloudflare-dns-token"),
    });
    assert_eq!(operation.operation_kind(), OperationKind::RegisterAccount);
}

#[test]
fn registration_round_trips_through_nota_without_secret_bytes() {
    let operation = Operation::RegisterAccount(Registration {
        provider: Provider::Cloudflare,
        account: ProviderAccount::new("primary"),
        credential: CredentialHandle::new("cloudflare-dns-token"),
    });

    let text = encode_to_text(&operation);
    assert_eq!(
        text,
        "(RegisterAccount (Cloudflare [primary] [cloudflare-dns-token]))"
    );

    let decoded = NotaSource::new(&text).parse::<Operation>().expect("decode");
    assert_eq!(decoded, operation);
}

#[test]
fn policy_uses_capability_directives_not_boolean_flags() {
    let operation = Operation::SetPolicy(Policy {
        zones: vec![ZonePolicy {
            provider: Provider::Cloudflare,
            account: ProviderAccount::new("primary"),
            allowed_zones: vec![DomainName::new("goldragon.criome")],
        }],
        capabilities: vec![CapabilityPolicy {
            provider: Provider::Cloudflare,
            account: ProviderAccount::new("primary"),
            capability: Capability::RedirectRules,
            directive: CapabilityDirective::Enable,
        }],
    });

    assert_eq!(operation.operation_kind(), OperationKind::SetPolicy);
    let request = operation.into_request();
    assert_eq!(request.payloads().len(), 1);
}

#[test]
fn plan_preparation_round_trips_through_meta_contract() {
    let operation = Operation::PreparePlan(PlanPreparation {
        desired_state: desired_state(),
    });

    assert_eq!(operation.operation_kind(), OperationKind::PreparePlan);

    let text = encode_to_text(&operation);
    let decoded = NotaSource::new(&text).parse::<Operation>().expect("decode");
    assert_eq!(decoded, operation);
}

#[test]
fn projection_preparation_uses_domain_projection_contract() {
    let operation = Operation::PrepareProjection(ProjectionPreparation {
        provider: Provider::Cloudflare,
        projection: Projection {
            query: ProjectionQuery {
                domain: signal_domain_criome::DomainName::new("goldragon.criome"),
                scope: ProjectionScope::PublicRecords,
            },
            records: vec![],
            redirects: vec![],
        },
    });

    assert_eq!(operation.operation_kind(), OperationKind::PrepareProjection);
}

#[test]
fn reply_round_trips_through_nota() {
    let reply = Reply::AccountRegistered(AccountRegistered {
        provider: Provider::Cloudflare,
        account: ProviderAccount::new("primary"),
    });

    assert_eq!(reply.kind(), ReplyKind::AccountRegistered);

    let text = encode_to_text(&reply);
    let decoded = NotaSource::new(&text).parse::<Reply>().expect("decode");
    assert_eq!(decoded, reply);
}

#[test]
fn plan_prepared_reply_carries_public_plan_record() {
    let reply = Reply::PlanPrepared(Plan {
        identifier: PlanIdentifier::new("plan-one"),
        provider: Provider::Cloudflare,
        zone: DomainName::new("goldragon.criome"),
        records_to_create: vec![],
        records_to_update: vec![],
        record_names_to_delete: vec![],
        redirects_to_create: vec![],
        redirects_to_update: vec![],
        redirect_sources_to_delete: vec![],
    });

    assert_eq!(reply.kind(), ReplyKind::PlanPrepared);

    let text = encode_to_text(&reply);
    let decoded = NotaSource::new(&text).parse::<Reply>().expect("decode");
    assert_eq!(decoded, reply);
}

#[test]
fn application_and_rejection_replies_are_typed() {
    let applied = Reply::PlanApplied(PlanApplied {
        plan: PlanIdentifier::new("plan-one"),
    });
    assert_eq!(applied.kind(), ReplyKind::PlanApplied);

    let rejected = Reply::RequestRejected(RequestRejected {
        reason: RejectionReason::PlanNotApproved,
    });
    assert_eq!(rejected.kind(), ReplyKind::RequestRejected);

    let unconfigured = Reply::RequestRejected(RequestRejected {
        reason: RejectionReason::ProviderNotConfigured,
    });
    assert_eq!(unconfigured.kind(), ReplyKind::RequestRejected);
}

#[test]
fn approve_and_apply_are_distinct_meta_operations() {
    let approve = Operation::ApprovePlan(Approval {
        plan: PlanIdentifier::new("plan-one"),
    });
    let apply = Operation::ApplyPlan(Application {
        plan: PlanIdentifier::new("plan-one"),
    });

    assert_eq!(approve.kind(), OperationKind::ApprovePlan);
    assert_eq!(apply.kind(), OperationKind::ApplyPlan);
}

#[test]
fn contract_does_not_depend_on_deprecated_signal_core() {
    let manifest = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .expect("manifest");
    assert!(!manifest.contains("signal-core"));
}
