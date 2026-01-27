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
    <div class="lock-icon">🔒</div>
    <h1>Honey Did</h1>
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
    background: #F0EFEB;
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
    color: #283618;
    font-weight: 600;
  }

  .subtitle {
    color: #606060;
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
    border: 2px solid #D4D4D4;
    border-radius: 8px;
    text-align: center;
    background: white;
  }

  input:focus {
    outline: none;
    border-color: #283618;
  }

  .error {
    color: #9B2C2C;
    margin: 0;
    font-size: 0.9rem;
  }

  .warning {
    color: #744210;
    background: #FEFCBF;
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
    background: #283618;
    color: #F0EFEB;
  }

  .btn-primary:hover:not(:disabled) {
    background: #1f2a12;
  }

  .btn-primary:disabled {
    background: #B7B7A4;
    cursor: not-allowed;
  }

  .btn-secondary {
    background: #D4D4D4;
    color: #283618;
  }

  .btn-secondary:hover {
    background: #B7B7A4;
  }

  .btn-danger {
    background: #9B2C2C;
    color: white;
  }

  .btn-danger:hover {
    background: #822727;
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
    color: #606060;
    font-size: 0.85rem;
    margin-top: 24px;
    cursor: pointer;
  }

  .clear-link:hover {
    color: #9B2C2C;
    text-decoration: underline;
  }
</style>
