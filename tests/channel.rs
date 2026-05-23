use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use owner_signal_cloud::{
    AccountRegistered, Application, Approval, CapabilityDirective, CapabilityPolicy,
    CredentialHandle, Operation, OperationKind, PlanApplied, PlanIdentifier, Policy, Provider,
    ProviderAccount, Registration, RejectionReason, Reply, ReplyKind, RequestRejected, ZonePolicy,
};
use signal_cloud::{Capability, DomainName};
use signal_frame::{RequestPayload, SignalOperationHeads};

fn encode_to_text<T: NotaEncode>(value: &T) -> String {
    let mut encoder = Encoder::new();
    value.encode(&mut encoder).expect("encode");
    encoder.into_string()
}

#[test]
fn operations_are_owner_authority_verbs() {
    assert_eq!(
        <Operation as SignalOperationHeads>::HEADS,
        &[
            "RegisterAccount",
            "RotateCredential",
            "SetPolicy",
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
        "(RegisterAccount (Cloudflare primary cloudflare-dns-token))"
    );

    let mut decoder = Decoder::new(&text);
    let decoded = Operation::decode(&mut decoder).expect("decode");
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
fn reply_round_trips_through_nota() {
    let reply = Reply::AccountRegistered(AccountRegistered {
        provider: Provider::Cloudflare,
        account: ProviderAccount::new("primary"),
    });

    assert_eq!(reply.kind(), ReplyKind::AccountRegistered);

    let text = encode_to_text(&reply);
    let mut decoder = Decoder::new(&text);
    let decoded = Reply::decode(&mut decoder).expect("decode");
    assert_eq!(decoded, reply);
}

#[test]
fn application_and_rejection_replies_are_typed() {
    let applied = Reply::PlanApplied(PlanApplied {
        plan: PlanIdentifier::new("plan-one"),
    });
    assert_eq!(applied.kind(), ReplyKind::PlanApplied);

    let rejected = Reply::RequestRejected(RequestRejected {
        operation: OperationKind::ApplyPlan,
        reason: RejectionReason::PlanNotApproved,
    });
    assert_eq!(rejected.kind(), ReplyKind::RequestRejected);
}

#[test]
fn approve_and_apply_are_distinct_owner_operations() {
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
