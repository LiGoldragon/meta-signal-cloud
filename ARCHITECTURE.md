# owner-signal-cloud Architecture

`owner-signal-cloud` is the owner-only Signal contract for the `cloud`
component. It controls provider account registration, credential-handle
rotation, policy changes, plan preparation, and live provider plan
application. It also accepts provider-neutral projections from `domain-criome`
and turns them into daemon-held provider plans.

## Boundary

The ordinary `signal-cloud` contract can observe and validate
provider-neutral desired state. This owner contract prepares, authorizes, and
applies plans because prepared plans are daemon-owned mutation intent and live
provider mutation changes external accounts, paid resources, and public domain
identity.

## Public Operations

- `RegisterAccount(Registration)` binds a provider account to a credential
  handle.
- `RotateCredential(Rotation)` changes the credential handle for an existing
  provider account.
- `SetPolicy(Policy)` replaces the daemon's provider-authority policy.
- `PreparePlan(PlanPreparation)` writes a provider plan into daemon plan state.
- `PrepareProjection(ProjectionPreparation)` accepts a `signal-domain-criome`
  projection and lets `cloud` lower it into a provider plan under owner
  authority.
- `ApprovePlan(Approval)` marks a prepared plan as approved for later
  application.
- `ApplyPlan(Application)` applies a prepared plan.
- `RetireAccount(Retirement)` removes an account binding.

## Ordinary vs owner split

Per Spirit records 311 and 325 (Maximum certainty, 2026-05-23), the cloud
surface splits Mutate-class verbs onto this owner contract (privileged) and
Query-class verbs onto `signal-cloud` (public). `PreparePlan` lives here
because it mutates daemon-internal plan store state, even though it does not
mutate external provider state directly. Cloudflare and other provider states
are treated as external state the cloud daemon reflects.

This is a workspace generalization: a component whose state surface is a
reflected external resource exposes its read surface on the ordinary contract
and its mutation surface on the owner contract.

## Owns

- Secret-handle references, not secret bytes.
- Provider account policy.
- Zone allowlists.
- Capability directives.
- Owner-only plan approval and application records.

## Does Not Own

- Ordinary provider observation.
- Provider-neutral desired state.
- The runtime daemon's actor tree or database.
- The Criome domain registry.

## Constraints

- Depend on `signal-frame`, not deprecated `signal-core`.
- Reuse public provider/domain/plan types from `signal-cloud`.
- Reuse provider-neutral projection types from `signal-domain-criome` at the
  cloud/domain handoff boundary.
- Do not expose raw provider credential bytes.
