<script lang="ts">
  import { createEventDispatcher } from 'svelte';
        export let isOpen = false;
        const dispatch = createEventDispatcher();
        let activeSection: string | null = null;
        function toggle(id: string) {
    activeSection = activeSection === id ? null : id;
  }
        function close() {
    activeSection = null;
    dispatch('close');
  }
        const topics = [
    {
      id: 'getting-started',
      title: 'Getting Started',
      content: `Honey Did is a secure legacy document creator. It guides you through recording important information (finances, medical details, legal documents, contacts) and can export it in highly portable format (HTML document with encryption) so your loved ones have the information they need in your absence.  
      The app is organized into sections, each covering a category of information. You can navigate between sections using the sidebar on the left.
      The easiest way to begin is with <strong>Guided Setup</strong> (button in the sidebar footer). It walks you through the most critical categories step-by-step, starting with the essentials like finances, insurance, and legal matters, then continuing to secondary categories like household and personal wishes.
      You can also click any section in the sidebar and start filling in information directly.`
    },
    {
      id: 'filling-in-sections',
      title: 'Filling In Sections',
      content: `Each built-in section (Financial, Medical, Insurance, etc.) has predefined subsections for common items. For example, the Financial section includes Bank Accounts, Credit Cards, Investments, and Debts.
      To add an item, click the <strong>+ Add</strong> button within a subsection. Fill in the fields and your data is saved automatically.
      Most sections also include a <strong>Notes</strong> field at the bottom — use it for anything that doesn't fit the structured fields, like special instructions or context your recipient might need.
      You don't have to fill in everything at once. Come back any time to add or update information.`
    },
    {
      id: 'custom-sections',
      title: 'Custom Sections',
      content: `If the built-in sections don't cover something you need to track, you can create custom sections in two ways:
      <strong>Custom subsections within built-in sections:</strong> Scroll to the bottom of any built-in section (like Financial or Medical) and click <strong>+ Add Custom Subsection</strong>. Give it a name, then click the <strong>gear icon</strong> (⚙) to define the fields you want to track. Click <strong>Done</strong> when finished, then use <strong>+ Add Item</strong> to start adding entries.
      <strong>Top-level custom sections:</strong> Click <strong>+ Add Section</strong> at the bottom of the sidebar to create an entirely new section. It works the same way — add subsections, define fields, add items.
      Each custom subsection acts like a small database: you define the fields once (text, number, date, yes/no), and every item you add shares those fields. Think of it like a spreadsheet where you set up columns, then add rows.`
    },
    {
      id: 'welcome-screen',
      title: 'Welcome Screen & Security Questions',
      content: `The Welcome Screen section lets you create a personalized greeting that your recipient sees when they open your exported file.
      <strong>Message slides:</strong> Add messages that display before the document unlocks. Use these for context, instructions, or a personal note.
      <strong>Security questions:</strong> Instead of (or in addition to) a passphrase, you can set up 2–5 questions that your recipient must answer correctly to unlock the document. Answers are case-insensitive. Choose questions only your intended recipient would know.
      <strong>Fallback passphrase:</strong> Optionally set a passphrase as a backup in case your recipient can't remember the answers. You can type one or generate a random one.
      When you export with questions enabled, the recipient sees your message slides first, then answers the questions to unlock the document.`
    },
    {
      id: 'exporting',
      title: 'Exporting Your Document',
      content: `When you're ready to share your document, click <strong>Export</strong> in the sidebar footer.
      <strong>Question-based export:</strong> If you've set up security questions in the Welcome Screen section (2–5 questions), the export will use those questions to protect the file. The recipient answers correctly to unlock it.
      <strong>Passphrase-based export:</strong> If no questions are configured, you'll set a passphrase (minimum 8 characters). A strength meter helps you pick a strong one. You can also generate a random passphrase.
      The exported file is a single <strong>.html file</strong> with your data encrypted inside it. It can be opened in any modern web browser on any device — no special software needed.
      <strong>Print option:</strong> Check "Also print a physical copy" during export to print an unencrypted version. Store physical copies securely.
      Tip: After exporting, open the file yourself to make sure everything looks right and you can unlock it successfully.`
    },
    {
      id: 'importing',
      title: 'Importing a File',
      content: `You can import a previously exported Honey Did file to bring data back into the app.
      Click <strong>Import File</strong> in the sidebar footer, select the .html file, and enter the passphrase used to encrypt it.
      Imported data is <strong>merged</strong> into your current document — it adds to what you already have rather than replacing it. This is useful for combining information from multiple exports.`
    },
    {
      id: 'settings',
      title: 'Settings & Security',
      content: `Click the <strong>gear icon</strong> (⚙️) in the sidebar footer to open Settings.
      <strong>Appearance:</strong> Choose between Auto, Light, or Dark theme. Auto follows your system preference.
      <strong>App password:</strong> Set a password to protect the app itself. Once set, you'll need to enter it each time you open Honey Did. You can change or remove it in Settings.
      <strong>Clear on exit:</strong> When enabled, all data is automatically deleted when you close the app. Useful if you don't want data persisting on the device.
      <strong>Clear all data:</strong> Permanently deletes everything in the app. If a password is set, you'll need to confirm it first. This cannot be undone.`
    },
    {
      id: 'tips',
      title: 'Tips',
      content: `<strong>Start with Guided Setup.</strong> It prioritizes the most important categories and helps you think about what to include.
      <strong>Keep it current.</strong> Revisit your document periodically — when you change banks, doctors, insurance, or other key information.
      <strong>Test your export.</strong> After exporting, open the file in a browser and try unlocking it. Make sure your recipient will be able to access it.
      <strong>Store exports securely.</strong> The exported file is encrypted, but treat it like any sensitive document. Consider storing copies in a safe, a secure cloud folder, or with a trusted person.
      <strong>Use notes liberally.</strong> The Notes field at the bottom of each section is great for context that doesn't fit structured fields — "Call our advisor Sarah first" or "The safe combination is with the attorney."
      <strong>Don't worry about completeness.</strong> Some information is better than none. Fill in what you can now and add more later.`
    }
  ];
</script>
      {#if isOpen}
  <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
  <div class="overlay" on:click={close} on:keydown={(e) => e.key === 'Escape' && close()} role="presentation">
    <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
    <div class="dialog" role="dialog" aria-modal="true" aria-labelledby="help-dialog-title" on:click|stopPropagation on:keydown|stopPropagation>
      <div class="dialog-header">
        <h2 id="help-dialog-title">Help & Guide</h2>
        <button class="close-btn" on:click={close} title="Close">&times;</button>
      </div>
      <div class="dialog-body">
        {#each topics as topic (topic.id)}
          <div class="accordion-item">
            <button
              class="accordion-header"
              class:open={activeSection === topic.id}
              on:click={() => toggle(topic.id)}
            >
              <span class="accordion-arrow">{activeSection === topic.id ? '▾' : '▸'}</span>
              <span>{topic.title}</span>
            </button>
            {#if activeSection === topic.id}
              <div class="accordion-content">
                {@html topic.content}
              </div>
            {/if}
          </div>
        {/each}
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
    z-index: 1100;
  }
        .dialog {
    background: var(--bg-secondary);
    border-radius: 12px;
    width: 100%;
    max-width: 600px;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    box-shadow: var(--card-shadow);
  }
        .dialog-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 20px 24px 16px;
    border-bottom: 1px solid var(--border-color);
    flex-shrink: 0;
  }
        .dialog-header h2 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
  }
        .close-btn {
    background: none;
    border: none;
    font-size: 1.5rem;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 0;
    line-height: 1;
  }
        .close-btn:hover {
    color: var(--text-primary);
  }
        .dialog-body {
    overflow-y: auto;
    padding: 8px 0;
  }
        .accordion-item {
    border-bottom: 1px solid var(--border-color);
  }
        .accordion-item:last-child {
    border-bottom: none;
  }
        .accordion-header {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    padding: 14px 24px;
    background: none;
    border: none;
    cursor: pointer;
    font-size: 0.95rem;
    font-weight: 500;
    color: var(--text-primary);
    text-align: left;
  }
        .accordion-header:hover {
    background: var(--bg-tertiary);
  }
        .accordion-header.open {
    color: var(--accent-primary);
  }
        .accordion-arrow {
    font-size: 0.8rem;
    width: 14px;
    flex-shrink: 0;
    color: var(--text-secondary);
  }
        .accordion-header.open .accordion-arrow {
    color: var(--accent-primary);
  }
        .accordion-content {
    padding: 0 24px 16px 48px;
    font-size: 0.9rem;
    line-height: 1.6;
    color: var(--text-secondary);
  }
        .accordion-content :global(strong) {
    color: var(--text-primary);
    font-weight: 600;
  }

  @media (max-width: 768px) {
    .dialog {
      max-width: calc(100vw - 32px);
      max-height: calc(100vh - 32px);
      overflow-y: auto;
    }
  }
</style>
