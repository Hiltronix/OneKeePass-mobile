#!/usr/bin/env bash
# Build the arm64 FOSS Android APK from the checked-out Hiltronix source.
# Signing is deliberately separate: never keep release keys in this checkout.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

case "$(uname -s)" in
    Darwin|Linux) ;;
    *) echo "This build script requires macOS or Linux." >&2; exit 1 ;;
esac

for program in cargo rustc just yarn java; do
    command -v "$program" >/dev/null 2>&1 || {
        echo "Missing required tool: $program" >&2
        exit 1
    }
done

if [[ -z "${ANDROID_HOME:-${ANDROID_SDK_ROOT:-}}" ]]; then
    echo "Set ANDROID_HOME (or ANDROID_SDK_ROOT) to your Android SDK directory." >&2
    exit 1
fi

sdk="${ANDROID_HOME:-$ANDROID_SDK_ROOT}"
ndk="${OKP_ANDROID_NDK_HOME:-$sdk/ndk/28.2.13676358}"
if [[ ! -f "$ndk/source.properties" ]]; then
    echo "Android NDK r28c not found at $ndk" >&2
    exit 1
fi
if ! grep -Eq '^Pkg.Revision[[:space:]]*=[[:space:]]*28\.2\.13676358[[:space:]]*$' "$ndk/source.properties"; then
    echo "Unexpected NDK revision at $ndk (expected 28.2.13676358)." >&2
    exit 1
fi

case "$(uname -s)" in
    Darwin) ndk_host=darwin-x86_64 ;;
    Linux) ndk_host=linux-x86_64 ;;
esac
test -x "$ndk/toolchains/llvm/prebuilt/$ndk_host/bin/llvm-ar" || {
    echo "NDK LLVM toolchain is missing at $ndk" >&2
    exit 1
}
command -v cargo-ndk >/dev/null 2>&1 || {
    echo "cargo-ndk missing; install the version documented in docs/BUILD.md" >&2
    exit 1
}
export ANDROID_HOME="$sdk"
export ANDROID_NDK_HOME="$ndk"
export OKP_ANDROID_NDK_HOME="$ndk"
export OKP_FOSS_BUILD=1

echo "==> Installing locked JavaScript dependencies"
yarn install --frozen-lockfile

echo "==> Compiling the UniFFI binding generator"
(cd uniffi-bindgen && cargo build --release --locked)

echo "==> Compiling production ClojureScript UI"
(cd src-cljs && just shre)

echo "==> Compiling arm64 Rust library and generating Kotlin bindings"
(cd db-service-ffi && just bca-r)

echo "==> Compiling unsigned release APK (FOSS build)"
if [[ -f android/gradle/wrapper/gradle-wrapper.jar ]]; then
    (cd android && ./gradlew clean assembleRelease)
else
    # Upstream excludes the Gradle wrapper JAR via *.jar in .gitignore.
    # A separately installed Gradle of the exact wrapper version is needed.
    command -v gradle >/dev/null 2>&1 || {
        echo "Missing android/gradle/wrapper/gradle-wrapper.jar and Gradle 9.3.1." >&2
        echo "Install Gradle 9.3.1 or generate the wrapper with a trusted Gradle installation." >&2
        exit 1
    }
    gradle --version | grep -Eq '^Gradle 9\.3\.1([[:space:]]|$)' || {
        echo "Installed Gradle must be version 9.3.1." >&2
        exit 1
    }
    (cd android && gradle clean assembleRelease)
fi

apk="$root/android/app/build/outputs/apk/release/app-arm64-v8a-release-unsigned.apk"
if [[ ! -s "$apk" ]]; then
    echo "Expected unsigned release APK not found: $apk" >&2
    exit 1
fi

echo "==> APK: $apk"
if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$apk"
else
    shasum -a 256 "$apk"
fi
echo "Release APK is UNSIGNED. Sign privately and verify its manifest before installation."
