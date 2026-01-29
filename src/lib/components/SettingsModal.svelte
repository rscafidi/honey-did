<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { themePreference, type ThemePreference } from '../stores/theme';

  // @ts-ignore - injected by Vite define config
  const appVersion: string = __APP_VERSION__;

  export let isOpen = false;

  const dispatch = createEventDispatcher();

  const themeOptions: { value: ThemePreference; label: string; icon: string }[] = [
    { value: 'auto', label: 'Auto', icon: '💻' },
    { value: 'light', label: 'Light', icon: '☀️' },
    { value: 'dark', label: 'Dark', icon: '🌙' },
  ];

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
          <h3>Appearance</h3>
          <div class="theme-selector">
            {#each themeOptions as option}
              <button
                class="theme-option"
                class:active={$themePreference === option.value}
                on:click={() => themePreference.set(option.value)}
              >
                <span class="theme-icon">{option.icon}</span>
                <span class="theme-label">{option.label}</span>
              </button>
            {/each}
          </div>
        </div>

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

        <div class="settings-section about-section">
          <h3>About</h3>
          <div class="about-info">
            <div>
              <span class="about-label">Honey Did</span>
              <span class="about-version">v{appVersion}</span>
            </div>
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <span class="about-link" on:click={() => invoke('open_external_url', { url: 'https://github.com/rscafidi/honey-did' })}>github.com/rscafidi/honey-did</span>
          </div>
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
    background: var(--bg-secondary);
    border-radius: 12px;
    padding: 24px;
    width: 100%;
    max-width: 420px;
    box-shadow: var(--card-shadow);
  }

  h2 {
    margin: 0 0 20px 0;
    color: var(--text-primary);
    font-weight: 600;
  }

  h3 {
    margin: 0 0 16px 0;
    color: var(--text-secondary);
    font-size: 0.9rem;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .settings-section {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-bottom: 24px;
  }

  .theme-selector {
    display: flex;
    gap: 8px;
  }

  .theme-option {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    padding: 12px 8px;
    background: var(--bg-tertiary);
    border: 2px solid transparent;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .theme-option:hover {
    background: var(--bg-primary);
  }

  .theme-option.active {
    border-color: var(--accent-primary);
    background: var(--accent-light);
  }

  .theme-icon {
    font-size: 1.5rem;
  }

  .theme-label {
    font-size: 0.85rem;
    color: var(--text-primary);
    font-weight: 500;
  }

  .setting-button {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    padding: 12px 16px;
    background: var(--bg-tertiary);
    border: none;
    border-radius: 8px;
    cursor: pointer;
    text-align: left;
    color: var(--text-primary);
    transition: background 0.15s ease;
  }

  .setting-button:hover {
    background: var(--bg-primary);
  }

  .setting-button.danger {
    color: var(--error-color);
  }

  .setting-button.danger:hover {
    background: rgba(155, 44, 44, 0.1);
  }

  .setting-toggle {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background: var(--bg-tertiary);
    border-radius: 8px;
    cursor: pointer;
    color: var(--text-primary);
  }

  .setting-label {
    font-size: 0.95rem;
  }

  .setting-hint {
    display: block;
    font-size: 0.8rem;
    color: var(--text-secondary);
    font-weight: normal;
  }

  .setting-arrow {
    color: var(--text-muted);
  }

  .setting-toggle input {
    width: 20px;
    height: 20px;
  }

  .warning {
    color: var(--warning-text);
    background: var(--warning-bg);
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
    color: var(--text-primary);
  }

  .field input {
    width: 100%;
    padding: 10px 12px;
    border: 2px solid var(--border-color);
    border-radius: 6px;
    font-size: 1rem;
    box-sizing: border-box;
    background: var(--bg-secondary);
    color: var(--text-primary);
  }

  .field input:focus {
    outline: none;
    border-color: var(--accent-primary);
  }

  .error-hint {
    color: var(--error-color);
    font-size: 0.85rem;
    margin-top: 4px;
    display: block;
  }

  .error-message {
    color: var(--error-color);
    background: rgba(155, 44, 44, 0.1);
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
    font-weight: 500;
    transition: all 0.15s ease;
  }

  .btn-primary {
    background: var(--accent-primary);
    color: var(--bg-secondary);
  }

  .btn-primary:hover:not(:disabled) {
    opacity: 0.9;
  }

  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-secondary {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .btn-secondary:hover {
    background: var(--border-color);
  }

  .btn-danger {
    background: var(--error-color);
    color: white;
  }

  .btn-danger:hover {
    opacity: 0.9;
  }

  .about-section {
    margin-bottom: 0;
  }

  .about-info {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 12px 16px;
    background: var(--bg-tertiary);
    border-radius: 8px;
  }

  .about-label {
    font-weight: 500;
    color: var(--text-primary);
  }

  .about-version {
    color: var(--text-secondary);
    font-size: 0.9rem;
    margin-left: 6px;
  }

  .about-link {
    color: var(--accent-secondary);
    font-size: 0.85rem;
    cursor: pointer;
  }

  .about-link:hover {
    text-decoration: underline;
  }
</style>
