<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { document, isDocumentEmpty, setPasswordRequired, type CustomSection } from './lib/stores/document';
  import FinancialSection from './lib/sections/FinancialSection.svelte';
  import InsuranceSection from './lib/sections/InsuranceSection.svelte';
  import BillsSection from './lib/sections/BillsSection.svelte';
  import PropertySection from './lib/sections/PropertySection.svelte';
  import LegalSection from './lib/sections/LegalSection.svelte';
  import DigitalSection from './lib/sections/DigitalSection.svelte';
  import HouseholdSection from './lib/sections/HouseholdSection.svelte';
  import PersonalSection from './lib/sections/PersonalSection.svelte';
  import ContactsSection from './lib/sections/ContactsSection.svelte';
  import MedicalSection from './lib/sections/MedicalSection.svelte';
  import PetsSection from './lib/sections/PetsSection.svelte';
  import WelcomeScreenSection from './lib/sections/WelcomeScreenSection.svelte';
  import CustomSectionPage from './lib/sections/CustomSectionPage.svelte';
  import ExportDialog from './lib/components/ExportDialog.svelte';
  import ImportDialog from './lib/components/ImportDialog.svelte';
  import GuidedWizard from './lib/wizard/GuidedWizard.svelte';
  import LockScreen from './lib/components/LockScreen.svelte';
  import SetPasswordModal from './lib/components/SetPasswordModal.svelte';
  import SettingsModal from './lib/components/SettingsModal.svelte';

  type Section =
    | 'financial' | 'insurance' | 'bills' | 'property' | 'legal'
    | 'digital' | 'household' | 'personal' | 'contacts' | 'medical' | 'pets' | 'welcome';

  let currentSection: Section | string = 'financial';
  let showExportDialog = false;
  let showImportDialog = false;
  let isGuidedMode = false;
  let hasCheckedEmpty = false;

  // Password protection state
  let isLocked = false;
  let hasPassword = false;
  let showSetPasswordModal = false;
  let showSettings = false;
  let isLoading = true;

  // Custom section state
  let showAddSectionForm = false;
  let newSectionName = '';

  const sections: { id: Section; label: string; icon: string }[] = [
    { id: 'financial', label: 'Financial', icon: '💰' },
    { id: 'insurance', label: 'Insurance', icon: '🛡️' },
    { id: 'bills', label: 'Bills', icon: '📄' },
    { id: 'property', label: 'Property', icon: '🏠' },
    { id: 'legal', label: 'Legal', icon: '⚖️' },
    { id: 'digital', label: 'Digital Life', icon: '💻' },
    { id: 'household', label: 'Household', icon: '🔧' },
    { id: 'personal', label: 'Personal', icon: '💝' },
    { id: 'contacts', label: 'Contacts', icon: '📞' },
    { id: 'medical', label: 'Medical', icon: '🏥' },
    { id: 'pets', label: 'Pets', icon: '🐾' },
    { id: 'welcome', label: 'Welcome Screen', icon: '👋' },
  ];

  // Custom top-level sections (no parent)
  $: customTopLevelSections = ($document?.custom_sections || []).filter(s => !s.parent);

  function generateId(): string {
    return Math.random().toString(36).substring(2, 9);
  }

  function addCustomSection() {
    if (!newSectionName.trim() || !$document) return;

    const newSection: CustomSection = {
      id: generateId(),
      name: newSectionName.trim(),
      parent: undefined,
      subsections: [],
    };

    document.updateSection('custom_sections', [...($document.custom_sections || []), newSection]);
    currentSection = `custom-${newSection.id}`;
    newSectionName = '';
    showAddSectionForm = false;
  }

  function deleteCustomSection(sectionId: string) {
    if (!$document) return;
    document.updateSection(
      'custom_sections',
      ($document.custom_sections || []).filter(s => s.id !== sectionId)
    );
    if (currentSection === `custom-${sectionId}`) {
      currentSection = 'financial';
    }
  }

  function renameCustomSection(sectionId: string, newName: string) {
    if (!$document) return;
    document.updateSection(
      'custom_sections',
      ($document.custom_sections || []).map(s =>
        s.id === sectionId ? { ...s, name: newName } : s
      )
    );
  }

  function getCurrentSectionLabel(): string {
    if (currentSection.startsWith('custom-')) {
      const customId = currentSection.replace('custom-', '');
      const custom = customTopLevelSections.find(s => s.id === customId);
      return custom?.name || 'Custom Section';
    }
    return sections.find((s) => s.id === currentSection)?.label || '';
  }

  onMount(async () => {
    // Check if app has a password set
    try {
      hasPassword = await invoke<boolean>('has_app_password');
      if (hasPassword) {
        isLocked = true;
      }
    } catch (e) {
      console.error('Failed to check password status:', e);
    }

    isLoading = false;

    if (!isLocked) {
      await document.load();
      // Auto-enter guided mode for empty documents
      if (!hasCheckedEmpty) {
        hasCheckedEmpty = true;
        setTimeout(() => {
          const doc = $document;
          if (isDocumentEmpty(doc)) {
            isGuidedMode = true;
          }
        }, 100);
      }
    }

    // Set up password requirement callback
    setPasswordRequired(() => {
      if (!hasPassword) {
        showSetPasswordModal = true;
      }
    });
  });

  async function handleUnlock() {
    isLocked = false;
    await document.load();
    // Check for empty document after unlock
    if (!hasCheckedEmpty) {
      hasCheckedEmpty = true;
      setTimeout(() => {
        const doc = $document;
        if (isDocumentEmpty(doc)) {
          isGuidedMode = true;
        }
      }, 100);
    }
  }

  function handleDataCleared() {
    // Reset app state after data is cleared
    hasPassword = false;
    isLocked = false;
    isGuidedMode = true;
    hasCheckedEmpty = true;
    document.load();
  }

  function handlePasswordCreated() {
    hasPassword = true;
    showSetPasswordModal = false;
  }

  function enterGuidedMode() {
    isGuidedMode = true;
  }

  function exitGuidedMode() {
    isGuidedMode = false;
  }

  // Window close handler for "Clear on Exit" feature
  let unlistenClose: (() => void) | null = null;

  onMount(async () => {
    const appWindow = getCurrentWindow();
    unlistenClose = await appWindow.onCloseRequested(async (event) => {
      try {
        const clearOnExit = await invoke<boolean>('get_clear_on_exit');
        if (clearOnExit) {
          await invoke('clear_data_on_exit');
        }
      } catch (e) {
        console.error('Error during close:', e);
      }
    });
  });

  onDestroy(() => {
    if (unlistenClose) {
      unlistenClose();
    }
  });
</script>

{#if isLoading}
  <div class="loading">Loading...</div>
{:else if isLocked}
  <LockScreen on:unlock={handleUnlock} on:cleared={handleDataCleared} />
{:else if isGuidedMode}
  <GuidedWizard on:exit={exitGuidedMode} />
{:else}
  <main class="app">
    <aside class="sidebar">
      <div class="logo">
        <h1>Honey Did</h1>
      </div>
      <nav class="nav">
        {#each sections as section}
          <button
            class="nav-item"
            class:active={currentSection === section.id}
            on:click={() => (currentSection = section.id)}
          >
            <span class="nav-icon">{section.icon}</span>
            <span class="nav-label">{section.label}</span>
          </button>
        {/each}

        <!-- Custom top-level sections -->
        {#each customTopLevelSections as customSection (customSection.id)}
          <div class="nav-item-wrapper">
            <button
              class="nav-item"
              class:active={currentSection === `custom-${customSection.id}`}
              on:click={() => (currentSection = `custom-${customSection.id}`)}
            >
              <span class="nav-icon">📁</span>
              <span class="nav-label">{customSection.name}</span>
            </button>
            <button
              class="nav-delete"
              on:click|stopPropagation={() => deleteCustomSection(customSection.id)}
              title="Delete section"
            >×</button>
          </div>
        {/each}

        <!-- Add Section Button -->
        {#if showAddSectionForm}
          <div class="add-section-form">
            <input
              type="text"
              class="add-section-input"
              placeholder="Section name"
              bind:value={newSectionName}
              on:keydown={(e) => e.key === 'Enter' && addCustomSection()}
            />
            <div class="add-section-actions">
              <button class="btn-small btn-cancel" on:click={() => { showAddSectionForm = false; newSectionName = ''; }}>Cancel</button>
              <button class="btn-small btn-create" on:click={addCustomSection}>Create</button>
            </div>
          </div>
        {:else}
          <button class="add-section-btn" on:click={() => (showAddSectionForm = true)}>
            + Add Section
          </button>
        {/if}
      </nav>
      <div class="sidebar-footer">
        <div class="footer-row">
          <button class="btn btn-outline flex-1" on:click={enterGuidedMode}>
            Guided Setup
          </button>
          <button class="btn-icon" on:click={() => (showSettings = true)} title="Settings">
            ⚙️
          </button>
        </div>
        <button class="btn btn-secondary" on:click={() => (showImportDialog = true)}>
          Import File
        </button>
        <button class="btn btn-primary" on:click={() => (showExportDialog = true)}>
          Export
        </button>
      </div>
    </aside>
    <section class="content">
      <header class="content-header">
        <h2>{getCurrentSectionLabel()}</h2>
      </header>
      <div class="content-body">
        {#if currentSection === 'financial'}
          <FinancialSection />
        {:else if currentSection === 'insurance'}
          <InsuranceSection />
        {:else if currentSection === 'bills'}
          <BillsSection />
        {:else if currentSection === 'property'}
          <PropertySection />
        {:else if currentSection === 'legal'}
          <LegalSection />
        {:else if currentSection === 'digital'}
          <DigitalSection />
        {:else if currentSection === 'household'}
          <HouseholdSection />
        {:else if currentSection === 'personal'}
          <PersonalSection />
        {:else if currentSection === 'contacts'}
          <ContactsSection />
        {:else if currentSection === 'medical'}
          <MedicalSection />
        {:else if currentSection === 'pets'}
          <PetsSection />
        {:else if currentSection === 'welcome'}
          <WelcomeScreenSection />
        {:else if currentSection.startsWith('custom-')}
          {@const customId = currentSection.replace('custom-', '')}
          {@const customSection = customTopLevelSections.find(s => s.id === customId)}
          {#if customSection}
            <CustomSectionPage
              section={customSection}
              on:update={(e) => {
                if ($document) {
                  document.updateSection(
                    'custom_sections',
                    ($document.custom_sections || []).map(s =>
                      s.id === customSection.id ? e.detail : s
                    )
                  );
                }
              }}
              on:rename={(e) => renameCustomSection(customSection.id, e.detail)}
            />
          {/if}
        {/if}
      </div>
    </section>
  </main>
{/if}

<ExportDialog
  bind:isOpen={showExportDialog}
  on:close={() => (showExportDialog = false)}
  on:exported={(e) => {
    console.log('Exported to:', e.detail.filePath);
  }}
/>

<ImportDialog
  bind:isOpen={showImportDialog}
  on:close={() => (showImportDialog = false)}
  on:imported={(e) => {
    console.log('Imported from:', e.detail.fileName);
    document.load();
  }}
/>

<SetPasswordModal
  bind:isOpen={showSetPasswordModal}
  on:created={handlePasswordCreated}
  on:cancel={() => (showSetPasswordModal = false)}
/>

<SettingsModal
  bind:isOpen={showSettings}
  on:close={() => (showSettings = false)}
  on:cleared={handleDataCleared}
/>

<style>
  @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&display=swap');

  :global(html), :global(body) {
    margin: 0;
    padding: 0;
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
  }

  .loading {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100vh;
    background: #F0EFEB;
    font-size: 1.2rem;
    color: #283618;
  }

  .app {
    display: flex;
    height: 100vh;
    background: #F0EFEB;
  }

  .sidebar {
    width: 240px;
    background: #283618;
    display: flex;
    flex-direction: column;
  }

  .logo {
    padding: 20px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .logo h1 {
    margin: 0;
    font-size: 1.5rem;
    color: #F0EFEB;
    font-weight: 600;
  }

  .nav {
    flex: 1;
    overflow-y: auto;
    padding: 10px;
  }

  .nav-item {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 12px;
    border: none;
    background: none;
    border-radius: 8px;
    cursor: pointer;
    text-align: left;
    gap: 12px;
    color: #B7B7A4;
    transition: all 0.15s ease;
  }

  .nav-item:hover {
    background: rgba(255, 255, 255, 0.1);
    color: #F0EFEB;
  }

  .nav-item.active {
    background: #F0EFEB;
    color: #283618;
  }

  .nav-icon {
    font-size: 1.25rem;
  }

  .nav-label {
    flex: 1;
    font-weight: 500;
  }

  .nav-item-wrapper {
    position: relative;
    display: flex;
    align-items: center;
  }

  .nav-item-wrapper .nav-item {
    flex: 1;
  }

  .nav-delete {
    position: absolute;
    right: 8px;
    background: none;
    border: none;
    color: #B7B7A4;
    font-size: 1.2rem;
    cursor: pointer;
    opacity: 0;
    transition: opacity 0.15s ease, color 0.15s ease;
    padding: 4px 8px;
  }

  .nav-item-wrapper:hover .nav-delete {
    opacity: 1;
  }

  .nav-delete:hover {
    color: #9B2C2C;
  }

  .add-section-btn {
    display: block;
    width: calc(100% - 20px);
    margin: 10px;
    padding: 10px 12px;
    background: none;
    border: 1px dashed rgba(255, 255, 255, 0.3);
    border-radius: 8px;
    color: #B7B7A4;
    cursor: pointer;
    text-align: left;
    font-size: 0.9rem;
    transition: all 0.15s ease;
  }

  .add-section-btn:hover {
    border-color: rgba(255, 255, 255, 0.5);
    color: #F0EFEB;
    background: rgba(255, 255, 255, 0.05);
  }

  .add-section-form {
    padding: 10px;
  }

  .add-section-input {
    width: 100%;
    padding: 10px 12px;
    border: 1px solid rgba(255, 255, 255, 0.3);
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.1);
    color: #F0EFEB;
    font-size: 0.9rem;
    box-sizing: border-box;
  }

  .add-section-input::placeholder {
    color: #B7B7A4;
  }

  .add-section-input:focus {
    outline: none;
    border-color: rgba(255, 255, 255, 0.5);
  }

  .add-section-actions {
    display: flex;
    gap: 8px;
    margin-top: 8px;
  }

  .btn-small {
    padding: 6px 12px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.8rem;
    font-weight: 500;
  }

  .btn-cancel {
    background: rgba(255, 255, 255, 0.1);
    color: #B7B7A4;
  }

  .btn-cancel:hover {
    background: rgba(255, 255, 255, 0.2);
  }

  .btn-create {
    background: #DDE5B6;
    color: #283618;
  }

  .btn-create:hover {
    background: #ADC178;
  }

  .sidebar-footer {
    padding: 15px;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .footer-row {
    display: flex;
    gap: 8px;
  }

  .flex-1 {
    flex: 1;
  }

  .btn-icon {
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    cursor: pointer;
    font-size: 1.2rem;
    transition: background 0.15s ease;
  }

  .btn-icon:hover {
    background: rgba(255, 255, 255, 0.2);
  }

  .btn {
    padding: 10px 16px;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.9rem;
    font-weight: 500;
    transition: all 0.15s ease;
  }

  .btn-primary {
    background: #F0EFEB;
    color: #283618;
  }

  .btn-primary:hover {
    background: #D4D4D4;
  }

  .btn-secondary {
    background: rgba(255, 255, 255, 0.1);
    color: #B7B7A4;
  }

  .btn-secondary:hover {
    background: rgba(255, 255, 255, 0.2);
    color: #F0EFEB;
  }

  .btn-outline {
    background: transparent;
    color: #B7B7A4;
    border: 2px solid #B7B7A4;
  }

  .btn-outline:hover {
    background: rgba(255, 255, 255, 0.1);
    color: #F0EFEB;
    border-color: #F0EFEB;
  }

  .content {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .content-header {
    padding: 20px;
    background: white;
    border-bottom: 1px solid #D4D4D4;
  }

  .content-header h2 {
    margin: 0;
    color: #283618;
    font-weight: 600;
  }

  .content-body {
    flex: 1;
    padding: 20px;
    overflow-y: auto;
  }
</style>
