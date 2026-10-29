# Hiltronix OneKeePass Android dependency advisory review

**Snapshot:** GitHub Actions run 40879025336 (2026-10-29), RustSec and Yarn Classic.
**Source:** Hiltronix/OneKeePass-mobile, offline Android security branch.
**Status:** Outstanding advisories require reachability and dependency-tree triage.
The scan was intentionally performed without automatic upgrades.

## Rust, locked dependencies

- 286 Cargo.lock dependencies inspected.
- 1 reported vulnerability: `RUSTSEC-2023-0071`, `rsa 0.9.10`. This is the
  Marvin timing side-channel advisory, with no generally available patch in
  this audit snapshot. Retained as an explicitly documented residual risk.
- Unsoundness warning: `RUSTSEC-2026-0386`, `tokio 1.52.3`,
  `AsyncFd` constructors can violate I/O safety if called with invalid or
  improperly owned descriptors. Verify whether the Android binary actually
  uses the affected API before making a disruptive core dependency upgrade.
- Unmaintained warning: `RUSTSEC-2026-0374`, `basic-toml 0.1.10`.
  Investigate dependency path, use of the crate and replacement compatibility.
- Yanked dependencies: `der 0.8.0` and `spin 0.9.8`. Investigate
  appropriate updates separately from security fixes.

## JavaScript, locked dependencies

- Initial audit: **87 distinct advisories** across **27 package names**,
  representing 434 occurrences through dependency paths (1,271 packages).
- After six compatible security pins, the follow-up audit at
  https://github.com/Hiltronix/OneKeePass-mobile/actions/runs/40884754628
  reports **60 distinct advisories** across **21 package names**.
  **27 advisories were eliminated, with none newly introduced.**
- Remaining distinct severity counts: **1 critical**, **35 high**,
  **18 moderate**, **6 low**.
- Remaining repeated-path occurrence counts: **2 critical**, **189 high**,
  **85 moderate**, **66 low**, totaling **342**. These are not equivalent
  to 342 exploitable flaws in the installed Android binary.
- Fixed and CI-tested: `shell-quote 1.12.0`, `nanoid 3.3.20`,
  `@xmldom/xmldom 0.8.15`,
  `@babel/plugin-transform-modules-systemjs 7.29.4`,
  `browserslist 4.28.7`, `launch-editor 2.14.2`.
- The remaining critical advisory is `GHSA-67hx-6x53-jw92`
  (`CVE-2023-45133`) in legacy `babel-traverse 6.26.0`, required by
  `@cljs-oss/module-deps 1.1.1`. The dependency is used in the ClojureScript
  build toolchain, not as an Android app service.
- The upstream advisory describes execution of attacker-crafted JavaScript
  during compilation when an affected plugin calls `path.evaluate()` or
  `path.evaluateTruthy()`. Compiling only reviewed, trusted project sources
  reduces exposure, but does not eliminate build-supply-chain attacks.
  Reference: https://github.com/babel/babel/security/advisories/GHSA-67hx-6x53-jw92
- Babel 6 is unsupported and has **no same-major security fix**. Simply
  overriding `babel-traverse 6.26.0` with `@babel/traverse 7.x` is unsafe
  because the internal AST and traversal APIs changed.
- **Disposition:** unresolved build-time conditional RCE exposure, not a
  demonstrated reachable exploit in the installed offline APK. Keep GitHub
  Actions compiling only reviewed code, keep permissions read-only, and
  investigate a migration of `@cljs-oss/module-deps` separately. Do not
  claim the critical advisory fixed or silently waive it.
- Other unresolved advisories involve `sharp`, `brace-expansion`,
  `minimatch`, `js-yaml`, `picomatch`, `lodash` and other packages.
  Prioritize packages actually bundled into the APK or used with untrusted
  sources in the build environment.

## Gradle and native Android dependencies

The dedicated Maven/OSV workflow at
https://github.com/Hiltronix/OneKeePass-mobile/actions/runs/40880823352
successfully resolved **103** Android `releaseRuntimeClasspath` Maven
dependencies. It found **one** advisory-affected coordinate:

- `com.squareup.okio:okio:2.9.0`, `GHSA-w33c-445m-f8w7`
  (`CVE-2023-3635`, medium severity / CVSS 5.9).
- Okio `GzipSource` can throw an uncaught error on malformed gzip data,
  potentially causing denial of service.
- The advisory lists `3.4.0` as the first patched Okio 3.x release.
- The runtime dependency graph shows this Okio version is selected through
  React Native `0.85.3` and `okhttp:4.9.2`. A forced Okio 3.x override
  may introduce binary/API compatibility problems with OkHttp 4.9.2.
- The Hiltronix APK has no Internet permission, reducing remote exploit
  reachability. A malicious local gzip input path has not been ruled out.
- Treat this as an **unresolved residual advisory** until an Android-compatible
  patched upgrade is compiled and behavior tested, or the affected API is
  demonstrably unreachable.

The scan covers resolved release Maven coordinates, but **not every native
JNI binary or all build-plugin dependencies**. Review these separately, along
with AndroidX Credentials, AndroidX Security Crypto and React Native versions.

## Required before closing the audit

1. Triage all critical and high advisories for reachability in the built APK
   and in the build pipeline. Fix compatible versions or document justified
   residual risks by advisory ID.
2. Scan resolved Android Gradle and native AAR/JNI dependencies.
3. Verify final signed APK certificate, package ID, permissions and SHA-256.
4. Run disposable KDBX, recovery, Autofill, passkey, biometric and
   interoperability tests in `HILTRONIX-ANDROID-SECURITY-VALIDATION.md`.
5. Keep permanently signed test APKs off production data until these checks pass.

The full machine-readable reports are attached to:
https://github.com/Hiltronix/OneKeePass-mobile/actions/runs/40879025336
