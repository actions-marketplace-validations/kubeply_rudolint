# Rule Documentation

Each implemented rule should have a dedicated page named after the rule code,
for example `RDL3007.md` or `RDK1000.md`.

Rule pages should include:

- rule code and title.
- default severity.
- rule family and category.
- provenance.
- rationale.
- bad example.
- good example.
- configuration notes.
- autofix behavior:
  - safe automatic fix.
  - manual suggestion.
  - no-fix rationale.
- compatibility notes.

`RDL` pages document Hadolint compatibility provenance. `RDK` pages document
project-native BuildKit behavior. `RSC` pages document shell-analysis behavior
implemented by `rudolint-shell`.
