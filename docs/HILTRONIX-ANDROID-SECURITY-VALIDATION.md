# Hiltronix OneKeePass Android security acceptance tests

Status: NOT YET DEVICE-VERIFIED. Use only disposable KDBX files containing no real credentials.

## Preconditions

- Download the **permanently signed** APK from the exact security-branch GitHub Actions run.
- Record source commit, APK SHA-256, signing certificate SHA-256, package ID and versionCode.
- Check that the manifest has no `INTERNET`, `ACCESS_NETWORK_STATE`, or `CAMERA` permission.
- Keep an independent, known-good copy of every disposable test KDBX outside the app.
- Confirm the installed package is `com.onekeepassmobile.hiltronix` and can be updated by a second build with the same signing certificate.
- Use an Android 16 device. Repeat with at least one additional SAF document provider if available.
- Disable all network interfaces and check that none of the operations below depend on access to a server.

## Functional and security acceptance

| ID | Test | Required result |
| --- | --- | --- |
| DB-01 | Create an empty KDBX with a disposable password, save, close app, reopen | Database opens and authenticates |
| DB-01A | Force a simulated short write or fsync failure while creating a brand-new KDBX through a test SAF provider | Creation reports failure, does not register a corrupt file as a successful database, and retains only a fully serialized encrypted recovery snapshot |
| DB-01B | Simulate a Kotlin provider close failure after native new-KDBX creation | UI does not report success if the provider fails to finalize the newly created document |
| DB-02 | Add, edit, rename and delete entries and groups | Saves persist, UUIDs remain stable, no white/blank screen |
| DB-03 | Add a binary attachment, save, close and reopen | Attachment is present and byte-identical |
| DB-04 | Open the same disposable KDBX with Hiltronix OneKeePass desktop, edit, save and reopen on Android | Group/entry/attachment data interoperates without loss |
| DB-05 | Save a larger KDBX, then delete many entries and save again | File shrinks correctly and reopens without trailing data |
| DB-06 | Revoke document write access or select a read-only provider | Save fails with an actionable error; original file remains intact |
| DB-07 | Simulate provider write failure using a disposable document provider that can fail after a short/partial write | A verified encrypted recovery snapshot remains available and recovery Save As restores modified data |
| DB-08 | Force-stop the app during a controlled slow provider write, then reopen | App either opens original file or offers a valid recovery snapshot; no silent data loss |
| DB-09 | Crash/stop during snapshot preparation, then restart | A partial snapshot is not listed as a valid backup and cannot replace a good KDBX |
| DB-10 | Test recovery Save As with a read-only target and a provider that fails on close | No false success or loss of the source recovery snapshot |
| DB-11 | Modify the same disposable KDBX externally before saving on Android | Conflict detected; user must not silently overwrite the external change |
| SEC-01 | Launch `otpauth://totp/...` from an external scanner | OTP imports correctly, without in-app camera permission |
| SEC-02 | Verify Android Autofill with locked and unlocked database, cancellation, and another application | Only selected credentials are filled after authentication |
| SEC-03 | Exercise passkey registration and sign-in with a test relying party | RP ID, challenge, requested credential and response are correct |
| SEC-04 | Test Android Keystore and biometric unlock, including denial and re-enrollment | Denial does not expose secrets; invalidated credentials fail closed |
| SEC-05 | Copy a protected password to clipboard and wait for configured expiration | Sensitive labeling and clipboard cleanup behave as designed |
| SEC-06 | Check native/JavaScript logging with Logcat while using *disposable* secrets | No KDBX passwords, entry fields, OTP secrets, private keys or raw URI query secrets appear |
| SEC-07 | Test airplane mode and local firewall monitoring throughout | No unexpected outbound network connections or dependency on network services |
| SEC-08 | Check APK permission list and package/service exports | No network/camera permission or unintended public authentication entry point |

## Interrupted-write test guidance

Use a **disposable KDBX** and a test document provider that can be configured to fail
after an exact number of bytes, on `fsync` or on descriptor close. Merely pressing
Force Stop at a random instant is not a reliable corruption test. Verify file
SHA-256 before/after failures and separately verify the recovered database by
opening it with the correct test password. A valid backup's existence does not
prove the provider's currently stored document is valid.

A SAF provider may buffer writes, reject seeking/truncation or perform deferred
sync after closing a descriptor. The staged backup substantially reduces data
loss but does not make an arbitrary provider's write atomic. Do not claim full
power-loss durability without tests on the actual storage/provider combination.

## Completion record

Record device build and Android version, provider names, test timestamps, APK
SHA-256 and signing certificate fingerprint, test outcome and reproducible
failure details for each ID. Do not copy real passwords, private keys, OTP seeds
or customer data into GitHub issues or build logs.

**Audit closure requires:** a passing signed APK, reconciled RustSec/Yarn/Gradle
advisories, reviewed exported components, and passing device tests above.
