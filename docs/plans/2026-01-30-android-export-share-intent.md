# Android Export via Share Intent

## Problem

Exporting on Android is broken: hardcoded filesystem paths (`/sdcard/Download`, `/storage/emulated/0/Download`) don't work reliably across devices. Tauri's `save()` dialog on Android returns content URIs that break `std::fs::write`. The file appears to save but nothing is actually there.

## Solution

Write the export file to the app's **cache directory** (always writable, no permissions needed), then use Android's native **share intent** (`ACTION_SEND`) via `FileProvider` to let the user choose where to save or send the file. Desktop export stays unchanged.

## Implementation Steps

### Step 1: Update `save_html_to_downloads` (Rust)

In `src-tauri/src/lib.rs`, change the Android path in `get_download_dir()` to use the app's cache directory instead of trying to find an external Downloads folder:

- Use `std::env::var("TMPDIR")` which on Android points to the app's cache dir
- Fall back to `storage::get_data_dir()` joined with `"exports"`
- The file written here is temporary — the share intent is what delivers it to the user

### Step 2: Create `SharePlugin.kt`

Create a new Kotlin file at `src-tauri/gen/android/app/src/main/java/com/honeydid/app/SharePlugin.kt`:

- Extends `app.tauri.plugin.Plugin`
- Annotated with `@TauriPlugin`
- Has a `@Command` method `shareFile` that:
  1. Receives a file path and MIME type from the invoke args
  2. Creates a `content://` URI via `FileProvider.getUriForFile()`
  3. Builds an `Intent` with `ACTION_SEND`, `EXTRA_STREAM`, and `FLAG_GRANT_READ_URI_PERMISSION`
  4. Launches `Intent.createChooser()` so the user picks the destination
  5. Returns success to the Rust caller

### Step 3: Register the plugin in `MainActivity.kt`

Override `onCreate` in `MainActivity` to register the `SharePlugin` with the Tauri app handle so the `share_file` command becomes available to the frontend.

### Step 4: Add `share_file` Tauri command (Rust)

In `src-tauri/src/lib.rs`:

- Add a new `share_file` command that takes `file_path` and `mime_type` strings
- On Android (`#[cfg(target_os = "android")]`): use `tauri::plugin::PluginHandle` to invoke the Kotlin `shareFile` method
- On desktop (`#[cfg(not(target_os = "android"))]`): no-op or return an error (desktop uses the save dialog)

### Step 5: Update `ExportDialog.svelte`

Change the mobile export flow in `saveHtmlFile()`:

- Call `save_html_to_downloads` to write the file to cache (returns the path)
- Then call `share_file` with that path and `text/html` MIME type
- Update the success message to "File ready to share"
- Remove the path display (the user chose the destination via the share sheet)

## Files Changed

| File | Change |
|------|--------|
| `src-tauri/src/lib.rs` | Update `get_download_dir()` Android impl to use cache dir; add `share_file` command |
| `src-tauri/gen/android/app/src/main/java/com/honeydid/app/SharePlugin.kt` | New file — Kotlin plugin for Android share intent |
| `src-tauri/gen/android/app/src/main/java/com/honeydid/app/MainActivity.kt` | Register SharePlugin |
| `src/lib/components/ExportDialog.svelte` | Call `share_file` after writing to cache on mobile |

## What stays the same

- Desktop export flow (save dialog + writeTextFile) is untouched
- All encryption/HTML generation logic unchanged
- FileProvider already configured in AndroidManifest.xml and file_paths.xml
