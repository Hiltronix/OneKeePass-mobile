# Hiltronix Android FOSS baseline build (arm64)

This procedure builds the currently configured **OneKeePass mobile v0.24.0** from this fork, using the pinned Hiltronix shared Rust core and the existing upstream FOSS code path.

**Security warning:** The current Android manifest **still declares `android.permission.INTERNET` and `android.permission.CAMERA`**. The custom package ID, offline-only behavior and network permission removal have **not** been implemented. Do not install this baseline APK as a hardened replacement for your official password manager.

## Host and tools

The provided script is for **macOS or Linux**, not native Windows. It can be run on a Linux build machine/VM; the existing Rust `just` recipes use Bash and Android NDK Unix toolchains.

Install:

- Rust (the `db-service-ffi/rust-toolchain.toml` pins **1.91.0**)
- `cargo-ndk` **3.5.7**, `just` (1.40+)
- Node.js (20.19.4+), Yarn Classic 1.22.x
- JDK 17 or 21
- Android SDK, build tools, and side-by-side NDK versions **28.2.13676358** (Rust/Botan) and **27.1.12297006** (Gradle).

Follow upstream `docs/BUILD.md` for the complete toolchain prerequisites. Also install **Gradle 9.3.1** if the Gradle wrapper JAR is missing. The current repository does not contain `android/gradle/wrapper/gradle-wrapper.jar`; its root `.gitignore` excludes `*.jar`. The build script uses the locally installed, version-checked Gradle 9.3.1 as a fallback.

## Build

Configure `ANDROID_HOME` to the SDK directory, where `ndk/28.2.13676358` exists. Alternatively set `OKP_ANDROID_NDK_HOME` to that NDK's absolute path.

```bash
export ANDROID_HOME="$HOME/Android/Sdk" # Linux example; change for your installation
bash scripts/build-android-foss.sh
```

The script:

1. Runs `yarn install --frozen-lockfile` to restore locked JS packages and patches.
2. Builds `uniffi-bindgen` with `cargo build --release --locked`.
3. Runs `just shre` to compile the production ClojureScript UI.
4. Runs `just bca-r` to build the arm64 Rust library and generate Kotlin bindings.
5. Runs Gradle `clean assembleRelease` with `OKP_FOSS_BUILD=1` for the camera-free JavaScript build, and checks the output file exists.

The Rust `justfile` now detects the host's NDK prebuilt toolchain (`darwin-x86_64` or `linux-x86_64`) instead of embedding the upstream developer's personal macOS path.

Expected result:

```text
android/app/build/outputs/apk/release/app-arm64-v8a-release-unsigned.apk
```

## Signing and validation

The APK is **unsigned**. Keep your private release signing key **outside this repository**, never commit a keystore or passwords. Android builds distributed for use require signing; follow the official Android `zipalign`/`apksigner` process using your private key.

Before installing:

- Verify APK signatures using `apksigner verify --verbose --print-certs <apk>`.
- Inspect the final merged manifest and APK permissions using Android SDK tools (e.g. `aapt dump permissions <apk>` or `apkanalyzer manifest permissions <apk>`).
- Validate that any future permission-removal work eliminates `INTERNET` from the **final APK**, not merely the source manifest.
- Test open, unlock, edit, save, attachment and OneKeePass-template compatibility using disposable KDBX test databases.

The current application ID is `com.onekeepassmobile`. Do **not** install this privately signed version over the Play Store version: Android will reject updates with different signatures, and maintaining separate installations will require a distinct application ID and consideration of Android AutoFill/credential-provider registration.

**Status:** Build script, path configuration and documentation are committed, but full native compilation, Gradle packaging and installation have **not been executed** here. The upstream release guide itself notes that byte-for-byte APK reproducibility varies by OS, paths and native toolchains.

## Later hardening work

Removing the `INTERNET` permission and auditing indirect data-transfer paths, disabling unwanted network functionality and update paths, and setting up independent package identity and private signing are subsequent steps, not accomplished by this baseline build.

## GitHub Actions: installable test APK

The workflow [Hiltronix Android ARM64 test APK](../.github/workflows/hiltronix-android-test.yml) compiles the unsigned release output on a GitHub-hosted Linux runner, signs a **test-only** copy with a temporary CI key, verifies that signature and uploads the installable APK.

To obtain the test build:

1. Open the [Android Actions runs](https://github.com/Hiltronix/OneKeePass-mobile/actions/workflows/hiltronix-android-test.yml).
2. Open a **successful** run (green checkmark). If there are none, inspect that run's build log; no APK exists until compilation and signing finish.
3. Under **Artifacts**, download the \`Hiltronix-Android-arm64-TEST-...\` ZIP.
4. Extract \`OneKeePass-Hiltronix-Android-arm64-TEST.apk\`, transfer it to your Android phone, and open it from the phone's Files app to sideload it. Approve the Android installation-from-this-source prompt if needed.

This test variant uses application ID **\`com.onekeepassmobile.hiltronix\`**. It is intended to coexist with the official OneKeePass. It is **not** a verified offline build: the baseline main manifest still requests INTERNET and CAMERA. Use only a disposable KDBX database while testing.

The test signing key is generated **afresh on every workflow run** and is discarded. You cannot install a different run's APK as an update over an earlier test build: uninstall the earlier Hiltronix test app first (its app-private data will be lost). The signing key is not uploaded. Do not use this mechanism for production signing.
