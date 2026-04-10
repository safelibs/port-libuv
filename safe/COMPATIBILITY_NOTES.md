Linux compatibility notes for the Rust port:

- The Linux backend remains pinned to epoll for this release candidate. The translated `io_uring` support code is present only to preserve symbol and source compatibility where upstream exposes related internals, but runtime feature gating keeps the backend disabled.
- This is intentional for the final parity sweep. Re-enabling `io_uring` late would reintroduce correctness and privilege-drop risk around `CVE-2024-22017` without improving the verifier contract for Ubuntu 24.04.
- Public headers, exported symbols, Debian packaging, and the staged/shared/static link contracts remain aligned with the upstream libuv baseline.
