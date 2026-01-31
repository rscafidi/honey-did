<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  const dispatch = createEventDispatcher();

  let password = '';
  let error = '';
  let isVerifying = false;
  let showClearConfirm = false;
  let clearConfirmation = '';
  let clearError = '';

  let biometricAvailable = false;
  let biometricEnabled = false;
  let biometricError = '';

  onMount(async () => {
    try {
      const result = await invoke<{ available: boolean; enrolled: boolean }>('check_biometric_availability');
      biometricAvailable = result.available && result.enrolled;
      if (biometricAvailable) {
        biometricEnabled = await invoke<boolean>('get_biometric_enabled');
        if (biometricEnabled) {
          handleBiometricUnlock();
        }
      }
    } catch (_) {
      // Biometric not available
    }
  });

  async function handleBiometricUnlock() {
    biometricError = '';
    try {
      const decryptedPassword = await invoke<string>('authenticate_biometric');
      isVerifying = true;
      const valid = await invoke<boolean>('verify_app_password', { password: decryptedPassword });
      if (valid) {
        dispatch('unlock');
      } else {
        biometricError = 'Stored password is invalid. Please use your password.';
      }
    } catch (e: any) {
      const msg = `${e}`;
      if (msg.includes('key_invalidated')) {
        biometricError = 'Biometrics changed. Please use your password and re-enable fingerprint in Settings.';
        biometricEnabled = false;
        try {
          await invoke('set_biometric_enabled', { enabled: false });
          await invoke('clear_biometric_enrollment');
        } catch (_) {}
      } else if (msg.includes('user_cancelled')) {
        // User dismissed prompt — do nothing, password field is ready
      } else {
        biometricError = 'Biometric unlock failed. Please use your password.';
      }
    } finally {
      isVerifying = false;
    }
  }

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
    if (clearConfirmation.toUpperCase() !== 'DELETE ALL DATA') {
      clearError = 'Type DELETE ALL DATA to confirm';
      return;
    }

    clearError = '';

    try {
      await invoke('force_clear_all_data', { confirmation: clearConfirmation });
      dispatch('cleared');
    } catch (e) {
      clearError = `${e}`;
      clearConfirmation = '';
    }
  }
</script>

<div class="lock-screen">
  <div class="lock-container">
    <div class="logo-wrapper">
      <svg class="logo-icon" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
        <rect x="8" y="6" width="32" height="36" rx="2" fill="#F0EFEB" stroke="#DDE5B6" stroke-width="1.5"/>
        <ellipse cx="24" cy="6" rx="16" ry="3" fill="#DDE5B6"/>
        <ellipse cx="24" cy="6" rx="14" ry="2" fill="#F0EFEB"/>
        <ellipse cx="24" cy="42" rx="16" ry="3" fill="#DDE5B6"/>
        <ellipse cx="24" cy="42" rx="14" ry="2" fill="#F0EFEB"/>
        <text x="24" y="28" text-anchor="middle" font-family="Georgia, serif" font-style="italic" font-size="16" font-weight="600" fill="#283618">HD</text>
        <line x1="14" y1="34" x2="34" y2="34" stroke="#B7B7A4" stroke-width="1" stroke-linecap="round"/>
        <line x1="16" y1="37" x2="32" y2="37" stroke="#B7B7A4" stroke-width="0.75" stroke-linecap="round"/>
      </svg>
      <h1>Honey Did</h1>
    </div>
    <p class="subtitle">Enter your password to unlock</p>

    {#if !showClearConfirm}
      <div class="form">
        <!-- svelte-ignore a11y-autofocus -->
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
        {#if biometricAvailable && biometricEnabled}
          <button
            class="btn btn-biometric"
            on:click={handleBiometricUnlock}
            disabled={isVerifying}
          >
            <svg class="fingerprint-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M2 12C2 6.5 6.5 2 12 2a10 10 0 0 1 8 4"/>
              <path d="M5 19.5C5.5 18 6 15 6 12c0-3.5 2.5-6 6-6 3 0 5.5 2 6 5"/>
              <path d="M12 12v4c0 2.5-1 4-3 5.5"/>
              <path d="M9 12c0 2 .5 3.5 1.5 5"/>
              <path d="M15 12c0 4-1 6-3 8"/>
              <path d="M18 12a9 9 0 0 1-1.5 5c-.5 1-2 2.5-3.5 3.5"/>
              <path d="M22 12c0 4-1.5 7-4 9.5"/>
            </svg>
            Fingerprint Unlock
          </button>
        {/if}
        {#if biometricError}
          <p class="biometric-error">{biometricError}</p>
        {/if}
      </div>
      <button class="clear-link" on:click={() => (showClearConfirm = true)}>
        Forgot password? Clear all data
      </button>
    {:else}
      <div class="form">
        <p class="warning">This will permanently delete all your data. Type DELETE ALL DATA to confirm.</p>
        <!-- svelte-ignore a11y-autofocus -->
        <input
          type="text"
          bind:value={clearConfirmation}
          placeholder="Type DELETE ALL DATA"
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
    background: var(--bg-primary);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .lock-container {
    text-align: center;
    max-width: 320px;
    padding: 40px;
  }

  .logo-wrapper {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    margin-bottom: 8px;
  }

  .logo-icon {
    width: 48px;
    height: 48px;
  }

  h1 {
    margin: 0;
    color: var(--text-primary);
    font-weight: 600;
    font-size: 1.75rem;
  }

  .subtitle {
    color: var(--text-secondary);
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
    border: 2px solid var(--border-color);
    border-radius: 8px;
    text-align: center;
    background: var(--bg-secondary);
    color: var(--text-primary);
  }

  input:focus {
    outline: none;
    border-color: var(--accent-primary);
  }

  .error {
    color: var(--error-color);
    margin: 0;
    font-size: 0.9rem;
  }

  .warning {
    color: var(--warning-text);
    background: var(--warning-bg);
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

  .button-row {
    display: flex;
    gap: 12px;
  }

  .button-row .btn {
    flex: 1;
  }

  .btn-biometric {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .btn-biometric:hover:not(:disabled) {
    background: var(--border-color);
  }

  .btn-biometric:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .fingerprint-icon {
    width: 20px;
    height: 20px;
  }

  .biometric-error {
    color: var(--warning-text);
    background: var(--warning-bg);
    padding: 10px;
    border-radius: 8px;
    font-size: 0.85rem;
    margin: 0;
  }

  .clear-link {
    background: none;
    border: none;
    color: var(--text-secondary);
    font-size: 0.85rem;
    margin-top: 24px;
    cursor: pointer;
  }

  .clear-link:hover {
    color: var(--error-color);
    text-decoration: underline;
  }

  @media (max-width: 768px) {
    .lock-container {
      max-width: calc(100vw - 32px);
      padding: 24px;
    }
  }
</style>
