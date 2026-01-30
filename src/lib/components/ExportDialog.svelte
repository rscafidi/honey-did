<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { save } from '@tauri-apps/plugin-dialog';
  import { writeTextFile } from '@tauri-apps/plugin-fs';
  import { document as documentStore, isDocumentEmpty } from '../stores/document';

  function isMobile(): boolean {
    // @ts-ignore
    return window.__TAURI_INTERNALS__?.metadata?.currentWebview?.label != null &&
           /android|iphone|ipad/i.test(navigator.userAgent);
  }

  async function saveHtmlFile(html: string): Promise<string | null> {
    const date = new Date().toISOString().split('T')[0];
    const fileName = `honey-did-${date}.html`;

    if (isMobile()) {
      // On Android: write to app cache, then open the share sheet so the
      // user can save to Downloads, email, Drive, etc.
      const savedPath = await invoke<string>('save_html_to_downloads', {
        html,
        fileName,
      });
      await invoke('share_file', {
        filePath: savedPath,
        mimeType: 'text/html',
      });
      return savedPath;
    } else {
      // Desktop: use native dialog + writeTextFile
      const filePath = await save({
        defaultPath: fileName,
        filters: [{ name: 'HTML Files', extensions: ['html', 'htm'] }],
      });
      if (!filePath) return null;
      await writeTextFile(filePath, html);
      return filePath;
    }
  }

  export let isOpen = false;

  const dispatch = createEventDispatcher();

  let passphrase = '';
  let confirmPassphrase = '';
  let includePrint = false;
  let includeWelcomeScreen = true;
  let isExporting = false;
  let error = '';
  let successMessage = '';

  // Question-based unlock detection
  $: questionSlides = $documentStore?.welcome_screen?.slides?.filter(s => s.type === 'question') || [];
  $: messageSlides = $documentStore?.welcome_screen?.slides?.filter(s => s.type === 'message') || [];
  $: questionCount = questionSlides.length;
  $: hasValidQuestionConfig = $documentStore?.welcome_screen?.enabled && questionCount >= 2 && questionCount <= 5;
  $: hasInvalidQuestionConfig = $documentStore?.welcome_screen?.enabled && questionCount === 1;

  // Legacy welcome screen (message-only slides)
  $: legacyWelcomeAvailable = $documentStore?.welcome_screen?.enabled &&
                              ($documentStore?.welcome_screen?.slides?.length || 0) > 0 &&
                              !hasValidQuestionConfig;
  $: slideCount = $documentStore?.welcome_screen?.slides?.length || 0;
  $: isEmpty = isDocumentEmpty($documentStore);

  $: passphraseStrength = calculateStrength(passphrase);
  $: passphrasesMatch = passphrase === confirmPassphrase;
  $: canExportPassphrase = passphrase.length >= 8 && passphrasesMatch && !isExporting;
  $: canExportQuestions = hasValidQuestionConfig && canExportPassphrase;

  function calculateStrength(pass: string): { score: number; label: string; color: string } {
    if (!pass) return { score: 0, label: '', color: '#ddd' };
    let score = 0;
    if (pass.length >= 8) score += 1;
    if (pass.length >= 12) score += 1;
    if (pass.length >= 16) score += 1;
    if (/[a-z]/.test(pass) && /[A-Z]/.test(pass)) score += 1;
    if (/\d/.test(pass)) score += 1;
    if (/[^a-zA-Z0-9]/.test(pass)) score += 1;
    if (pass.includes('-') && pass.split('-').length >= 3) score += 2;

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

  async function handleExportWithQuestions() {
    if (!canExportQuestions) return;

    error = '';
    successMessage = '';
    isExporting = true;

    try {
      const html = await invoke<string>('export_html_with_questions', {
        passphrase,
        includeWelcomeScreen: true
      });

      if (!html || html.length === 0) {
        throw new Error('Generated HTML is empty');
      }

      const filePath = await saveHtmlFile(html);
      if (!filePath) {
        isExporting = false;
        return;
      }

      if (includePrint) {
        const printHtml = await invoke<string>('get_print_html');
        const printFrame = document.createElement('iframe');
        printFrame.style.display = 'none';
        document.body.appendChild(printFrame);
        printFrame.contentDocument?.write(printHtml);
        printFrame.contentDocument?.close();
        printFrame.contentWindow?.print();
        document.body.removeChild(printFrame);
      }

      successMessage = isMobile() ? 'File shared successfully!' : 'File exported successfully!';
      dispatch('exported', { filePath });
    } catch (e) {
      error = `Export failed: ${e}`;
    } finally {
      isExporting = false;
    }
  }

  async function handleExportWithPassphrase() {
    if (!canExportPassphrase) return;

    error = '';
    successMessage = '';
    isExporting = true;

    try {
      const html = await invoke<string>('export_html', {
        passphrase,
        includeWelcomeScreen: !!(legacyWelcomeAvailable && includeWelcomeScreen)
      });

      if (!html || html.length === 0) {
        throw new Error('Generated HTML is empty');
      }

      const filePath = await saveHtmlFile(html);
      if (!filePath) {
        isExporting = false;
        return;
      }

      if (includePrint) {
        const printHtml = await invoke<string>('get_print_html');
        const printFrame = document.createElement('iframe');
        printFrame.style.display = 'none';
        document.body.appendChild(printFrame);
        printFrame.contentDocument?.write(printHtml);
        printFrame.contentDocument?.close();
        printFrame.contentWindow?.print();
        document.body.removeChild(printFrame);
      }

      successMessage = isMobile() ? 'File shared successfully!' : 'File exported successfully!';
      dispatch('exported', { filePath });
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
    successMessage = '';
    dispatch('close');
  }
</script>

{#if isOpen}
  <div class="overlay" on:keydown={(e) => e.key === 'Escape' && close()} role="presentation">
    <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
    <div class="dialog" role="dialog" aria-modal="true" aria-labelledby="export-dialog-title" on:click|stopPropagation on:keydown|stopPropagation>
      <h2 id="export-dialog-title">Create Your Secure File</h2>

      {#if successMessage}
        <div class="success-view">
          <p class="success-message">{successMessage}</p>
          <div class="actions">
            <button type="button" class="btn-primary" on:click={close}>Done</button>
          </div>
        </div>
      {:else}

      {#if isEmpty}
        <p class="warning empty-warning">Your document is empty. The exported file won't contain any information.</p>
      {/if}

      {#if hasInvalidQuestionConfig}
        <p class="warning empty-warning">You have 1 question on the Welcome Screen. Question-based unlock requires at least 2 questions. Add another question or remove the existing one.</p>
      {/if}

      {#if hasValidQuestionConfig}
        <!-- Question-based export mode -->
        <div class="form">
          <div class="info-box">
            <div class="info-icon">?</div>
            <div>
              <strong>Question-based unlock enabled</strong>
              <p>Your file will be unlocked by answering {questionCount} question{questionCount === 1 ? '' : 's'} you set up in the Welcome Screen section.</p>
            </div>
          </div>

          <div class="summary">
            <div class="summary-item">
              <span class="summary-label">Messages:</span>
              <span>{messageSlides.length} slide{messageSlides.length === 1 ? '' : 's'}</span>
            </div>
            <div class="summary-item">
              <span class="summary-label">Questions:</span>
              <span>{questionCount} question{questionCount === 1 ? '' : 's'}</span>
            </div>
          </div>

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

          <p class="info-note">This passphrase will serve as a backup in case the recipient forgets the answers to the questions.</p>

          <label class="checkbox-field">
            <input type="checkbox" bind:checked={includePrint} />
            <span>Also print a physical copy</span>
          </label>

          {#if includePrint}
            <p class="warning">
              Printed copies can be found by anyone. Store securely.
            </p>
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
            on:click={handleExportWithQuestions}
            disabled={!canExportQuestions}
          >
            {isExporting ? 'Exporting...' : 'Export File'}
          </button>
        </div>

      {:else}
        <!-- Passphrase-based export mode -->
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

          {#if legacyWelcomeAvailable}
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
            on:click={handleExportWithPassphrase}
            disabled={!canExportPassphrase}
          >
            {isExporting ? 'Exporting...' : 'Export File'}
          </button>
        </div>
      {/if}

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

  .info-box {
    display: flex;
    gap: 12px;
    padding: 16px;
    background: var(--accent-light);
    border-radius: 8px;
    border-left: 4px solid var(--accent-secondary);
  }

  .info-icon {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    background: var(--accent-secondary);
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
    font-size: 1.25rem;
    flex-shrink: 0;
  }

  .info-box strong {
    color: var(--text-primary);
    display: block;
    margin-bottom: 4px;
  }

  .info-box p {
    margin: 0;
    color: var(--accent-secondary);
    font-size: 0.9rem;
  }

  .summary {
    background: var(--bg-tertiary);
    padding: 12px 16px;
    border-radius: 8px;
  }

  .summary-item {
    display: flex;
    justify-content: space-between;
    padding: 4px 0;
  }

  .summary-label {
    color: var(--text-secondary);
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

  .passphrase-input {
    display: flex;
    gap: 8px;
  }

  .passphrase-input input {
    flex: 1;
  }

  .generate-btn {
    padding: 10px 16px;
    background: var(--accent-light);
    border: none;
    border-radius: 6px;
    color: var(--accent-primary);
    cursor: pointer;
    white-space: nowrap;
    font-weight: 500;
    transition: background 0.15s ease;
  }

  .generate-btn:hover {
    background: var(--accent-hover);
  }

  .strength-meter {
    height: 4px;
    background: var(--border-color);
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
    color: var(--error-color);
    font-size: 0.85rem;
    margin-top: 4px;
  }

  .checkbox-field {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    color: var(--text-primary);
  }

  .checkbox-field input {
    width: 18px;
    height: 18px;
  }

  .warning {
    padding: 10px 12px;
    background: var(--warning-bg);
    border-radius: 6px;
    font-size: 0.9rem;
    color: var(--warning-text);
    margin: 0;
  }

  .empty-warning {
    margin-bottom: 16px;
  }

  .info-note {
    padding: 10px 12px;
    background: var(--accent-light);
    border-radius: 6px;
    font-size: 0.9rem;
    color: var(--accent-secondary);
    margin: 0;
  }

  .error-message {
    color: var(--error-color);
    background: rgba(155, 44, 44, 0.1);
    padding: 10px 12px;
    border-radius: 6px;
    margin: 0;
  }

  .success-view {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .success-message {
    color: var(--accent-secondary);
    background: var(--accent-light);
    padding: 16px;
    border-radius: 8px;
    margin: 0;
    font-weight: 500;
    text-align: center;
    font-size: 1.05rem;
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

  @media (max-width: 768px) {
    .dialog {
      max-width: calc(100vw - 32px);
      max-height: calc(100vh - 32px);
      overflow-y: auto;
    }
  }
</style>
