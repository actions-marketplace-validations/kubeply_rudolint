# Performance Budgets

These budgets are advisory until benchmark fixtures are stable. They exist to
keep design decisions honest before the first usable release.

Initial targets:

- cold start: under 50 ms on a typical CI Linux runner.
- one Dockerfile: under 20 ms after process start.
- 1,000 small Dockerfiles: under 2 seconds.
- JSON output overhead: under 15 percent of lint time for large fixture sets.
- SARIF output overhead: under 30 percent of lint time for large fixture sets.
- release binary size: track every release; warn on unexpected growth over 20
  percent.

Benchmark runs should record hardware, OS, target triple, commit SHA, and
command line. CI performance jobs should start advisory-only and run on `main`
until the corpus is stable enough to gate pull requests.
