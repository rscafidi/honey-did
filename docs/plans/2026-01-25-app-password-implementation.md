# App Password Protection Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add app-level password protection with settings menu and data clearing options.

**Architecture:** Lock screen guards app when data exists, password setup on first data entry, settings modal with security options.

**Tech Stack:** Svelte 4, TypeScript, Tauri 2.0, Argon2id

---

## Task 1: Add Password Storage Commands to Rust Backend

**Files:**
- Modify: `src-tauri/src/storage.rs`
- Modify: `src-tauri/src/main.rs`

**Add to storage.rs:**

```rust
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// Hashes a password using Argon2id
pub fn hash_password(password: &str) -> Result<String, StorageError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| StorageError::KeyringError(format!("Password hashing failed: {}", e)))?;
    Ok(hash.to_string())
}

/// Verifies a password against a stored hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, StorageError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| StorageError::KeyringError(format!("Invalid hash format: {}", e)))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Saves the app password hash
pub fn save_password_hash(hash: &str) -> Result<(), StorageError> {
    let data_dir = get_data_dir()?;
    fs::create_dir_all(&data_dir)
        .map_err(|e| StorageError::IoError(e.to_string()))?;
    let file_path = data_dir.join("password.hash");
    fs::write(&file_path, hash)
        .map_err(|e| StorageError::IoError(e.to_string()))?;
    Ok(())
}

/// Loads the app password hash if it exists
pub fn load_password_hash() -> Result<Option<String>, StorageError> {
    let data_dir = get_data_dir()?;
    let file_path = data_dir.join("password.hash");
    if !file_path.exists() {
        return Ok(None);
    }
    let hash = fs::read_to_string(&file_path)
        .map_err(|e| StorageError::IoError(e.to_string()))?;
    Ok(Some(hash))
}

/// Deletes the password hash file
pub fn delete_password_hash() -> Result<(), StorageError> {
    let data_dir = get_data_dir()?;
    let file_path = data_dir.join("password.hash");
    if file_path.exists() {
        fs::remove_file(&file_path)
            .map_err(|e| StorageError::IoError(e.to_string()))?;
    }
    Ok(())
}

/// Deletes the document file
pub fn delete_document() -> Result<(), StorageError> {
    let data_dir = get_data_dir()?;
    let file_path = data_dir.join("document.encrypted");
    if file_path.exists() {
        fs::remove_file(&file_path)
            .map_err(|e| StorageError::IoError(e.to_string()))?;
    }
    Ok(())
}

/// Saves settings to a JSON file
pub fn save_settings(clear_on_exit: bool) -> Result<(), StorageError> {
    let data_dir = get_data_dir()?;
    fs::create_dir_all(&data_dir)
        .map_err(|e| StorageError::IoError(e.to_string()))?;
    let file_path = data_dir.join("settings.json");
    let json = format!(r#"{{"clear_on_exit":{}}}"#, clear_on_exit);
    fs::write(&file_path, json)
        .map_err(|e| StorageError::IoError(e.to_string()))?;
    Ok(())
}

/// Loads settings from JSON file
pub fn load_settings() -> Result<bool, StorageError> {
    let data_dir = get_data_dir()?;
    let file_path = data_dir.join("settings.json");
    if !file_path.exists() {
        return Ok(false);
    }
    let json = fs::read_to_string(&file_path)
        .map_err(|e| StorageError::IoError(e.to_string()))?;
    // Simple parsing - look for true/false after clear_on_exit
    Ok(json.contains("true"))
}

/// Deletes settings file
pub fn delete_settings() -> Result<(), StorageError> {
    let data_dir = get_data_dir()?;
    let file_path = data_dir.join("settings.json");
    if file_path.exists() {
        fs::remove_file(&file_path)
            .map_err(|e| StorageError::IoError(e.to_string()))?;
    }
    Ok(())
}
```

**Add Tauri commands to main.rs:**

```rust
#[tauri::command]
fn set_app_password(password: String) -> Result<(), String> {
    let hash = storage::hash_password(&password).map_err(|e| e.to_string())?;
    storage::save_password_hash(&hash).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn verify_app_password(password: String) -> Result<bool, String> {
    let hash = storage::load_password_hash()
        .map_err(|e| e.to_string())?
        .ok_or("No password set")?;
    storage::verify_password(&password, &hash).map_err(|e| e.to_string())
}

#[tauri::command]
fn has_app_password() -> Result<bool, String> {
    let hash = storage::load_password_hash().map_err(|e| e.to_string())?;
    Ok(hash.is_some())
}

#[tauri::command]
fn change_app_password(old_password: String, new_password: String) -> Result<(), String> {
    // Verify old password first
    let hash = storage::load_password_hash()
        .map_err(|e| e.to_string())?
        .ok_or("No password set")?;
    let valid = storage::verify_password(&old_password, &hash).map_err(|e| e.to_string())?;
    if !valid {
        return Err("Incorrect password".to_string());
    }
    // Set new password
    let new_hash = storage::hash_password(&new_password).map_err(|e| e.to_string())?;
    storage::save_password_hash(&new_hash).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn clear_all_data(state: State<AppState>, password: String) -> Result<(), String> {
    // Verify password first
    let hash = storage::load_password_hash()
        .map_err(|e| e.to_string())?
        .ok_or("No password set")?;
    let valid = storage::verify_password(&password, &hash).map_err(|e| e.to_string())?;
    if !valid {
        return Err("Incorrect password".to_string());
    }
    // Clear everything
    storage::delete_document().map_err(|e| e.to_string())?;
    storage::delete_password_hash().map_err(|e| e.to_string())?;
    storage::delete_settings().map_err(|e| e.to_string())?;
    // Reset in-memory state
    let mut doc = state.document.lock().map_err(|e| e.to_string())?;
    *doc = LegacyDocument::default();
    Ok(())
}

#[tauri::command]
fn get_clear_on_exit() -> Result<bool, String> {
    storage::load_settings().map_err(|e| e.to_string())
}

#[tauri::command]
fn set_clear_on_exit(enabled: bool) -> Result<(), String> {
    storage::save_settings(enabled).map_err(|e| e.to_string())
}

#[tauri::command]
fn clear_data_on_exit(state: State<AppState>) -> Result<(), String> {
    storage::delete_document().map_err(|e| e.to_string())?;
    storage::delete_password_hash().map_err(|e| e.to_string())?;
    storage::delete_settings().map_err(|e| e.to_string())?;
    let mut doc = state.document.lock().map_err(|e| e.to_string())?;
    *doc = LegacyDocument::default();
    Ok(())
}
```

Register all commands in invoke_handler.

---

## Task 2: Create LockScreen Component

**Files:**
- Create: `src/lib/components/LockScreen.svelte`

```svelte
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  const dispatch = createEventDispatcher();

  let password = '';
  let error = '';
  let isVerifying = false;
  let showClearConfirm = false;
  let clearPassword = '';
  let clearError = '';

  async function handleUnlock() {
    if (!password || password.length < 8) {
      error = 'Password must be at least 8 characters';
      return;
    }

    error = '';
    isVerifying = true;

    try {
      const valid = await invoke<boolean>('verify_app_password', { password });
      if (valid) {
        dispatch('unlock');
      } else {
        error = 'Incorrect password';
        password = '';
      }
    } catch (e) {
      error = `Error: ${e}`;
    } finally {
      isVerifying = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      handleUnlock();
    }
  }

  async function handleClearData() {
    if (!clearPassword) {
      clearError = 'Enter your password to confirm';
      return;
    }

    clearError = '';

    try {
      await invoke('clear_all_data', { password: clearPassword });
      dispatch('cleared');
    } catch (e) {
      clearError = `${e}`;
      clearPassword = '';
    }
  }
</script>

<div class="lock-screen">
  <div class="lock-container">
    <div class="lock-icon">🔒</div>
    <h1>honey-did</h1>
    <p class="subtitle">Enter your password to unlock</p>

    {#if !showClearConfirm}
      <div class="form">
        <input
          type="password"
          bind:value={password}
          on:keydown={handleKeydown}
          placeholder="Enter password"
          disabled={isVerifying}
          autofocus
        />
        {#if error}
          <p class="error">{error}</p>
        {/if}
        <button
          class="btn btn-primary"
          on:click={handleUnlock}
          disabled={isVerifying}
        >
          {isVerifying ? 'Verifying...' : 'Unlock'}
        </button>
      </div>
      <button class="clear-link" on:click={() => (showClearConfirm = true)}>
        Forgot password? Clear all data
      </button>
    {:else}
      <div class="form">
        <p class="warning">This will permanently delete all your data. Enter your password to confirm.</p>
        <input
          type="password"
          bind:value={clearPassword}
          placeholder="Enter password to confirm"
          autofocus
        />
        {#if clearError}
          <p class="error">{clearError}</p>
        {/if}
        <div class="button-row">
          <button class="btn btn-secondary" on:click={() => (showClearConfirm = false)}>
            Cancel
          </button>
          <button class="btn btn-danger" on:click={handleClearData}>
            Clear All Data
          </button>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .lock-screen {
    position: fixed;
    inset: 0;
    background: #f5f5f5;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .lock-container {
    text-align: center;
    max-width: 320px;
    padding: 40px;
  }

  .lock-icon {
    font-size: 4rem;
    margin-bottom: 16px;
  }

  h1 {
    margin: 0 0 8px 0;
    color: #333;
  }

  .subtitle {
    color: #666;
    margin: 0 0 24px 0;
  }

  .form {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  input {
    padding: 12px;
    font-size: 1rem;
    border: 2px solid #ddd;
    border-radius: 8px;
    text-align: center;
  }

  input:focus {
    outline: none;
    border-color: #1976d2;
  }

  .error {
    color: #dc3545;
    margin: 0;
    font-size: 0.9rem;
  }

  .warning {
    color: #856404;
    background: #fff3cd;
    padding: 12px;
    border-radius: 8px;
    font-size: 0.9rem;
    margin: 0;
  }

  .btn {
    padding: 12px 24px;
    font-size: 1rem;
    border: none;
    border-radius: 8px;
    cursor: pointer;
  }

  .btn-primary {
    background: #1976d2;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: #1565c0;
  }

  .btn-primary:disabled {
    background: #90caf9;
    cursor: not-allowed;
  }

  .btn-secondary {
    background: #e0e0e0;
    color: #333;
  }

  .btn-secondary:hover {
    background: #d0d0d0;
  }

  .btn-danger {
    background: #dc3545;
    color: white;
  }

  .btn-danger:hover {
    background: #c82333;
  }

  .button-row {
    display: flex;
    gap: 12px;
  }

  .button-row .btn {
    flex: 1;
  }

  .clear-link {
    background: none;
    border: none;
    color: #666;
    font-size: 0.85rem;
    margin-top: 24px;
    cursor: pointer;
  }

  .clear-link:hover {
    color: #dc3545;
    text-decoration: underline;
  }
</style>
```

---

## Task 3: Create SetPasswordModal Component

**Files:**
- Create: `src/lib/components/SetPasswordModal.svelte`

```svelte
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  export let isOpen = false;

  const dispatch = createEventDispatcher();

  let password = '';
  let confirmPassword = '';
  let error = '';
  let isSaving = false;

  $: passwordsMatch = password === confirmPassword;
  $: canSave = password.length >= 8 && passwordsMatch && !isSaving;

  async function handleSave() {
    if (!canSave) return;

    error = '';
    isSaving = true;

    try {
      await invoke('set_app_password', { password });
      dispatch('created');
      close();
    } catch (e) {
      error = `Failed to set password: ${e}`;
    } finally {
      isSaving = false;
    }
  }

  function close() {
    password = '';
    confirmPassword = '';
    error = '';
    dispatch('cancel');
  }
</script>

{#if isOpen}
  <div class="overlay" role="presentation">
    <div class="dialog" role="dialog" aria-modal="true" aria-labelledby="setpw-title">
      <h2 id="setpw-title">Set App Password</h2>
      <p class="description">
        Create a password to protect your data. You'll need this password each time you open the app.
      </p>

      <div class="form">
        <div class="field">
          <label for="new-password">Password</label>
          <input
            id="new-password"
            type="password"
            bind:value={password}
            placeholder="At least 8 characters"
          />
          {#if password && password.length < 8}
            <span class="hint error-hint">Password must be at least 8 characters</span>
          {/if}
        </div>

        <div class="field">
          <label for="confirm-password">Confirm Password</label>
          <input
            id="confirm-password"
            type="password"
            bind:value={confirmPassword}
            placeholder="Re-enter password"
          />
          {#if confirmPassword && !passwordsMatch}
            <span class="hint error-hint">Passwords don't match</span>
          {/if}
        </div>

        {#if error}
          <p class="error-message">{error}</p>
        {/if}
      </div>

      <div class="actions">
        <button class="btn btn-secondary" on:click={close}>Cancel</button>
        <button class="btn btn-primary" on:click={handleSave} disabled={!canSave}>
          {isSaving ? 'Saving...' : 'Set Password'}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .dialog {
    background: white;
    border-radius: 12px;
    padding: 24px;
    width: 100%;
    max-width: 400px;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  }

  h2 {
    margin: 0 0 12px 0;
    color: #333;
  }

  .description {
    color: #666;
    margin: 0 0 20px 0;
    font-size: 0.95rem;
    line-height: 1.5;
  }

  .form {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .field label {
    display: block;
    margin-bottom: 6px;
    font-weight: 500;
    color: #333;
  }

  .field input {
    width: 100%;
    padding: 10px 12px;
    border: 2px solid #ddd;
    border-radius: 6px;
    font-size: 1rem;
    box-sizing: border-box;
  }

  .field input:focus {
    outline: none;
    border-color: #1976d2;
  }

  .hint {
    display: block;
    font-size: 0.85rem;
    margin-top: 4px;
  }

  .error-hint {
    color: #dc3545;
  }

  .error-message {
    color: #dc3545;
    background: #f8d7da;
    padding: 10px 12px;
    border-radius: 6px;
    margin: 0;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    margin-top: 24px;
  }

  .btn {
    padding: 10px 20px;
    border: none;
    border-radius: 6px;
    font-size: 1rem;
    cursor: pointer;
  }

  .btn-primary {
    background: #1976d2;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: #1565c0;
  }

  .btn-primary:disabled {
    background: #90caf9;
    cursor: not-allowed;
  }

  .btn-secondary {
    background: #e0e0e0;
    color: #333;
  }

  .btn-secondary:hover {
    background: #d0d0d0;
  }
</style>
```

---

## Task 4: Create SettingsModal Component

**Files:**
- Create: `src/lib/components/SettingsModal.svelte`

```svelte
<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  export let isOpen = false;

  const dispatch = createEventDispatcher();

  let clearOnExit = false;
  let showChangePassword = false;
  let showClearConfirm = false;

  // Change password fields
  let oldPassword = '';
  let newPassword = '';
  let confirmNewPassword = '';
  let changeError = '';
  let isChanging = false;

  // Clear data fields
  let clearPassword = '';
  let clearError = '';

  $: newPasswordsMatch = newPassword === confirmNewPassword;
  $: canChangePassword = oldPassword && newPassword.length >= 8 && newPasswordsMatch && !isChanging;

  onMount(async () => {
    try {
      clearOnExit = await invoke<boolean>('get_clear_on_exit');
    } catch (e) {
      console.error('Failed to load settings:', e);
    }
  });

  async function handleClearOnExitToggle() {
    clearOnExit = !clearOnExit;
    try {
      await invoke('set_clear_on_exit', { enabled: clearOnExit });
    } catch (e) {
      console.error('Failed to save setting:', e);
      clearOnExit = !clearOnExit; // Revert
    }
  }

  async function handleChangePassword() {
    if (!canChangePassword) return;

    changeError = '';
    isChanging = true;

    try {
      await invoke('change_app_password', {
        oldPassword,
        newPassword
      });
      showChangePassword = false;
      oldPassword = '';
      newPassword = '';
      confirmNewPassword = '';
    } catch (e) {
      changeError = `${e}`;
    } finally {
      isChanging = false;
    }
  }

  async function handleClearData() {
    if (!clearPassword) {
      clearError = 'Enter your password to confirm';
      return;
    }

    clearError = '';

    try {
      await invoke('clear_all_data', { password: clearPassword });
      dispatch('cleared');
      close();
    } catch (e) {
      clearError = `${e}`;
      clearPassword = '';
    }
  }

  function close() {
    showChangePassword = false;
    showClearConfirm = false;
    oldPassword = '';
    newPassword = '';
    confirmNewPassword = '';
    clearPassword = '';
    changeError = '';
    clearError = '';
    dispatch('close');
  }
</script>

{#if isOpen}
  <div class="overlay" on:click={close} on:keydown={(e) => e.key === 'Escape' && close()} role="presentation">
    <div class="dialog" role="dialog" aria-modal="true" on:click|stopPropagation on:keydown|stopPropagation>
      <h2>Settings</h2>

      {#if !showChangePassword && !showClearConfirm}
        <div class="settings-section">
          <h3>Security</h3>

          <button class="setting-button" on:click={() => (showChangePassword = true)}>
            <span class="setting-label">Change Password</span>
            <span class="setting-arrow">→</span>
          </button>

          <label class="setting-toggle">
            <span class="setting-label">
              Clear on Exit
              <span class="setting-hint">Delete all data when app closes</span>
            </span>
            <input type="checkbox" checked={clearOnExit} on:change={handleClearOnExitToggle} />
          </label>

          {#if clearOnExit}
            <p class="warning">Data will be deleted when you close the app.</p>
          {/if}

          <button class="setting-button danger" on:click={() => (showClearConfirm = true)}>
            <span class="setting-label">Clear All Data</span>
            <span class="setting-arrow">→</span>
          </button>
        </div>

        <div class="actions">
          <button class="btn btn-secondary" on:click={close}>Close</button>
        </div>

      {:else if showChangePassword}
        <div class="sub-section">
          <div class="field">
            <label for="old-pw">Current Password</label>
            <input id="old-pw" type="password" bind:value={oldPassword} />
          </div>
          <div class="field">
            <label for="new-pw">New Password</label>
            <input id="new-pw" type="password" bind:value={newPassword} placeholder="At least 8 characters" />
          </div>
          <div class="field">
            <label for="confirm-new-pw">Confirm New Password</label>
            <input id="confirm-new-pw" type="password" bind:value={confirmNewPassword} />
            {#if confirmNewPassword && !newPasswordsMatch}
              <span class="error-hint">Passwords don't match</span>
            {/if}
          </div>
          {#if changeError}
            <p class="error-message">{changeError}</p>
          {/if}
          <div class="actions">
            <button class="btn btn-secondary" on:click={() => (showChangePassword = false)}>Cancel</button>
            <button class="btn btn-primary" on:click={handleChangePassword} disabled={!canChangePassword}>
              {isChanging ? 'Changing...' : 'Change Password'}
            </button>
          </div>
        </div>

      {:else if showClearConfirm}
        <div class="sub-section">
          <p class="warning">This will permanently delete all your data. This cannot be undone.</p>
          <div class="field">
            <label for="clear-pw">Enter Password to Confirm</label>
            <input id="clear-pw" type="password" bind:value={clearPassword} />
          </div>
          {#if clearError}
            <p class="error-message">{clearError}</p>
          {/if}
          <div class="actions">
            <button class="btn btn-secondary" on:click={() => (showClearConfirm = false)}>Cancel</button>
            <button class="btn btn-danger" on:click={handleClearData}>Clear All Data</button>
          </div>
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .dialog {
    background: white;
    border-radius: 12px;
    padding: 24px;
    width: 100%;
    max-width: 420px;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  }

  h2 {
    margin: 0 0 20px 0;
    color: #333;
  }

  h3 {
    margin: 0 0 16px 0;
    color: #666;
    font-size: 0.9rem;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .settings-section {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .setting-button {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    padding: 12px 16px;
    background: #f5f5f5;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    text-align: left;
  }

  .setting-button:hover {
    background: #eeeeee;
  }

  .setting-button.danger {
    color: #dc3545;
  }

  .setting-button.danger:hover {
    background: #f8d7da;
  }

  .setting-toggle {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background: #f5f5f5;
    border-radius: 8px;
    cursor: pointer;
  }

  .setting-label {
    font-size: 0.95rem;
  }

  .setting-hint {
    display: block;
    font-size: 0.8rem;
    color: #666;
    font-weight: normal;
  }

  .setting-arrow {
    color: #999;
  }

  .setting-toggle input {
    width: 20px;
    height: 20px;
  }

  .warning {
    color: #856404;
    background: #fff3cd;
    padding: 12px;
    border-radius: 8px;
    font-size: 0.9rem;
    margin: 8px 0;
  }

  .sub-section {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .field label {
    display: block;
    margin-bottom: 6px;
    font-weight: 500;
    color: #333;
  }

  .field input {
    width: 100%;
    padding: 10px 12px;
    border: 2px solid #ddd;
    border-radius: 6px;
    font-size: 1rem;
    box-sizing: border-box;
  }

  .field input:focus {
    outline: none;
    border-color: #1976d2;
  }

  .error-hint {
    color: #dc3545;
    font-size: 0.85rem;
    margin-top: 4px;
  }

  .error-message {
    color: #dc3545;
    background: #f8d7da;
    padding: 10px 12px;
    border-radius: 6px;
    margin: 0;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    margin-top: 24px;
  }

  .btn {
    padding: 10px 20px;
    border: none;
    border-radius: 6px;
    font-size: 1rem;
    cursor: pointer;
  }

  .btn-primary {
    background: #1976d2;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: #1565c0;
  }

  .btn-primary:disabled {
    background: #90caf9;
    cursor: not-allowed;
  }

  .btn-secondary {
    background: #e0e0e0;
    color: #333;
  }

  .btn-secondary:hover {
    background: #d0d0d0;
  }

  .btn-danger {
    background: #dc3545;
    color: white;
  }

  .btn-danger:hover {
    background: #c82333;
  }
</style>
```

---

## Task 5: Integrate Password Protection into App.svelte

**Files:**
- Modify: `src/App.svelte`

**Changes:**
1. Import new components (LockScreen, SetPasswordModal, SettingsModal)
2. Add state: `isLocked`, `hasPassword`, `showSetPasswordModal`, `showSettings`, `pendingAction`
3. On mount: check `has_app_password()`, if true set `isLocked = true`
4. Add LockScreen when locked
5. Add Settings button with gear icon to sidebar
6. Add SetPasswordModal
7. Add SettingsModal
8. Create `requirePassword()` function that shows SetPasswordModal if no password

---

## Task 6: Intercept Data Entry to Require Password

**Files:**
- Create: `src/lib/stores/appState.ts`
- Modify: `src/lib/sections/FinancialSection.svelte` (as example, then apply to all sections)

**appState.ts:**
```typescript
import { writable } from 'svelte/store';

export const appState = writable({
  hasPassword: false,
  pendingAction: null as (() => void) | null,
  showSetPasswordModal: false
});

export function requirePassword(action: () => void) {
  appState.update(state => {
    if (state.hasPassword) {
      action();
      return state;
    } else {
      return {
        ...state,
        pendingAction: action,
        showSetPasswordModal: true
      };
    }
  });
}
```

Wrap all add/update functions in sections with `requirePassword()`.

---

## Task 7: Handle Clear on Exit

**Files:**
- Modify: `src/App.svelte`

Add window close handler using Tauri's event system to check `clearOnExit` setting and call `clear_data_on_exit` if enabled.

---

## Task 8: Build and Test

Run `npm run build` to verify frontend compiles.

---
