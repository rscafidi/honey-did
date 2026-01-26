<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { document } from '../stores/document';

  export let isOpen = false;

  const dispatch = createEventDispatcher();

  let passphrase = '';
  let confirmPassphrase = '';
  let includePrint = false;
  let includeWelcomeScreen = true;
  let isExporting = false;
  let error = '';

  $: welcomeScreenAvailable = $document?.welcome_screen?.enabled &&
                              ($document?.welcome_screen?.slides?.length || 0) > 0;
  $: slideCount = $document?.welcome_screen?.slides?.length || 0;

  $: passphraseStrength = calculateStrength(passphrase);
  $: passphrasesMatch = passphrase === confirmPassphrase;
  $: canExport = passphrase.length >= 8 && passphrasesMatch && !isExporting;

  function calculateStrength(pass: string): { score: number; label: string; color: string } {
    if (!pass) return { score: 0, label: '', color: '#ddd' };
    let score = 0;
    if (pass.length >= 8) score += 1;
    if (pass.length >= 12) score += 1;
    if (pass.length >= 16) score += 1;
    if (/[a-z]/.test(pass) && /[A-Z]/.test(pass)) score += 1;
    if (/\d/.test(pass)) score += 1;
    if (/[^a-zA-Z0-9]/.test(pass)) score += 1;
    if (pass.includes('-') && pass.split('-').length >= 3) score += 2; // Passphrase bonus

    if (score <= 2) return { score, label: 'Weak', color: '#dc3545' };
    if (score <= 4) return { score, label: 'Fair', color: '#ffc107' };
    if (score <= 6) return { score, label: 'Good', color: '#28a745' };
    return { score, label: 'Strong', color: '#1976d2' };
  }

  async function generatePassphrase() {
    try {
      passphrase = await invoke<string>('generate_passphrase');
      confirmPassphrase = passphrase;
    } catch (e) {
      error = `Failed to generate passphrase: ${e}`;
    }
  }

  async function handleExport() {
    if (!canExport) return;

    error = '';
    isExporting = true;

    try {
      // Show native save dialog and save the encrypted file
      const filePath = await invoke<string | null>('save_export_with_dialog', {
        passphrase,
        includeWelcomeScreen: welcomeScreenAvailable && includeWelcomeScreen
      });

      if (filePath === null) {
        // User cancelled the dialog
        isExporting = false;
        return;
      }

      if (includePrint) {
        // Get unencrypted print-friendly HTML
        const printHtml = await invoke<string>('get_print_html');

        // Open print dialog - create a hidden iframe to print
        const printFrame = document.createElement('iframe');
        printFrame.style.display = 'none';
        document.body.appendChild(printFrame);
        printFrame.contentDocument?.write(printHtml);
        printFrame.contentDocument?.close();
        printFrame.contentWindow?.print();
        document.body.removeChild(printFrame);
      }

      dispatch('exported', { filePath });
      close();
    } catch (e) {
      error = `Export failed: ${e}`;
    } finally {
      isExporting = false;
    }
  }

  function close() {
    passphrase = '';
    confirmPassphrase = '';
    includePrint = false;
    includeWelcomeScreen = true;
    error = '';
    dispatch('close');
  }
</script>

{#if isOpen}
  <div class="overlay" on:keydown={(e) => e.key === 'Escape' && close()} role="presentation">
    <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
    <div class="dialog" role="dialog" aria-modal="true" aria-labelledby="export-dialog-title" on:click|stopPropagation on:keydown|stopPropagation>
      <h2 id="export-dialog-title">Create Your Secure File</h2>

      <div class="form">
        <div class="field">
          <label for="passphrase">Choose a passphrase</label>
          <div class="passphrase-input">
            <input
              id="passphrase"
              type="text"
              bind:value={passphrase}
              placeholder="Enter a memorable passphrase"
            />
            <button type="button" class="generate-btn" on:click={generatePassphrase}>
              Generate
            </button>
          </div>
          {#if passphrase}
            <div class="strength-meter">
              <div class="strength-bar" style="width: {passphraseStrength.score * 12.5}%; background: {passphraseStrength.color}"></div>
            </div>
            <span class="strength-label" style="color: {passphraseStrength.color}">{passphraseStrength.label}</span>
          {/if}
        </div>

        <div class="field">
          <label for="confirm-passphrase">Confirm passphrase</label>
          <input
            id="confirm-passphrase"
            type="password"
            bind:value={confirmPassphrase}
            placeholder="Re-enter passphrase"
          />
          {#if confirmPassphrase && !passphrasesMatch}
            <span class="error-text">Passphrases don't match</span>
          {/if}
        </div>

        <label class="checkbox-field">
          <input type="checkbox" bind:checked={includePrint} />
          <span>Also print a physical copy</span>
        </label>

        {#if includePrint}
          <p class="warning">
            Printed copies can be found by anyone. Store securely.
          </p>
        {/if}

        {#if welcomeScreenAvailable}
          <label class="checkbox-field">
            <input type="checkbox" bind:checked={includeWelcomeScreen} />
            <span>Include welcome screen ({slideCount} slide{slideCount === 1 ? '' : 's'})</span>
          </label>
        {/if}

        {#if error}
          <p class="error-message">{error}</p>
        {/if}
      </div>

      <div class="actions">
        <button type="button" class="btn-secondary" on:click={close}>Cancel</button>
        <button
          type="button"
          class="btn-primary"
          on:click={handleExport}
          disabled={!canExport}
        >
          {isExporting ? 'Exporting...' : 'Export File'}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(40, 54, 24, 0.5);
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
    max-width: 450px;
    box-shadow: 0 20px 60px rgba(40, 54, 24, 0.3);
  }

  h2 {
    margin: 0 0 20px 0;
    color: #283618;
    font-weight: 600;
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
    color: #283618;
  }

  .field input {
    width: 100%;
    padding: 10px 12px;
    border: 2px solid #D4D4D4;
    border-radius: 6px;
    font-size: 1rem;
    box-sizing: border-box;
  }

  .field input:focus {
    outline: none;
    border-color: #283618;
  }

  .passphrase-input {
    display: flex;
    gap: 8px;
  }

  .passphrase-input input {
    flex: 1;
  }

  .generate-btn {
    padding: 10px 16px;
    background: #DDE5B6;
    border: none;
    border-radius: 6px;
    color: #283618;
    cursor: pointer;
    white-space: nowrap;
    font-weight: 500;
    transition: background 0.15s ease;
  }

  .generate-btn:hover {
    background: #ADC178;
  }

  .strength-meter {
    height: 4px;
    background: #D4D4D4;
    border-radius: 2px;
    margin-top: 8px;
    overflow: hidden;
  }

  .strength-bar {
    height: 100%;
    transition: width 0.3s, background 0.3s;
  }

  .strength-label {
    font-size: 0.85rem;
    margin-top: 4px;
    display: block;
  }

  .error-text {
    color: #9B2C2C;
    font-size: 0.85rem;
    margin-top: 4px;
  }

  .checkbox-field {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    color: #283618;
  }

  .checkbox-field input {
    width: 18px;
    height: 18px;
  }

  .warning {
    padding: 10px 12px;
    background: #FEFCBF;
    border-radius: 6px;
    font-size: 0.9rem;
    color: #744210;
    margin: 0;
  }

  .error-message {
    color: #9B2C2C;
    background: #FED7D7;
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

  .btn-primary, .btn-secondary {
    padding: 10px 20px;
    border: none;
    border-radius: 6px;
    font-size: 1rem;
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
</style>
