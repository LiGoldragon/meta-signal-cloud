# owner-signal-cloud Architecture

`owner-signal-cloud` is the owner-only Signal contract for the `cloud`
component. It controls provider account registration, credential-handle
rotation, policy changes, and live provider plan application.

## Boundary

The ordinary `signal-cloud` contract can observe, validate, and prepare
plans. This owner contract authorizes and applies those plans because live
provider mutation changes external accounts, paid resources, and public
domain identity.

## Public Operations

- `RegisterAccount(Registration)` binds a provider account to a credential
  handle.
- `RotateCredential(Rotation)` changes the credential handle for an existing
  provider account.
- `SetPolicy(Policy)` replaces the daemon's provider-authority policy.
- `ApprovePlan(Approval)` marks a prepared plan as approved for later
  application.
- `ApplyPlan(Application)` applies a prepared plan.
- `RetireAccount(Retirement)` removes an account binding.

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
- Do not expose raw provider credential bytes.
