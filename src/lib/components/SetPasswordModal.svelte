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
