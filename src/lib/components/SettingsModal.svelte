<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  export let isOpen = false;

  const dispatch = createEventDispatcher();

  let clearOnExit = false;
  let showChangePassword = false;
  let showClearConfirm = false;
  let hasPassword = false;

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

  $: if (isOpen) {
    loadSettings();
  }

  async function loadSettings() {
    try {
      clearOnExit = await invoke<boolean>('get_clear_on_exit');
      hasPassword = await invoke<boolean>('has_app_password');
    } catch (e) {
      console.error('Failed to load settings:', e);
    }
  }

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
    // If password is set, require it
    if (hasPassword && !clearPassword) {
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
    <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
    <div class="dialog" role="dialog" aria-modal="true" on:click|stopPropagation on:keydown|stopPropagation>
      <h2>Settings</h2>

      {#if !showChangePassword && !showClearConfirm}
        <div class="settings-section">
          <h3>Security</h3>

          {#if hasPassword}
            <button class="setting-button" on:click={() => (showChangePassword = true)}>
              <span class="setting-label">Change Password</span>
              <span class="setting-arrow">→</span>
            </button>
          {/if}

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
          {#if hasPassword}
            <div class="field">
              <label for="clear-pw">Enter Password to Confirm</label>
              <input id="clear-pw" type="password" bind:value={clearPassword} />
            </div>
          {/if}
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
    display: block;
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
