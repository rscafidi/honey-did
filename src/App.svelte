<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { document, isDocumentEmpty, setPasswordRequired, type CustomSection, customSectionsStore } from './lib/stores/document';
  import { theme } from './lib/stores/theme';
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
  import LicenseModal from './lib/components/LicenseModal.svelte';
  import HelpModal from './lib/components/HelpModal.svelte';

  type Section =
    | 'financial' | 'insurance' | 'bills' | 'property' | 'legal'
    | 'digital' | 'household' | 'personal' | 'contacts' | 'medical' | 'pets' | 'welcome';

  let currentSection: Section | string = 'financial';
  let showExportDialog = false;
  let showImportDialog = false;
  let isGuidedMode = false;
  let showIntro = false;
  let hasCheckedEmpty = false;

  // Mobile sidebar state
  let sidebarOpen = false;

  function toggleSidebar() {
    sidebarOpen = !sidebarOpen;
  }

  function closeSidebar() {
    sidebarOpen = false;
  }

  // Password protection state
  let isLocked = false;
  let hasPassword = false;
  let showSetPasswordModal = false;
  let showSettings = false;
  let showLicense = false;
  let showHelp = false;
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
  $: customTopLevelSections = ($customSectionsStore || []).filter((s: CustomSection) => !s.parent);

  // Detect invalid welcome screen question state (exactly 1 question)
  $: welcomeQuestionCount = ($document?.welcome_screen?.slides?.filter(s => s.type === 'question') || []).length;
  $: hasInvalidQuestionConfig = $document?.welcome_screen?.enabled && welcomeQuestionCount === 1;

  let showQuestionWarning = false;
  let pendingSection: Section | string | null = null;

  function navigateToSection(sectionId: Section | string) {
    if (currentSection === 'welcome' && sectionId !== 'welcome' && hasInvalidQuestionConfig) {
      pendingSection = sectionId;
      showQuestionWarning = true;
      return;
    }
    currentSection = sectionId;
    closeSidebar();
  }

  function dismissQuestionWarning(proceed: boolean) {
    showQuestionWarning = false;
    if (proceed && pendingSection) {
      currentSection = pendingSection;
    }
    pendingSection = null;
  }

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

  // Save to disk when switching sections (after the old section's onDestroy flushes to store)
  $: if (currentSection) {
    // Use setTimeout(0) so the outgoing component's onDestroy runs first
    setTimeout(() => document.saveToDisk(), 0);
  }

  $: currentSectionLabel = currentSection.startsWith('custom-')
    ? customTopLevelSections.find(s => s.id === currentSection.replace('custom-', ''))?.name || 'Custom Section'
    : sections.find((s) => s.id === currentSection)?.label || '';

  onMount(async () => {
    // Detect Android and set safe area CSS variable.
    // Android WebView does not populate env(safe-area-inset-bottom) for the
    // system navigation bar, so we detect it and apply a fixed inset.
    if (/android/i.test(navigator.userAgent)) {
      window.document.documentElement.style.setProperty('--android-nav-bar-height', '48px');
    }

    // Check if app has a password set.
    // On Android, the Tauri backend may not be ready after activity recreation,
    // so we retry with delays to avoid getting stuck on the loading screen.
    for (let attempt = 0; attempt < 10; attempt++) {
      try {
        const passwordCheck = invoke<boolean>('has_app_password');
        const timeout = new Promise<never>((_, reject) =>
          setTimeout(() => reject(new Error('timeout')), 3000)
        );
        hasPassword = await Promise.race([passwordCheck, timeout]);
        if (hasPassword) {
          isLocked = true;
        }
        break;
      } catch (e) {
        if (attempt < 9) {
          await new Promise(r => setTimeout(r, 500));
        } else {
          console.error('Failed to check password status after retries:', e);
        }
      }
    }

    isLoading = false;

    if (!isLocked) {
      await document.load();
      // Show intro screen for empty documents
      if (!hasCheckedEmpty) {
        hasCheckedEmpty = true;
        setTimeout(() => {
          const doc = $document;
          if (isDocumentEmpty(doc)) {
            showIntro = true;
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
    resetInactivityTimer();
    // Show intro screen for empty document after unlock
    if (!hasCheckedEmpty) {
      hasCheckedEmpty = true;
      setTimeout(() => {
        const doc = $document;
        if (isDocumentEmpty(doc)) {
          showIntro = true;
        }
      }, 100);
    }
  }

  function handleDataCleared() {
    // Reset app state after data is cleared
    hasPassword = false;
    isLocked = false;
    showIntro = true;
    hasCheckedEmpty = true;
    document.load();
  }

  function startGuidedSetup() {
    showIntro = false;
    isGuidedMode = true;
  }

  function skipIntro() {
    showIntro = false;
  }

  function handlePasswordCreated() {
    hasPassword = true;
    showSetPasswordModal = false;
    resetInactivityTimer();
  }

  function enterGuidedMode() {
    isGuidedMode = true;
  }

  function exitGuidedMode() {
    isGuidedMode = false;
  }

  // Save to disk on window blur / visibility change (user tabbed away or switched apps)
  function handleVisibilityOrBlur() {
    document.saveToDisk();
  }

  // Inactivity lock: re-lock after 60 minutes of no user interaction
  const INACTIVITY_TIMEOUT_MS = 60 * 60 * 1000; // 1 hour
  let inactivityTimer: ReturnType<typeof setTimeout> | null = null;

  function resetInactivityTimer() {
    if (inactivityTimer) clearTimeout(inactivityTimer);
    if (hasPassword && !isLocked) {
      inactivityTimer = setTimeout(() => {
        if (hasPassword && !isLocked) {
          document.saveToDisk();
          isLocked = true;
        }
      }, INACTIVITY_TIMEOUT_MS);
    }
  }

  function handleUserActivity() {
    resetInactivityTimer();
  }

  // Window close handler for "Clear on Exit" feature + final save
  let unlistenClose: (() => void) | null = null;

  onMount(async () => {
    window.addEventListener('blur', handleVisibilityOrBlur);
    window.addEventListener('visibilitychange', handleVisibilityOrBlur);
    window.addEventListener('pagehide', handleVisibilityOrBlur);

    // Track user activity for inactivity lock
    window.addEventListener('pointerdown', handleUserActivity);
    window.addEventListener('keydown', handleUserActivity);
    window.addEventListener('scroll', handleUserActivity, true);

    const appWindow = getCurrentWindow();
    unlistenClose = await appWindow.onCloseRequested(async (event) => {
      try {
        // Flush any unsaved edits before closing
        await document.saveToDisk();
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
    window.removeEventListener('blur', handleVisibilityOrBlur);
    window.removeEventListener('visibilitychange', handleVisibilityOrBlur);
    window.removeEventListener('pagehide', handleVisibilityOrBlur);
    window.removeEventListener('pointerdown', handleUserActivity);
    window.removeEventListener('keydown', handleUserActivity);
    window.removeEventListener('scroll', handleUserActivity, true);
    if (inactivityTimer) clearTimeout(inactivityTimer);
    if (unlistenClose) {
      unlistenClose();
    }
  });
</script>

{#if isLoading}
  <div class="loading">Loading...</div>
{:else if isLocked}
  <LockScreen on:unlock={handleUnlock} on:cleared={handleDataCleared} />
{:else if showIntro}
  <div class="intro-screen">
    <div class="intro-card">
      <svg class="intro-logo" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
        <rect x="8" y="6" width="32" height="36" rx="2" fill="#F0EFEB" stroke="#DDE5B6" stroke-width="1.5"/>
        <ellipse cx="24" cy="6" rx="16" ry="3" fill="#DDE5B6"/>
        <ellipse cx="24" cy="6" rx="14" ry="2" fill="#F0EFEB"/>
        <ellipse cx="24" cy="42" rx="16" ry="3" fill="#DDE5B6"/>
        <ellipse cx="24" cy="42" rx="14" ry="2" fill="#F0EFEB"/>
        <text x="24" y="28" text-anchor="middle" font-family="Georgia, serif" font-style="italic" font-size="16" font-weight="600" fill="#283618">HD</text>
        <line x1="14" y1="34" x2="34" y2="34" stroke="#B7B7A4" stroke-width="1" stroke-linecap="round"/>
        <line x1="16" y1="37" x2="32" y2="37" stroke="#B7B7A4" stroke-width="0.75" stroke-linecap="round"/>
      </svg>
      <h1 class="intro-heading">Welcome to Honey Did</h1>
      <p class="intro-description">
        Are you the DOER in your family?  What would your family do if you disappeared tomorrow?  Welcome to Honey Did,
        a companion application to help you build a list of important information your loved ones may need in the event of your permanent absence.
        When you're done collecting information, export it into a highly portable and secure format that key individuals can open when needed.
      </p>
      <p class="intro-description">
        You can start with a guided setup that walks you through each section,
        or jump straight into the app and fill things in at your own pace.
      </p>
      <div class="intro-actions">
        <button class="btn-intro btn-intro-primary" on:click={startGuidedSetup}>
          Start Guided Setup
        </button>
        <button class="btn-intro btn-intro-secondary" on:click={skipIntro}>
          Skip to App
        </button>
      </div>
    </div>
  </div>
{:else if isGuidedMode}
  <GuidedWizard on:exit={exitGuidedMode} />
{:else}
  <div class="mobile-header">
    <button class="hamburger" on:click={toggleSidebar} aria-label="Toggle menu">
      <span class="hamburger-line"></span>
      <span class="hamburger-line"></span>
      <span class="hamburger-line"></span>
    </button>
    <span class="mobile-title">{currentSectionLabel}</span>
  </div>
  <main class="app">
    <aside class="sidebar" class:open={sidebarOpen}>
      <div class="logo">
        <svg class="logo-icon" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
          <!-- Scroll body -->
          <rect x="8" y="6" width="32" height="36" rx="2" fill="#F0EFEB" stroke="#DDE5B6" stroke-width="1.5"/>
          <!-- Top roll -->
          <ellipse cx="24" cy="6" rx="16" ry="3" fill="#DDE5B6"/>
          <ellipse cx="24" cy="6" rx="14" ry="2" fill="#F0EFEB"/>
          <!-- Bottom roll -->
          <ellipse cx="24" cy="42" rx="16" ry="3" fill="#DDE5B6"/>
          <ellipse cx="24" cy="42" rx="14" ry="2" fill="#F0EFEB"/>
          <!-- Cursive HD text -->
          <text x="24" y="28" text-anchor="middle" font-family="Georgia, serif" font-style="italic" font-size="16" font-weight="600" fill="#283618">HD</text>
          <!-- Decorative lines -->
          <line x1="14" y1="34" x2="34" y2="34" stroke="#B7B7A4" stroke-width="1" stroke-linecap="round"/>
          <line x1="16" y1="37" x2="32" y2="37" stroke="#B7B7A4" stroke-width="0.75" stroke-linecap="round"/>
        </svg>
        <span class="logo-text">Honey Did</span>
      </div>
      <nav class="nav">
        {#each sections as section}
          <button
            class="nav-item"
            class:active={currentSection === section.id}
            on:click={() => navigateToSection(section.id)}
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
              on:click={() => navigateToSection(`custom-${customSection.id}`)}
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
          <button class="btn-icon" on:click={() => (showHelp = true)} title="Help">
            ❓
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
        <div class="sidebar-legal">
          <span>&copy; scafidi.dev</span>
          <span class="legal-sep">&middot;</span>
          <!-- svelte-ignore a11y-no-static-element-interactions -->
          <!-- svelte-ignore a11y-click-events-have-key-events -->
          <span class="license-link" on:click={() => (showLicense = true)}>MIT License</span>
        </div>
      </div>
    </aside>
    {#if sidebarOpen}
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <div class="sidebar-backdrop" on:click={closeSidebar}></div>
    {/if}
    <section class="content">
      <header class="content-header">
        <h2>{currentSectionLabel}</h2>
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
  on:passwordCreated={handlePasswordCreated}
/>

<LicenseModal
  bind:isOpen={showLicense}
  on:close={() => (showLicense = false)}
/>

<HelpModal
  bind:isOpen={showHelp}
  on:close={() => (showHelp = false)}
/>

{#if showQuestionWarning}
  <div class="warning-overlay" on:click={() => dismissQuestionWarning(false)} on:keydown={(e) => e.key === 'Escape' && dismissQuestionWarning(false)} role="presentation">
    <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
    <div class="warning-dialog" role="dialog" aria-modal="true" on:click|stopPropagation on:keydown|stopPropagation>
      <p class="warning-text">You have 1 question on the Welcome Screen. Question-based unlock requires at least 2 questions. Add another question or remove the existing one.</p>
      <div class="warning-actions">
        <button class="btn btn-secondary" on:click={() => dismissQuestionWarning(false)}>Go Back</button>
        <button class="btn btn-primary" on:click={() => dismissQuestionWarning(true)}>Continue Anyway</button>
      </div>
    </div>
  </div>
{/if}

<style>
  @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&display=swap');

  :global(:root) {
    /* Bottom safe area: env() works on iOS, --android-nav-bar-height is set via JS on Android */
    --android-nav-bar-height: 0px;
    --safe-area-bottom: max(env(safe-area-inset-bottom), var(--android-nav-bar-height));

    /* Light theme (default) */
    --bg-primary: #F0EFEB;
    --bg-secondary: #FFFFFF;
    --bg-tertiary: #F5F5F5;
    --text-primary: #283618;
    --text-secondary: #606060;
    --text-muted: #888888;
    --border-color: #D4D4D4;
    --accent-primary: #283618;
    --accent-secondary: #606C38;
    --accent-light: #DDE5B6;
    --accent-hover: #ADC178;
    --sidebar-bg: #283618;
    --sidebar-text: #F0EFEB;
    --sidebar-text-muted: #B7B7A4;
    --sidebar-active-bg: #F0EFEB;
    --sidebar-active-text: #283618;
    --error-color: #9B2C2C;
    --warning-bg: #FEFCBF;
    --warning-text: #744210;
    --card-shadow: 0 2px 8px rgba(40, 54, 24, 0.1);
  }

  :global([data-theme="dark"]) {
    --bg-primary: #1a1a1a;
    --bg-secondary: #2d2d2d;
    --bg-tertiary: #383838;
    --text-primary: #F0EFEB;
    --text-secondary: #B7B7A4;
    --text-muted: #888888;
    --border-color: #444444;
    --accent-primary: #DDE5B6;
    --accent-secondary: #ADC178;
    --accent-light: #3d4a2a;
    --accent-hover: #4a5a32;
    --sidebar-bg: #0f0f0f;
    --sidebar-text: #F0EFEB;
    --sidebar-text-muted: #888888;
    --sidebar-active-bg: #3d4a2a;
    --sidebar-active-text: #F0EFEB;
    --error-color: #FC8181;
    --warning-bg: #744210;
    --warning-text: #FEFCBF;
    --card-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  }

  :global(html), :global(body) {
    margin: 0;
    padding: 0;
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
    background: var(--bg-primary);
    color: var(--text-primary);
  }

  .loading {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100vh;
    background: var(--bg-primary);
    font-size: 1.2rem;
    color: var(--text-primary);
  }

  .app {
    display: flex;
    height: 100vh;
    background: var(--bg-primary);
  }

  .sidebar {
    width: 240px;
    background: var(--sidebar-bg);
    display: flex;
    flex-direction: column;
  }

  .logo {
    padding: 16px 20px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .logo-icon {
    width: 40px;
    height: 40px;
    flex-shrink: 0;
  }

  .logo-text {
    font-size: 1.25rem;
    color: var(--sidebar-text);
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
    color: var(--sidebar-text-muted);
    transition: all 0.15s ease;
  }

  .nav-item:hover {
    background: rgba(255, 255, 255, 0.1);
    color: var(--sidebar-text);
  }

  .nav-item.active {
    background: var(--sidebar-active-bg);
    color: var(--sidebar-active-text);
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
    color: var(--sidebar-text-muted);
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
    color: var(--error-color);
  }

  .add-section-btn {
    display: block;
    width: calc(100% - 20px);
    margin: 10px;
    padding: 10px 12px;
    background: none;
    border: 1px dashed rgba(255, 255, 255, 0.3);
    border-radius: 8px;
    color: var(--sidebar-text-muted);
    cursor: pointer;
    text-align: left;
    font-size: 0.9rem;
    transition: all 0.15s ease;
  }

  .add-section-btn:hover {
    border-color: rgba(255, 255, 255, 0.5);
    color: var(--sidebar-text);
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
    color: var(--sidebar-text);
    font-size: 0.9rem;
    box-sizing: border-box;
  }

  .add-section-input::placeholder {
    color: var(--sidebar-text-muted);
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
    color: var(--sidebar-text-muted);
  }

  .btn-cancel:hover {
    background: rgba(255, 255, 255, 0.2);
  }

  .btn-create {
    background: var(--accent-light);
    color: var(--accent-primary);
  }

  .btn-create:hover {
    background: var(--accent-hover);
  }

  .sidebar-footer {
    padding: 15px;
    padding-bottom: calc(15px + var(--safe-area-bottom, 0px));
    border-top: 1px solid rgba(255, 255, 255, 0.1);
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .footer-row {
    display: flex;
    align-items: stretch;
    gap: 8px;
  }

  .flex-1 {
    flex: 1;
  }

  .btn-icon {
    width: 40px;
    flex-shrink: 0;
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
    background: var(--sidebar-text);
    color: var(--sidebar-bg);
  }

  .btn-primary:hover {
    background: var(--border-color);
  }

  .btn-secondary {
    background: rgba(255, 255, 255, 0.1);
    color: var(--sidebar-text-muted);
  }

  .btn-secondary:hover {
    background: rgba(255, 255, 255, 0.2);
    color: var(--sidebar-text);
  }

  .btn-outline {
    background: transparent;
    color: var(--sidebar-text-muted);
    border: 2px solid var(--sidebar-text-muted);
  }

  .btn-outline:hover {
    background: rgba(255, 255, 255, 0.1);
    color: var(--sidebar-text);
    border-color: var(--sidebar-text);
  }

  .content {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .content-header {
    padding: 20px;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
  }

  .content-header h2 {
    margin: 0;
    color: var(--text-primary);
    font-weight: 600;
  }

  .content-body {
    flex: 1;
    padding: 20px;
    overflow-y: auto;
    background: var(--bg-primary);
  }

  .sidebar-legal {
    text-align: center;
    font-size: 0.75rem;
    color: var(--sidebar-text-muted);
    padding: 8px 15px 0;
    opacity: 0.7;
  }

  .legal-sep {
    margin: 0 4px;
  }

  .license-link {
    cursor: pointer;
    text-decoration: underline;
    transition: color 0.15s ease;
  }

  .license-link:hover {
    color: var(--sidebar-text);
  }

  .warning-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .warning-dialog {
    background: var(--bg-secondary);
    border-radius: 12px;
    padding: 24px;
    max-width: 400px;
    box-shadow: var(--card-shadow);
  }

  .warning-text {
    margin: 0 0 20px 0;
    color: var(--warning-text);
    background: var(--warning-bg);
    padding: 12px;
    border-radius: 8px;
    font-size: 0.95rem;
    line-height: 1.5;
  }

  .warning-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
  }

  /* Mobile layout */
  .mobile-header {
    display: none;
  }

  @media (max-width: 768px) {
    .mobile-header {
      display: flex;
      align-items: center;
      gap: 12px;
      padding: 16px 16px;
      padding-top: calc(20px + env(safe-area-inset-top));
      background: var(--sidebar-bg);
      color: var(--sidebar-text);
    }

    .hamburger {
      background: none;
      border: none;
      cursor: pointer;
      padding: 12px;
      display: flex;
      flex-direction: column;
      gap: 5px;
    }

    .hamburger-line {
      display: block;
      width: 26px;
      height: 3px;
      background: var(--sidebar-text);
      border-radius: 2px;
    }

    .mobile-title {
      font-weight: 600;
      font-size: 1.1rem;
    }

    .app {
      flex-direction: column;
    }

    .sidebar {
      position: fixed;
      left: 0;
      top: 0;
      bottom: 0;
      z-index: 200;
      padding-top: calc(12px + env(safe-area-inset-top));
      transform: translateX(-100%);
      transition: transform 0.25s ease;
    }

    .sidebar.open {
      transform: translateX(0);
    }

    .sidebar-backdrop {
      position: fixed;
      inset: 0;
      background: rgba(0, 0, 0, 0.5);
      z-index: 150;
    }

    .content {
      width: 100%;
    }

    .content-header {
      display: none;
    }

    .content-body {
      padding: 16px;
      padding-bottom: calc(16px + var(--safe-area-bottom, 0px));
    }
  }

  /* Intro screen */
  .intro-screen {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
    background: var(--bg-primary);
    padding: 20px;
    box-sizing: border-box;
  }

  .intro-card {
    max-width: 480px;
    width: 100%;
    background: var(--bg-secondary);
    border-radius: 16px;
    padding: 48px 40px;
    box-shadow: var(--card-shadow);
    text-align: center;
  }

  .intro-logo {
    width: 72px;
    height: 72px;
    margin-bottom: 24px;
  }

  .intro-heading {
    margin: 0 0 16px;
    font-size: 1.75rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .intro-description {
    margin: 0 0 12px;
    font-size: 1rem;
    line-height: 1.6;
    color: var(--text-secondary);
  }

  .intro-description:last-of-type {
    margin-bottom: 32px;
  }

  .intro-actions {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .btn-intro {
    padding: 14px 24px;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    font-size: 1rem;
    font-weight: 500;
    transition: all 0.15s ease;
  }

  .btn-intro-primary {
    background: var(--accent-primary);
    color: var(--bg-primary);
  }

  .btn-intro-primary:hover {
    background: var(--accent-secondary);
  }

  .btn-intro-secondary {
    background: transparent;
    color: var(--text-secondary);
    border: 1px solid var(--border-color);
  }

  .btn-intro-secondary:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  @media (max-width: 768px) {
    .intro-card {
      padding: 32px 24px;
    }

    .intro-heading {
      font-size: 1.5rem;
    }
  }
</style>
