# Architecture

`rudolint` is split into four layers.

## Parser

The parser owns source spans and Dockerfile syntax. It should understand:

- parser directives
- instruction continuations
- flags on `FROM`, `RUN`, `COPY`, and `ADD`
- BuildKit mounts
- heredocs
- stage aliases

The parser should not decide whether code is good or bad.

## Model

The model turns parsed instructions into facts rules can consume:

- stages
- base images
- copied files
- `RUN` commands
- BuildKit frontend version
- mount graph
- declared build arguments
- environment variables

This layer is intentionally thin at the baseline stage.

## Rules

Rules consume the model and produce diagnostics. Rules must be deterministic and
must not perform network or filesystem access unless the CLI explicitly enables
repository-wide analysis.

Compatibility rules live under `RDL` and `RSC`. BuildKit-native rules live under
`RDK`.

## Output

Output renderers convert diagnostics into human text, JSON, SARIF, and future
editor/LSP responses. Renderers must not change rule behavior.
