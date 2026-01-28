<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  const dispatch = createEventDispatcher();

  let password = '';
  let error = '';
  let isVerifying = false;
  let showClearConfirm = false;
  let clearConfirmation = '';
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
        <p class="warning">This will permanently delete all your data. Type DELETE ALL DATA to confirm.</p>
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
</style>
