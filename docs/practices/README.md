# Rust Clean Code Agent Guide

This document set is written for coding agents and maintainers working on Rust codebases.
The rules are intentionally operational: agents should treat every `MUST` as a quality gate.

## Core contract

1. Every handwritten Rust source file MUST stay at or below 100 physical lines.
2. Split larger files into focused, domain-specific modules before finishing the task.
3. Modules, types, fields, functions, and traits MUST expose only the minimum API needed.
4. The workspace MUST keep at least 80% line coverage, measured by the agreed coverage command.
5. Code MUST compile, be formatted, pass Clippy, and pass all tests before it is considered complete.
6. Clean Code and SOLID principles MUST be applied in a Rust-native way, not by forcing Java-style class hierarchies.

## How agents should use these files

Read files in this order:

1. `00-agent-contract.md` for non-negotiable behaviour.
2. `01-rust-style-and-idioms.md` for Rust coding expectations.
3. `02-modules-visibility-and-api-boundaries.md` for privacy and public APIs.
4. `03-file-size-and-domain-module-splitting.md` for the 100-line source-file rule.
5. `04-solid-in-rust.md` for SOLID translated into Rust.
6. `05-clean-code-principles.md` for Clean Code-style readability rules.
7. `06-testing-and-80-percent-coverage.md` for test and coverage expectations.
8. `07-ci-quality-gates.md` for commands that prove compliance.
9. `08-review-checklists.md` for final review.
10. `09-rust-snippets-and-patterns.md` for small reusable patterns.
11. `10-agent-prompt-template.md` for a reusable instruction block.
12. `SOURCES.md` for reference links used when writing these guides.

## Conflict resolution

When a repository already contains stricter standards, follow the stricter standard.
When a repository contains weaker standards, follow this guide unless the user explicitly asks otherwise.
When any rule conflicts with safety, security, or correctness, prioritize safety, security, and correctness.

## Definition of done

A change is not done until:

- no handwritten Rust source file exceeds 100 physical lines;
- public API surface is justified and minimal;
- `cargo fmt`, `cargo clippy`, tests, and coverage gates pass;
- changed behaviour is covered by fast unit tests;
- complex logic is decomposed into names, modules, and tests that explain intent.
