<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  export let isOpen = false;

  const dispatch = createEventDispatcher();

  let passphrase = '';
  let fileName = '';
  let isImporting = false;
  let error = '';
  let fileContent = '';

  $: canImport = passphrase.length >= 1 && fileName.trim() && !isImporting;

  async function handleFileSelect(event: Event) {
    const input = event.target as HTMLInputElement;
    if (input.files && input.files.length > 0) {
      const file = input.files[0];
      fileName = file.name;

      // Read file content using FileReader
      const reader = new FileReader();
      reader.onload = (e) => {
        fileContent = e.target?.result as string || '';
      };
      reader.onerror = () => {
        error = 'Failed to read file';
      };
      reader.readAsText(file);
    }
  }

  async function handleImport() {
    if (!canImport) return;

    error = '';
    isImporting = true;

    try {
      // Import the document from the HTML file
      const imported = await invoke('import_file', {
        encryptedHtml: fileContent,
        passphrase
      });

      // Merge the imported document into the current state
      await invoke('merge_document', { imported });

      dispatch('imported', { fileName });
      close();
    } catch (e) {
      if (String(e).includes('Decryption failed')) {
        error = 'Incorrect passphrase. Please try again.';
      } else {
        error = `Import failed: ${e}`;
      }
    } finally {
      isImporting = false;
    }
  }

  function close() {
    passphrase = '';
    fileName = '';
    fileContent = '';
    error = '';
    dispatch('close');
  }
</script>

{#if isOpen}
  <div class="overlay" on:keydown={(e) => e.key === 'Escape' && close()} role="presentation">
    <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
    <div class="dialog" role="dialog" aria-modal="true" aria-labelledby="import-dialog-title" on:click|stopPropagation on:keydown|stopPropagation>
      <h2 id="import-dialog-title">Import Legacy Document</h2>

      <div class="form">
        <div class="field">
          <label for="import-file">Select HTML file</label>
          <input
            id="import-file"
            type="file"
            accept=".html,.htm"
            on:change={handleFileSelect}
          />
          {#if fileName}
            <span class="file-name">{fileName}</span>
          {/if}
        </div>

        <div class="field">
          <label for="import-passphrase">Enter passphrase</label>
          <input
            id="import-passphrase"
            type="password"
            bind:value={passphrase}
            placeholder="Enter the passphrase used to encrypt the file"
          />
        </div>

        <div class="warning">
          <strong>Note:</strong> Importing will replace your current document with the imported data.
          Make sure to export your current document first if you want to keep it.
        </div>

        {#if error}
          <p class="error-message">{error}</p>
        {/if}
      </div>

      <div class="actions">
        <button type="button" class="btn-secondary" on:click={close}>Cancel</button>
        <button
          type="button"
          class="btn-primary"
          on:click={handleImport}
          disabled={!canImport}
        >
          {isImporting ? 'Importing...' : 'Import'}
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
    background: var(--bg-secondary);
    border-radius: 12px;
    padding: 24px;
    width: 100%;
    max-width: 450px;
    box-shadow: var(--card-shadow);
  }

  h2 {
    margin: 0 0 20px 0;
    color: var(--text-primary);
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
    color: var(--text-primary);
  }

  .field input[type="password"] {
    width: 100%;
    padding: 10px 12px;
    border: 2px solid var(--border-color);
    border-radius: 6px;
    font-size: 1rem;
    box-sizing: border-box;
    background: var(--bg-secondary);
    color: var(--text-primary);
  }

  .field input[type="file"] {
    width: 100%;
    padding: 10px 0;
    color: var(--text-primary);
  }

  .field input:focus {
    outline: none;
    border-color: var(--accent-primary);
  }

  .file-name {
    display: block;
    font-size: 0.9rem;
    color: var(--text-primary);
    font-weight: 500;
    margin-top: 4px;
  }

  .warning {
    padding: 12px;
    background: var(--warning-bg);
    border-radius: 6px;
    font-size: 0.9rem;
    color: var(--warning-text);
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
</style>
