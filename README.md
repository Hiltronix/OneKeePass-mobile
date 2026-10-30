# Hiltronix OneKeePass Android 0.24.0 — customized security-focused build

> **Unofficial fork of [OneKeePass Mobile](https://github.com/OneKeePass/mobile).** The customized Android application is maintained on the [`security/android-logging-crypto-2026-10-28` branch](https://github.com/Hiltronix/OneKeePass-mobile/tree/security/android-logging-crypto-2026-10-28). **The `main` branch is upstream-based source, not the customized Android application.** This build is not an official OneKeePass release and is not supported by the original developers.

## Why this fork exists

The purpose is to retain **OneKeePass Mobile 0.24.0** as a controlled, independently signed Android password manager while reducing unnecessary network capabilities, updating selected dependencies and hardening database saving and authentication. The original KeePass/KDBX compatibility and everyday password-management functions are intended to remain intact.

## Hiltronix improvements and security changes

- **Version pinned to 0.24.0:** No automatic migration to later upstream versions. The customized About screen displays **Hiltronix Revision: 2026-10-29-1** underneath `v0.24.0`.
- **Independent installation and signing:** Package `com.onekeepassmobile.hiltronix` uses a separately controlled permanent signing key. It installs independently of the official app and is not replaced by Play Store updates. Future custom updates must use the same certificate.
- **Offline application:** Removed Android Internet/network-state permissions, SFTP/WebDAV functionality and supporting remote-storage interfaces, HTTP clients, remote-change UI, and URL-based icon downloading. Local KDBX files remain supported through Android's document picker.
- **Camera and scanner removal:** Removed the internal QR scanner, camera permission and camera library. TOTP setup via externally obtained `otpauth://totp` links is retained.
- **More resilient KDBX saving:** Stages encrypted recovery snapshots before overwriting an Android document; rejects incomplete and failed writes; synchronizes backup data; handles shorter files without leaving trailing bytes; and preserves recovery options following failures. Added read-only checks, partial-file cleanup, safer new-database creation and file-descriptor ownership fixes.
- **Authentication hardening:** Restricted internal Autofill and passkey activities, used explicit immutable Autofill pending intents, cleared invalid passkey request state, and corrected biometric cancellation/failure handling.
- **Reduced sensitive logging:** Removed or restricted sensitive diagnostic logging and unnecessary command exposure in release builds.
- **Native startup fix:** Corrected the Rust/Botan shared C++ runtime linkage that prevented the Android APK from launching on some devices.
- **Dependency maintenance:** Updated selected Rust cryptography/XML and JavaScript build dependencies, including `crypto-bigint`, `quick-xml`, `shell-quote`, `nanoid`, `@xmldom/xmldom`, and Babel-related tooling.
- **Build verification:** GitHub Actions checks security-sensitive source rules, encrypted-save behavior, prohibited permissions, native library dependencies, dependency lockfiles, APK signatures and package identity. Production APKs are independently signed; no signing key is committed to source control.

## Builds, testing and limitations

- **Customized source:** [Hiltronix Android security branch](https://github.com/Hiltronix/OneKeePass-mobile/tree/security/android-logging-crypto-2026-10-28).
- **Signed APK:** [Hiltronix 0.24.0, Revision 2026-10-29-1 release](https://github.com/Hiltronix/OneKeePass-mobile/releases/tag/android-v0.24.0-hiltronix-rev-2026-10-29-1) (verified permanent-signing build #115). Android package `com.onekeepassmobile.hiltronix`; APK SHA-256: `113191b4686eefbdd98f5a9e1771da36973497038e4737df25cbcf72cfde4902`.
- **Device testing:** The customized app has been reported to start, open a KDBX database and display the updated About screen on Android. This does **not** mean all error-recovery, Autofill, passkey, biometric or interoperability scenarios have been certified.
- **Residual dependency findings:** Some upstream/build-tool advisories remain under review; see the [dependency audit](https://github.com/Hiltronix/OneKeePass-mobile/blob/security/android-logging-crypto-2026-10-28/docs/HILTRONIX-ANDROID-DEPENDENCY-AUDIT-2026-10-29.md) and [device-test checklist](https://github.com/Hiltronix/OneKeePass-mobile/blob/security/android-logging-crypto-2026-10-28/docs/HILTRONIX-ANDROID-SECURITY-VALIDATION.md).
- **Original credit:** OneKeePass and its core features were developed by the [original OneKeePass maintainers and contributors](https://github.com/OneKeePass/mobile). Hiltronix claims credit only for the modifications described here. Existing GPLv3 licensing and upstream attribution remain in effect.

**Note:** The original upstream README is retained below for attribution and reference. Its references to official downloads, iOS, SFTP/WebDAV, cloud networking and other upstream features do **not** describe the customized Hiltronix Android APK.

---

<h1 align="center">
  <img src="./android/app/src/main/res/mipmap-xxxhdpi/ic_launcher.png" alt="OneKeePass" width="128" />
  <br>
  <div>OneKeePass</div>
</h1>

**OneKeePass Mobile Application** is a open source and secure password manager. It supports the KeePass-compatible database (Kdbx 4.x). It stores and manages all your passwords and other sensitive information in a single encrypted file that can be stored in any location, including local and public cloud solutions. The app relies on iOS and Android OS provided integration with storage providers.  


All created database files are encrypted using the best and most secure encryption algorithms currently known (AES-256, ChaCha20).The database is locked with a master key.

You can install OneKeePass for Android from Google Play store and OneKeePass for iOS from Apple app store

<a href="https://apps.apple.com/us/app/onekeepass-password-manager/id6448712640" target="_blank"><img alt="Get it on App Store" src="https://imgur.com/GdGqPMY.png" width="153" height="46"></a>
<a href="https://play.google.com/store/apps/details?id=com.onekeepassmobile" target="_blank"><img alt="Get it on Google Play" src="https://imgur.com/YQzmZi9.png" width="153" height="46"></a>
<a href="https://apt.izzysoft.de/packages/com.onekeepassmobile" target="_blank"><img alt="Get it at IzzyOnDroid" src="https://gitlab.com/IzzyOnDroid/repo/-/raw/master/assets/IzzyOnDroidButtonGreyBorder_nofont.png" height="46"></a>

| iOS  | Android |
| ------------- | ------------- |
| <img src="./screenshots/i_entrytypescat.png" alt="" width="325" height="650" /> | <img src="./screenshots/a_entrytypescat.png" alt="" width="300" height="650" />|

<br>

OneKeePass is also available for **Windows, macOS and Linux desktops** can be downloaded from [here](https://github.com/OneKeePass/desktop/releases)

## Features
- Entries are grouped as Types or Categories or the standard Group tree
- Any number of custom fields can be added
- Custom fields can be organized as section
- Any number of databases can be created and used
- Fully offline (you decide the location where the password database files are stored)
- Optional additional security by using a key file with or without a master password
- Database quick open or unlock using biometrics - FaceID (iOS) or Fingerprint/Face unlock (Android)
- Upload attachments to an entry
- Time based One-Time Passwords (TOTP) can added and used - the current code and its remaining time are shown on each entry list row
- Supports Light or Dark Theme
- Multi-language support
- AutoFill supported - user name, password and the one-time verification code
- Passkey (WebAuthn/FIDO2) support for passwordless sign-in and registration (iOS 17+, Android 14+)
- Native direct connection support to SFTP or WebDAV server
- Remote server credentials stored as **SFTP Connection** or **WebDAV Connection** entries inside the database — no need to re-enter credentials each time
- Custom icons support — upload icons from a file or fetch from a URL/favicon; assign to entries and groups; visible in the AutoFill extension
- Linked/Child Databases Opening feature
- Diceware Passphrase generator
- App protection with PIN locking
- Merging of databases
- Moving an entry or a group from one group to another
- Cloning of an entry
- External database change detection and merge
- Search across all entries of a database, and sorting of entries by title, created or modified time



## FAQ
Common FAQ about the application can be found [here](./docs/FAQ.md)

## How to build

OneKeePass mobile application is developed primarily using the **ClojureScript** and **Rust** programming languages. Swift and Kotlin are used for the native modules development that interact with Rust core api lib.

The apps are built with - [React Native](https://reactnative.dev/),[React Native Paper](https://reactnativepaper.com/) and many other well known react native packages 

To build and run on a simulator/emulator locally, please follow the instructions [here](./docs/BUILD.md) for both iOS and Android platforms

## How to Contribute

### Translations

If you want to contribute translation of texts used in this application in your language of choice, please see [OneKeePass Translations](https://github.com/OneKeePass/onekeepass-translations)

### Code
The CONTRIBUTING document and other guidelines for code contributions are not yet ready. Till that time, please feel free to::
- [Report bugs, suggest new features](https://github.com/OneKeePass/mobile/issues)

## License
This project is licensed under the GPLv3.
