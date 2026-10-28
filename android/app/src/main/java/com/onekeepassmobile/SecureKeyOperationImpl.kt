package com.onekeepassmobile

import android.content.SharedPreferences
import android.util.Log
import androidx.biometric.BiometricManager
import androidx.biometric.BiometricManager.Authenticators.BIOMETRIC_STRONG
import androidx.biometric.BiometricManager.Authenticators.BIOMETRIC_WEAK
import androidx.security.crypto.EncryptedSharedPreferences
import com.facebook.react.bridge.ReactApplicationContext
import onekeepass.mobile.ffi.SecureKeyOperation
import androidx.security.crypto.MasterKey

class SecureKeyOperationImpl(val reactContext: ReactApplicationContext) : SecureKeyOperation {

    private val TAG = "SecureKeyOperation";

    private var sharedPreferences: SharedPreferences? = null

    init {
        createStore()

    }

    fun createStore() {
        // Uses android-keystore to maintain the main key.  This key is maintained by android secure system
        // and we do not have access to this key. Encryption and decryption is done on the keystore side
        // and the final value is returned to us.
        val mainKey = MasterKey.Builder(reactContext, "okp_keys")
                .setKeyScheme(MasterKey.KeyScheme.AES256_GCM)
                .build()

        val sharedPrefsFile: String = "okp_key_store"

        // We use EncryptedSharedPreferences to store the OneKeePass key
        sharedPreferences = EncryptedSharedPreferences.create(
                reactContext,
                sharedPrefsFile,
                mainKey,
                EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
                EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
        )

    }

    override fun storeKey(dbKey: String, encKeyData: String) {
        // encKeyData is already encrypted in onekeepass core
        // In case of android, we end up doing encryption/decryption twice - one in onekeepass core itself
        // again in using MasterKey with EncryptedSharedPreferences
        with(sharedPreferences!!.edit()) {
            putString(dbKey, encKeyData)

            // Needs to call apply for committing the changes
            apply()
        }
    }

    override fun getKey(dbKey: String): String? {
        val k = sharedPreferences?.getString(dbKey, "NoKey")

        return k
    }

    override fun deleteKey(dbKey: String) {
        with(sharedPreferences!!.edit()) {
            remove(dbKey)
            apply()
        }
    }
}