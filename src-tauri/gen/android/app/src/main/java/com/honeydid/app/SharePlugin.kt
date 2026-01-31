package com.honeydid.app

import android.content.Intent
import androidx.core.content.FileProvider
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin
import java.io.File

@TauriPlugin
class SharePlugin(private val activity: android.app.Activity) : Plugin(activity) {

    @Command
    fun shareFile(invoke: Invoke) {
        try {
            val filePath = invoke.getString("filePath", "")
            val mimeType = invoke.getString("mimeType", "text/html")

            if (filePath.isNullOrEmpty()) {
                invoke.reject("No filePath provided")
                return
            }

            val file = File(filePath)
            if (!file.exists()) {
                invoke.reject("File not found: $filePath")
                return
            }

            val uri = FileProvider.getUriForFile(
                activity,
                "${activity.packageName}.fileprovider",
                file
            )

            val shareIntent = Intent(Intent.ACTION_SEND).apply {
                type = mimeType
                putExtra(Intent.EXTRA_STREAM, uri)
                addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
            }

            activity.startActivity(Intent.createChooser(shareIntent, "Save or share your file"))
            invoke.resolve()
        } catch (e: Exception) {
            invoke.reject("Share failed: ${e.message}")
        }
    }
}
