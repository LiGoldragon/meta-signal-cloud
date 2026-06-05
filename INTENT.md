# INTENT — meta-signal-cloud

*The meta (owner-only policy) wire contract for the `cloud` component. Defines the
typed request/reply channel that the cloud owner uses to register provider
accounts, rotate credential handles, set authority policy, prepare and approve
provider plans, and apply live provider mutations.
Companion to `ARCHITECTURE.md` and `Cargo.toml`. Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this `meta-signal-cloud` contract.
Workspace-shape intent stays in the primary workspace `primary/INTENT.md`.
Component daemon intent stays in `cloud/INTENT.md`. Ordinary provider observation
and validation stays in `signal-cloud/INTENT.md`.

## Why this repo exists

`meta-signal-cloud` is the **owner-only meta policy contract** for the `cloud`
component. It controls provider account registration, credential-handle rotation,
policy changes, plan preparation, and live provider plan application; it also
accepts provider-neutral projections from `domain-criome` and turns them into
daemon-held provider plans. The split is load-bearing: the ordinary `signal-cloud`
contract observes and validates provider-neutral desired state, while this meta
contract prepares, authorizes, and applies plans — because prepared plans are
daemon-owned mutation intent, and live provider mutation changes external
accounts, paid resources, and public domain identity.

This realizes a workspace generalization (per Spirit records 311 and 325): a
component whose state surface is a reflected external resource exposes its read
surface on the ordinary contract and its mutation surface on the meta contract.

## The channel shape

The meta channel carries:

- **`RegisterAccount(Registration)`** — bind a provider account to a credential
  handle.
- **`RotateCredential(Rotation)`** — change the credential handle for an account.
- **`SetPolicy(Policy)`** — replace the daemon's provider-authority policy.
- **`PreparePlan(PlanPreparation)`** — write a provider plan into daemon plan state.
- **`PrepareProjection(ProjectionPreparation)`** — accept a `signal-domain-criome`
  projection and let `cloud` lower it into a provider plan under owner authority.
- **`ApprovePlan(Approval)`** — mark a prepared plan as approved for later application.
- **`ApplyPlan(Application)`** — apply a prepared plan.
- **`RetireAccount(Retirement)`** — remove an account binding.

`PreparePlan` lives here because it mutates daemon-internal plan-store state, even
though it does not mutate external provider state directly; Cloudflare and other
provider states are treated as external state the cloud daemon reflects.

## Constraints

- Mutate-class verbs live on this meta contract (privileged); Query-class verbs
  live on `signal-cloud` (public).
- Owner-only operations live here because caller authority, not touched state,
  determines the contract split.
- Raw provider credential bytes are never exposed — this crate carries
  secret-handle references, not secret bytes.
- Depend on `signal-frame`, not deprecated `signal-core`. Reuse public
  provider/domain/plan types from `signal-cloud` and provider-neutral projection
  types from `signal-domain-criome` at the cloud/domain handoff boundary.
- This crate carries only typed wire vocabulary, NOTA codecs, and round-trip
  witnesses — no daemon actor tree or database.

## Non-ownership

This crate does not own:

- ordinary provider observation or provider-neutral desired state;
- the runtime daemon's actor tree or database;
- the Criome domain registry;
- raw provider credential bytes.

## See also

- `ARCHITECTURE.md` — public operations, the ordinary/owner split, and constraints.
- `../cloud/INTENT.md` — daemon-side intent (provider plans, application, reflection).
- `../signal-cloud/INTENT.md` — ordinary provider observation/validation contract.
- `../signal-domain-criome/INTENT.md` — provider-neutral projection vocabulary.
- `primary/skills/contract-repo.md` — contract repo discipline and naming rules.
- `primary/skills/component-triad.md` — repo triad structure and authority tiers.
