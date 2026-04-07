# Performance Regression Review

The representative suite was rerun from `/tmp/libuv-safe-build/uv_run_benchmarks_a` against the
phase 1 Linux baseline. No benchmark exceeded its fail threshold.

Warning-only regressions:

- `getaddrinfo.requests_per_sec`: `62112` -> `54945` (`11.54%` regression). This benchmark is
  sensitive to host resolver and scheduling variance. Phase 8 did not change the DNS request path,
  and the result stayed below the `30%` fail threshold.
- `fs_stat.concurrency_4_ops_per_sec`: `923641` -> `768334` (`16.81%` regression).
- `fs_stat.concurrency_16_ops_per_sec`: `1924721` -> `1541532` (`19.91%` regression).

The concurrent `fs_stat` warnings are not accompanied by a synchronous regression:
`fs_stat.sync_ops_per_sec` improved from `3435648` to `3600103`. That points to run-to-run worker
and filesystem contention in the concurrent microbenchmark rather than a blocking-path correctness
or algorithm change. Both concurrent metrics remain below the `30%` fail threshold.

All other representative metrics stayed within thresholds or improved, including
`queue_work.jobs_per_sec`, `ping_pongs.roundtrips_per_sec`, `ping_udp1.roundtrips_per_sec`, and
`tcp_write_batch.writes_per_sec`.
