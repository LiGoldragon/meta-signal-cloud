# owner-signal-cloud — Agent Instructions

Read `~/primary/AGENTS.md`, then this file.

This repository is a pure owner Signal contract crate. It declares
owner-only cloud-provider authority and policy records. It contains no
daemon, storage, actors, provider clients, or secret bytes.

Secret material crosses this contract only as durable secret handles.
Do not add fields that carry provider tokens, passwords, or private keys.
