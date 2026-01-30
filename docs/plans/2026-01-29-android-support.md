# Android Support Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Make the honey-did Tauri 2 app build and run on Android by adding responsive mobile layout, platform-gated keyring fallback, and mobile lifecycle handling.

**Architecture:** The Svelte frontend gets a responsive CSS layer that collapses the sidebar into a slide-out drawer on narrow screens. The Rust backend gets `#[cfg]`-gated alternatives for the `keyring` crate (which doesn't support Android) and the `directories` crate. App lifecycle hooks are extended to handle Android pause/resume events.

**Tech Stack:** Svelte 4, Tauri 2, Rust (cfg-gated platform code), CSS media queries

---

### Task 1: Responsive sidebar — mobile drawer

**Files:**
- Modify: `src/App.svelte`

Convert the fixed 240px sidebar into a slide-out drawer on screens narrower than 768px. Add a hamburger toggle button visible only on mobile. Overlay dims content when drawer is open.

**Changes to script section:**

Add state variables after existing state vars:

```typescript
let sidebarOpen = false;

function toggleSidebar() {
  sidebarOpen = !sidebarOpen;
}

function closeSidebar() {
  sidebarOpen = false;
}
```

Modify `navigateToSection` to close sidebar on mobile:

```typescript
function navigateToSection(sectionId: Section | string) {
  if (currentSection === 'welcome' && sectionId !== 'welcome' && hasInvalidQuestionConfig) {
    pendingSection = sectionId;
    showQuestionWarning = true;
    return;
  }
  currentSection = sectionId;
  closeSidebar();
}
```

**Changes to template:**

Add a mobile header bar inside the `{:else}` block (the main app view), before `<main class="app">`:

```svelte
<div class="mobile-header">
  <button class="hamburger" on:click={toggleSidebar} aria-label="Toggle menu">
    <span class="hamburger-line"></span>
    <span class="hamburger-line"></span>
    <span class="hamburger-line"></span>
  </button>
  <span class="mobile-title">{currentSectionLabel}</span>
</div>
```

Add `class:open={sidebarOpen}` to the `<aside class="sidebar">` element.

Add a backdrop div inside `<main class="app">`, after `</aside>`:

```svelte
{#if sidebarOpen}
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="sidebar-backdrop" on:click={closeSidebar}></div>
{/if}
```

**CSS additions (media queries):**

```css
.mobile-header {
  display: none;
}

@media (max-width: 768px) {
  .mobile-header {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    background: var(--sidebar-bg);
    color: var(--sidebar-text);
    position: sticky;
    top: 0;
    z-index: 100;
  }

  .hamburger {
    background: none;
    border: none;
    cursor: pointer;
    padding: 4px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .hamburger-line {
    display: block;
    width: 22px;
    height: 2px;
    background: var(--sidebar-text);
    border-radius: 1px;
  }

  .mobile-title {
    font-weight: 600;
    font-size: 1.1rem;
  }

  .app {
    flex-direction: column;
  }

  .sidebar {
    position: fixed;
    left: 0;
    top: 0;
    bottom: 0;
    z-index: 200;
    transform: translateX(-100%);
    transition: transform 0.25s ease;
  }

  .sidebar.open {
    transform: translateX(0);
  }

  .sidebar-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: 150;
  }

  .content {
    width: 100%;
  }

  .content-header {
    display: none;
  }

  .content-body {
    padding: 16px;
  }
}
```

**Verify:** `npx vite build` — clean build, no warnings.

---

### Task 2: Responsive form layouts

**Files:**
- Modify: `src/lib/sections/FinancialSection.svelte` (representative — pattern applies to all sections)
- Modify: `src/lib/components/FormPreview.svelte`
- Modify: `src/lib/components/CustomSectionEditor.svelte`

Sections currently use `max-width: 800px`. On mobile, forms should use full width with tighter padding.

**FinancialSection.svelte** — Add to bottom of `<style>`:

```css
@media (max-width: 768px) {
  .section {
    max-width: 100%;
  }
}
```

Apply the same pattern to all other section files that have `.section { max-width: 800px }`.

**CustomSectionEditor.svelte** — Add to bottom of `<style>`:

```css
@media (max-width: 768px) {
  .section-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }

  .section-actions {
    width: 100%;
    justify-content: flex-end;
  }
}
```

**FormPreview.svelte** — Add to bottom of `<style>`:

```css
@media (max-width: 768px) {
  .form-actions {
    flex-direction: column;
  }

  .form-actions .btn-primary,
  .form-actions .btn-secondary,
  .form-actions .btn-danger {
    width: 100%;
    text-align: center;
  }
}
```

**Verify:** `npx vite build` — clean build, no warnings.

---

### Task 3: Responsive modals and dialogs

**Files:**
- Modify: `src/lib/components/ExportDialog.svelte`
- Modify: `src/lib/components/ImportDialog.svelte`
- Modify: `src/lib/components/SettingsModal.svelte`
- Modify: `src/lib/components/SetPasswordModal.svelte`
- Modify: `src/lib/components/LockScreen.svelte`
- Modify: `src/lib/components/HelpModal.svelte`
- Modify: `src/lib/components/LicenseModal.svelte`

Dialogs currently use `max-width: 420-450px`. On mobile they should be near-fullscreen.

Add this media query to the `<style>` of each modal/dialog component:

```css
@media (max-width: 768px) {
  .dialog {
    max-width: calc(100vw - 32px);
    max-height: calc(100vh - 32px);
    overflow-y: auto;
  }
}
```

For LockScreen.svelte which uses `.lock-container` instead of `.dialog`:

```css
@media (max-width: 768px) {
  .lock-container {
    max-width: calc(100vw - 32px);
    padding: 24px;
  }
}
```

**Verify:** `npx vite build` — clean build, no warnings.

---

### Task 4: Keyring fallback for Android

**Files:**
- Modify: `src-tauri/src/storage.rs`

The `keyring` crate uses OS credential stores (Windows Credential Manager, macOS Keychain, Linux Secret Service) which are not available on Android. Add a file-based fallback for Android that stores the local encryption key in the app-private data directory.

**Replace the `get_or_create_local_key()` function with cfg-gated versions:**

```rust
/// Gets or creates a local encryption key.
/// On desktop, uses the OS keyring. On Android, uses a file in the app-private data directory.
#[cfg(not(target_os = "android"))]
pub fn get_or_create_local_key() -> Result<String, StorageError> {
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER)
        .map_err(|e| StorageError::KeyringError(e.to_string()))?;

    match entry.get_password() {
        Ok(key) => Ok(key),
        Err(keyring::Error::NoEntry) => {
            let key = generate_random_key();
            entry.set_password(&key)
                .map_err(|e| StorageError::KeyringError(e.to_string()))?;
            Ok(key)
        }
        Err(e) => Err(StorageError::KeyringError(e.to_string())),
    }
}

#[cfg(target_os = "android")]
pub fn get_or_create_local_key() -> Result<String, StorageError> {
    let data_dir = get_data_dir()?;
    let key_file = data_dir.join(".local_key");

    if key_file.exists() {
        let key = fs::read_to_string(&key_file)
            .map_err(|e| StorageError::IoError(e.to_string()))?;
        if key.len() == 128 {
            return Ok(key);
        }
    }

    let key = generate_random_key();
    fs::create_dir_all(&data_dir)
        .map_err(|e| StorageError::IoError(e.to_string()))?;
    fs::write(&key_file, &key)
        .map_err(|e| StorageError::IoError(e.to_string()))?;
    Ok(key)
}

fn generate_random_key() -> String {
    use rand::RngCore;
    use rand::rngs::OsRng;
    let mut key_bytes = [0u8; 64];
    OsRng.fill_bytes(&mut key_bytes);
    key_bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
```

Also gate the `keyring` import:

```rust
#[cfg(not(target_os = "android"))]
use keyring::Entry;
```

**Verify:** `npx vite build` succeeds (Rust changes verified by code review since Android NDK is not available in this environment).

---

### Task 5: Storage path fallback for Android

**Files:**
- Modify: `src-tauri/src/storage.rs`

The `directories` crate's `ProjectDirs` may not resolve correctly on Android. Add a cfg-gated fallback that uses Tauri's app data directory or a known Android-safe path.

**Replace `get_data_dir()` with cfg-gated versions:**

```rust
#[cfg(not(target_os = "android"))]
pub fn get_data_dir() -> Result<PathBuf, StorageError> {
    ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME)
        .map(|dirs| dirs.data_dir().to_path_buf())
        .ok_or(StorageError::NoDataDirectory)
}

#[cfg(target_os = "android")]
pub fn get_data_dir() -> Result<PathBuf, StorageError> {
    // On Android, use the app's internal data directory.
    // Tauri sets the HOME env var to the app's files directory.
    std::env::var("HOME")
        .map(PathBuf::from)
        .or_else(|_| std::env::var("TMPDIR").map(|t| PathBuf::from(t).join("data")))
        .map_err(|_| StorageError::NoDataDirectory)
}
```

Also gate the `directories` import:

```rust
#[cfg(not(target_os = "android"))]
use directories::ProjectDirs;
```

**Verify:** Code review for correctness (no Android compile available).

---

### Task 6: Mobile lifecycle — save on pause/visibility

**Files:**
- Modify: `src/App.svelte`

The existing `blur` and `visibilitychange` listeners already handle saving when the user leaves the app. On Android, the WebView fires `visibilitychange` when the app is backgrounded, and `pagehide` on termination. These are already covered by the existing handlers, but we should also listen for `pagehide` as a final save opportunity.

**In the `onMount` that sets up event listeners, add `pagehide`:**

Find:
```typescript
window.addEventListener('blur', handleVisibilityOrBlur);
window.addEventListener('visibilitychange', handleVisibilityOrBlur);
```

Replace with:
```typescript
window.addEventListener('blur', handleVisibilityOrBlur);
window.addEventListener('visibilitychange', handleVisibilityOrBlur);
window.addEventListener('pagehide', handleVisibilityOrBlur);
```

And in `onDestroy`, add the cleanup:

Find:
```typescript
window.removeEventListener('blur', handleVisibilityOrBlur);
window.removeEventListener('visibilitychange', handleVisibilityOrBlur);
```

Replace with:
```typescript
window.removeEventListener('blur', handleVisibilityOrBlur);
window.removeEventListener('visibilitychange', handleVisibilityOrBlur);
window.removeEventListener('pagehide', handleVisibilityOrBlur);
```

**Verify:** `npx vite build` — clean build.

---

### Task 7: Make file dialog async-compatible

**Files:**
- Modify: `src-tauri/src/main.rs`

The `blocking_save_file()` call in export commands blocks the thread. On Android, Tauri recommends async dialog APIs. The existing commands are already `async fn`, but they call `blocking_save_file()`. This works on desktop but may cause issues on Android. Wrap the blocking call in `tokio::task::spawn_blocking` so it doesn't block the async runtime.

**In `save_export_with_dialog`, replace:**

```rust
let file_path = app.dialog()
    .file()
    .set_file_name(&default_name)
    .add_filter("HTML Files", &["html", "htm"])
    .blocking_save_file();
```

**With:**

```rust
let dialog = app.dialog().clone();
let file_path = tokio::task::spawn_blocking(move || {
    dialog
        .file()
        .set_file_name(&default_name)
        .add_filter("HTML Files", &["html", "htm"])
        .blocking_save_file()
}).await.map_err(|e| e.to_string())?;
```

Apply the same change to `save_export_with_questions`.

**Note:** This requires checking that `app.dialog()` returns a cloneable handle. If it doesn't, keep the existing `blocking_save_file()` — Tauri's plugin handles the platform differences internally. This task is best verified during actual Android testing.

**Verify:** Code review for correctness.
