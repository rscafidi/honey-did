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
