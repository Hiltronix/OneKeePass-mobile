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

The RustSec and Yarn tools do **not** scan vulnerabilities in Gradle/AAR/JNI
dependencies. Their security advisories and resulting resolved release
classpath still require a separate review, including AndroidX Credentials,
AndroidX Security Crypto, React Native and native module versions.

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
