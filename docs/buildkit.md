# BuildKit And Buildx Scope

`rudolint` treats BuildKit as the default Docker build model.

## Frontend Syntax

The parser records the `# syntax=` directive and rules can use it to decide
whether features are available or whether a Dockerfile should pin a newer
frontend.

## Mounts

The parser extracts `RUN --mount=...` values into structured mount facts.

Initial mount families:

- `cache`
- `secret`
- `ssh`
- `bind`
- `tmpfs`

Rules should reason about mount type and options instead of grepping raw shell.

## Buildx

Buildx support should cover:

- multi-platform builds
- `TARGETPLATFORM`, `BUILDPLATFORM`, `TARGETARCH`, and `TARGETOS`
- stage platform pinning
- Bake file discovery and generated target contexts
- output modes that affect reproducibility

The first implementation focuses on Dockerfile syntax. Bake files should become
a separate parser so Dockerfile linting stays fast and dependency-light.

## Source Tracking

BuildKit and Buildx behavior should be implemented from tracked upstream
references, not from memory. Rule or parser changes that depend on BuildKit
semantics should cite the relevant source in the rule docs or fixture notes.

Track references for:

- Dockerfile frontend syntax versions.
- parser directives.
- `RUN --mount` option behavior.
- `RUN --network` and `RUN --security` entitlements.
- predefined BuildKit and Buildx build arguments.
- multi-platform `FROM --platform` behavior.
- Bake file syntax once Bake support starts.

The implementation plan intentionally keeps Bake parsing separate from
Dockerfile parsing so normal Dockerfile linting stays fast and dependency-light.
