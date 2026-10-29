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

- 1,271 package dependency occurrences included in Yarn's audit.
- 434 reported advisory occurrences, representing **87 distinct advisories**
  across **27 package names**.
- Unique advisory severity counts: **2 critical**, **56 high**,
  **23 moderate**, **6 low**.
- The report contains repeated dependency paths. Its counts do **not** mean
  434 independent vulnerabilities in the installed Android executable.
- The two distinct critical advisories involve `shell-quote` and
  `babel-traverse`, primarily via React Native CLI/Babel development tooling.
- Additional advisories touch `@xmldom/xmldom`, `sharp`, `brace-expansion`,
  `minimatch`, `js-yaml`, `nanoid`, `picomatch`, `lodash`, and more.
- Before changing a transitive pin, identify its parent dependencies, exact
  fixed version and compatibility with React Native 0.85.3 and the current
  ClojureScript/Metro bundle.
- Differentiate code used at build-time from JavaScript or native libraries
  actually packaged into the production APK. Build-time tooling still matters
  to supply-chain integrity, but exposure and remediation urgency differ.

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
