package com.honeydid.app

import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyPermanentlyInvalidatedException
import android.security.keystore.KeyProperties
import android.util.Base64
import androidx.biometric.BiometricManager
import androidx.biometric.BiometricPrompt
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import java.security.KeyStore
import javax.crypto.Cipher
import javax.crypto.KeyGenerator
import javax.crypto.SecretKey
import javax.crypto.spec.IvParameterSpec

@InvokeArg
class EnrollArgs {
    var password: String = ""
}

@TauriPlugin
class BiometricPlugin(private val activity: android.app.Activity) : Plugin(activity) {

    companion object {
        private const val KEYSTORE_ALIAS = "honey_did_biometric_key"
        private const val PREFS_NAME = "biometric_prefs"
        private const val KEY_ENCRYPTED_PASSWORD = "encrypted_password"
        private const val KEY_ENCRYPTION_IV = "encryption_iv"
        private const val TRANSFORMATION = "AES/CBC/PKCS7Padding"
    }

    @Command
    fun checkBiometricAvailability(invoke: Invoke) {
        try {
            val biometricManager = BiometricManager.from(activity)
            val result = biometricManager.canAuthenticate(BiometricManager.Authenticators.BIOMETRIC_STRONG)
            val obj = JSObject()
            when (result) {
                BiometricManager.BIOMETRIC_SUCCESS -> {
                    obj.put("available", true)
                    obj.put("enrolled", true)
                }
                BiometricManager.BIOMETRIC_ERROR_NONE_ENROLLED -> {
                    obj.put("available", true)
                    obj.put("enrolled", false)
                }
                else -> {
                    obj.put("available", false)
                    obj.put("enrolled", false)
                }
            }
            invoke.resolve(obj)
        } catch (e: Exception) {
            invoke.reject("Biometric check failed: ${e.message}")
        }
    }

    @Command
    fun enrollBiometric(invoke: Invoke) {
        try {
            val args = invoke.parseArgs(EnrollArgs::class.java)
            val password = args.password

            if (password.isEmpty()) {
                invoke.reject("No password provided")
                return
            }

            // Generate (or regenerate) the Keystore key
            generateKey()

            // Init cipher in encrypt mode
            val cipher = getCipher()
            val secretKey = getKey()
            if (secretKey == null) {
                invoke.reject("Failed to create biometric key")
                return
            }
            cipher.init(Cipher.ENCRYPT_MODE, secretKey)

            val fragmentActivity = activity as FragmentActivity
            val executor = ContextCompat.getMainExecutor(activity)

            val callback = object : BiometricPrompt.AuthenticationCallback() {
                override fun onAuthenticationSucceeded(result: BiometricPrompt.AuthenticationResult) {
                    try {
                        val authenticatedCipher = result.cryptoObject?.cipher
                        if (authenticatedCipher == null) {
                            invoke.reject("Authentication succeeded but cipher unavailable")
                            return
                        }
                        val encrypted = authenticatedCipher.doFinal(password.toByteArray(Charsets.UTF_8))
                        val iv = authenticatedCipher.iv

                        // Store encrypted password and IV
                        val prefs = activity.getSharedPreferences(PREFS_NAME, android.content.Context.MODE_PRIVATE)
                        prefs.edit()
                            .putString(KEY_ENCRYPTED_PASSWORD, Base64.encodeToString(encrypted, Base64.NO_WRAP))
                            .putString(KEY_ENCRYPTION_IV, Base64.encodeToString(iv, Base64.NO_WRAP))
                            .apply()

                        invoke.resolve()
                    } catch (e: Exception) {
                        invoke.reject("Enrollment failed: ${e.message}")
                    }
                }

                override fun onAuthenticationError(errorCode: Int, errString: CharSequence) {
                    invoke.reject("user_cancelled", errString.toString())
                }

                override fun onAuthenticationFailed() {
                    // Called on individual failed attempt; prompt stays open. No action needed.
                }
            }

            val promptInfo = BiometricPrompt.PromptInfo.Builder()
                .setTitle("Unlock Honey Did")
                .setSubtitle("Verify fingerprint to enable biometric unlock")
                .setNegativeButtonText("Cancel")
                .build()

            val biometricPrompt = BiometricPrompt(fragmentActivity, executor, callback)

            fragmentActivity.runOnUiThread {
                biometricPrompt.authenticate(promptInfo, BiometricPrompt.CryptoObject(cipher))
            }
        } catch (e: Exception) {
            invoke.reject("Enrollment failed: ${e.message}")
        }
    }

    @Command
    fun authenticateWithBiometric(invoke: Invoke) {
        try {
            val prefs = activity.getSharedPreferences(PREFS_NAME, android.content.Context.MODE_PRIVATE)
            val encryptedB64 = prefs.getString(KEY_ENCRYPTED_PASSWORD, null)
            val ivB64 = prefs.getString(KEY_ENCRYPTION_IV, null)

            if (encryptedB64 == null || ivB64 == null) {
                invoke.reject("no_enrollment", "No biometric enrollment found")
                return
            }

            val iv = Base64.decode(ivB64, Base64.NO_WRAP)
            val cipher = getCipher()
            val secretKey = getKey()
            if (secretKey == null) {
                clearStoredData()
                invoke.reject("key_invalidated", "Biometric key not found")
                return
            }

            try {
                cipher.init(Cipher.DECRYPT_MODE, secretKey, IvParameterSpec(iv))
            } catch (e: KeyPermanentlyInvalidatedException) {
                clearStoredData()
                invoke.reject("key_invalidated", "Biometrics have changed. Please re-enable fingerprint unlock.")
                return
            }

            val fragmentActivity = activity as FragmentActivity
            val executor = ContextCompat.getMainExecutor(activity)

            val callback = object : BiometricPrompt.AuthenticationCallback() {
                override fun onAuthenticationSucceeded(result: BiometricPrompt.AuthenticationResult) {
                    try {
                        val authenticatedCipher = result.cryptoObject?.cipher
                        if (authenticatedCipher == null) {
                            invoke.reject("Authentication succeeded but cipher unavailable")
                            return
                        }
                        val encrypted = Base64.decode(encryptedB64, Base64.NO_WRAP)
                        val decrypted = authenticatedCipher.doFinal(encrypted)
                        val password = String(decrypted, Charsets.UTF_8)

                        val obj = JSObject()
                        obj.put("password", password)
                        invoke.resolve(obj)
                    } catch (e: Exception) {
                        invoke.reject("Decryption failed: ${e.message}")
                    }
                }

                override fun onAuthenticationError(errorCode: Int, errString: CharSequence) {
                    invoke.reject("user_cancelled", errString.toString())
                }

                override fun onAuthenticationFailed() {
                    // Called on individual failed attempt; prompt stays open. No action needed.
                }
            }

            val promptInfo = BiometricPrompt.PromptInfo.Builder()
                .setTitle("Unlock Honey Did")
                .setSubtitle("Verify fingerprint to unlock")
                .setNegativeButtonText("Use Password")
                .build()

            val biometricPrompt = BiometricPrompt(fragmentActivity, executor, callback)

            fragmentActivity.runOnUiThread {
                biometricPrompt.authenticate(promptInfo, BiometricPrompt.CryptoObject(cipher))
            }
        } catch (e: Exception) {
            invoke.reject("Authentication failed: ${e.message}")
        }
    }

    @Command
    fun clearBiometricEnrollment(invoke: Invoke) {
        try {
            clearStoredData()
            // Delete the Keystore key
            try {
                val keyStore = KeyStore.getInstance("AndroidKeyStore")
                keyStore.load(null)
                if (keyStore.containsAlias(KEYSTORE_ALIAS)) {
                    keyStore.deleteEntry(KEYSTORE_ALIAS)
                }
            } catch (_: Exception) {
                // Ignore — key may not exist
            }
            invoke.resolve()
        } catch (e: Exception) {
            invoke.reject("Clear failed: ${e.message}")
        }
    }

    private fun generateKey() {
        val keyGenerator = KeyGenerator.getInstance(KeyProperties.KEY_ALGORITHM_AES, "AndroidKeyStore")
        val spec = KeyGenParameterSpec.Builder(
            KEYSTORE_ALIAS,
            KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT
        )
            .setBlockModes(KeyProperties.BLOCK_MODE_CBC)
            .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_PKCS7)
            .setUserAuthenticationRequired(true)
            .setInvalidatedByBiometricEnrollment(true)
            .build()
        keyGenerator.init(spec)
        keyGenerator.generateKey()
    }

    private fun getCipher(): Cipher {
        return Cipher.getInstance(TRANSFORMATION)
    }

    private fun getKey(): SecretKey? {
        val keyStore = KeyStore.getInstance("AndroidKeyStore")
        keyStore.load(null)
        return keyStore.getKey(KEYSTORE_ALIAS, null) as? SecretKey
    }

    private fun clearStoredData() {
        val prefs = activity.getSharedPreferences(PREFS_NAME, android.content.Context.MODE_PRIVATE)
        prefs.edit()
            .remove(KEY_ENCRYPTED_PASSWORD)
            .remove(KEY_ENCRYPTION_IV)
            .apply()
    }
}
