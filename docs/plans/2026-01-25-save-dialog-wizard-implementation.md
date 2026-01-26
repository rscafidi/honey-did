# Save Dialog & Guided Wizard Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add native save dialog for exports and guided wizard mode for new users.

**Architecture:** Feature 1 adds Tauri dialog plugin and modifies ExportDialog. Feature 2 adds new GuidedWizard component with step-by-step flow that wraps existing section components.

**Tech Stack:** Svelte 4, TypeScript, Tauri 2.0, tauri-plugin-dialog

---

## Task 1: Add Tauri Dialog Plugin

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Modify: `src-tauri/src/main.rs`
- Modify: `src-tauri/tauri.conf.json` (if needed for permissions)

**Step 1: Add dialog plugin dependency to Cargo.toml**

Add after line 24 (after `directories = "5"`):
```toml
tauri-plugin-dialog = "2"
```

**Step 2: Register plugin in main.rs**

Change line 91-106 to:
```rust
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            document: Mutex::new(document),
        })
        .invoke_handler(tauri::generate_handler![
            get_document,
            update_document,
            export_html,
            save_export,
            get_print_html,
            import_file,
            read_file,
            merge_document,
            generate_passphrase,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
```

**Step 3: Commit**
```bash
git add src-tauri/Cargo.toml src-tauri/src/main.rs
git commit -m "feat: add tauri-plugin-dialog dependency"
```

---

## Task 2: Create save_export_with_dialog Command

**Files:**
- Modify: `src-tauri/src/main.rs`

**Step 1: Add new command after save_export (around line 42)**

```rust
#[tauri::command]
async fn save_export_with_dialog(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    passphrase: String,
) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    let doc = state.document.lock().map_err(|e| e.to_string())?;
    let html = export::generate_encrypted_html(&doc, &passphrase).map_err(|e| e.to_string())?;
    drop(doc); // Release lock before async dialog

    // Generate filename with current date
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let default_name = format!("honey-did-{}.html", date);

    let file_path = app.dialog()
        .file()
        .set_file_name(&default_name)
        .add_filter("HTML Files", &["html", "htm"])
        .save_file();

    match file_path {
        Some(path) => {
            let path_str = path.to_string();
            std::fs::write(&path_str, html)
                .map_err(|e| format!("Failed to save file: {}", e))?;
            Ok(Some(path_str))
        }
        None => Ok(None), // User cancelled
    }
}
```

**Step 2: Register the new command in invoke_handler**

Add `save_export_with_dialog` to the handler list:
```rust
        .invoke_handler(tauri::generate_handler![
            get_document,
            update_document,
            export_html,
            save_export,
            save_export_with_dialog,
            get_print_html,
            import_file,
            read_file,
            merge_document,
            generate_passphrase,
        ])
```

**Step 3: Commit**
```bash
git add src-tauri/src/main.rs
git commit -m "feat: add save_export_with_dialog command with native file picker"
```

---

## Task 3: Update ExportDialog to Use Native Save Dialog

**Files:**
- Modify: `src/lib/components/ExportDialog.svelte`

**Step 1: Remove fileName state and related UI**

Remove line 11:
```typescript
  let fileName = 'honey-did-legacy-document.html';
```

Update line 18 canExport reactive statement:
```typescript
  $: canExport = passphrase.length >= 8 && passphrasesMatch && !isExporting;
```

**Step 2: Update handleExport to use new command**

Replace the handleExport function (lines 46-78):
```typescript
  async function handleExport() {
    if (!canExport) return;

    error = '';
    isExporting = true;

    try {
      // Show native save dialog and save the encrypted file
      const filePath = await invoke<string | null>('save_export_with_dialog', { passphrase });

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
```

**Step 3: Update close function to remove fileName reset**

Change line 83:
```typescript
  function close() {
    passphrase = '';
    confirmPassphrase = '';
    includePrint = false;
    error = '';
    dispatch('close');
  }
```

**Step 4: Remove the filename field from the form (lines 131-140)**

Delete this entire block:
```svelte
        <div class="field">
          <label for="filename">File name</label>
          <input
            id="filename"
            type="text"
            bind:value={fileName}
            placeholder="honey-did-legacy-document.html"
          />
          <span class="hint">File will be saved to your current directory</span>
        </div>
```

**Step 5: Commit**
```bash
git add src/lib/components/ExportDialog.svelte
git commit -m "feat: use native save dialog in ExportDialog"
```

---

## Task 4: Create Wizard Content Data File

**Files:**
- Create: `src/lib/wizard/wizardContent.ts`

**Step 1: Create the wizard content data**

```typescript
export interface WizardStepContent {
  id: string;
  title: string;
  icon: string;
  whyItMatters: string;
  prompts: string[];
  example: string;
}

export const prioritySteps: WizardStepContent[] = [
  {
    id: 'financial',
    title: 'Financial',
    icon: '💰',
    whyItMatters: 'Bank accounts, investments, and debts are essential for your family to manage finances, pay bills, and access funds during a difficult time.',
    prompts: [
      'Do you have any accounts your partner doesn\'t know about?',
      'Is there a safety deposit box? Where is the key?',
      'Any cryptocurrency or digital assets?',
      'Are there any automatic payments set up?',
    ],
    example: 'Chase Bank - Joint Checking - Last 4: 1234 - Primary account for bills',
  },
  {
    id: 'insurance',
    title: 'Insurance',
    icon: '🛡️',
    whyItMatters: 'Insurance policies provide financial protection. Without this information, claims could be delayed or missed entirely.',
    prompts: [
      'Do you have life insurance through your employer?',
      'Are there any policies with cash value?',
      'Who are the beneficiaries on each policy?',
      'Where are the policy documents stored?',
    ],
    example: 'Term Life - Northwestern Mutual - Policy #12345 - $500k - Spouse is beneficiary',
  },
  {
    id: 'legal',
    title: 'Legal',
    icon: '⚖️',
    whyItMatters: 'Legal documents like wills and powers of attorney ensure your wishes are followed and someone can act on your behalf if needed.',
    prompts: [
      'Where are the original documents stored?',
      'Who is your estate attorney?',
      'Is there a trust? Who is the trustee?',
      'Have you designated powers of attorney?',
    ],
    example: 'Will stored in home safe - Combination: ask John Smith (attorney) - Last updated 2024',
  },
  {
    id: 'medical',
    title: 'Medical',
    icon: '🏥',
    whyItMatters: 'Medical information helps family make informed decisions and ensures continuity of care during emergencies.',
    prompts: [
      'Any allergies or drug interactions to know about?',
      'Do you have an advance directive or living will?',
      'Who is your primary care doctor?',
      'Are there any ongoing treatments or medications?',
    ],
    example: 'Dr. Sarah Johnson - Primary Care - (555) 123-4567 - Annual checkup in March',
  },
  {
    id: 'contacts',
    title: 'Contacts',
    icon: '📞',
    whyItMatters: 'Knowing who to call first saves precious time and ensures the right people are notified.',
    prompts: [
      'Who should be called first in an emergency?',
      'Is there anyone who should NOT be contacted?',
      'Who has spare keys to your home?',
      'Any important work contacts to notify?',
    ],
    example: 'John Smith (brother) - (555) 987-6543 - Call first, has spare key',
  },
];

export const secondarySteps: WizardStepContent[] = [
  {
    id: 'bills',
    title: 'Bills',
    icon: '📄',
    whyItMatters: 'Regular bills need to be paid to avoid service interruptions and late fees.',
    prompts: [
      'Which bills are on autopay?',
      'Are there any annual payments that might be forgotten?',
      'What accounts are bills paid from?',
    ],
    example: 'Electric - ConEd - $150/month - Autopay from Chase checking',
  },
  {
    id: 'property',
    title: 'Property',
    icon: '🏠',
    whyItMatters: 'Property details help manage real estate, vehicles, and valuable items.',
    prompts: [
      'Where are property deeds and titles stored?',
      'Any rental properties or timeshares?',
      'Valuable items that should be appraised?',
    ],
    example: 'Home - 123 Main St - Deed in safe deposit box - Mortgage with Wells Fargo',
  },
  {
    id: 'digital',
    title: 'Digital Life',
    icon: '💻',
    whyItMatters: 'Digital accounts contain important information and memories that shouldn\'t be lost.',
    prompts: [
      'Do you use a password manager?',
      'Any subscriptions that should be cancelled?',
      'Where are photos and important files backed up?',
    ],
    example: '1Password - Master password in fireproof safe - Family vault shared',
  },
  {
    id: 'household',
    title: 'Household',
    icon: '🔧',
    whyItMatters: 'Household knowledge keeps the home running smoothly.',
    prompts: [
      'Where is the water shutoff valve?',
      'Any regular maintenance schedules?',
      'Trusted contractors for repairs?',
    ],
    example: 'Water shutoff - basement, left of stairs - red handle',
  },
  {
    id: 'personal',
    title: 'Personal',
    icon: '💝',
    whyItMatters: 'Personal wishes and messages provide comfort and guidance to loved ones.',
    prompts: [
      'Any funeral or memorial preferences?',
      'Messages you\'d want loved ones to have?',
      'Special items to pass to specific people?',
    ],
    example: 'Prefer cremation - scatter ashes at Lake Tahoe',
  },
  {
    id: 'pets',
    title: 'Pets',
    icon: '🐾',
    whyItMatters: 'Pets need continued care and someone who knows their routines.',
    prompts: [
      'Who should care for your pets?',
      'Any medical conditions or special diets?',
      'Where is the vet\'s information?',
    ],
    example: 'Max (golden retriever) - Takes thyroid medication daily - Vet: Dr. Paws (555) 222-3333',
  },
];
```

**Step 2: Commit**
```bash
git add src/lib/wizard/wizardContent.ts
git commit -m "feat: add wizard content with explanations and prompts"
```

---

## Task 5: Create WizardStep Component

**Files:**
- Create: `src/lib/wizard/WizardStep.svelte`

**Step 1: Create the component**

```svelte
<script lang="ts">
  import type { WizardStepContent } from './wizardContent';

  export let step: WizardStepContent;
  export let currentStep: number;
  export let totalSteps: number;
</script>

<div class="wizard-step">
  <div class="step-header">
    <div class="progress">Step {currentStep} of {totalSteps} — {step.title}</div>
    <div class="step-icon">{step.icon}</div>
    <h2>{step.title}</h2>
  </div>

  <div class="why-it-matters">
    <h3>Why this matters</h3>
    <p>{step.whyItMatters}</p>
  </div>

  <div class="form-area">
    <slot />
  </div>

  <div class="prompts">
    <h3>💡 Things to consider</h3>
    <ul>
      {#each step.prompts as prompt}
        <li>{prompt}</li>
      {/each}
    </ul>
  </div>

  <div class="example">
    <h3>Example entry</h3>
    <p class="example-text">"{step.example}"</p>
  </div>
</div>

<style>
  .wizard-step {
    max-width: 800px;
    margin: 0 auto;
  }

  .step-header {
    text-align: center;
    margin-bottom: 24px;
  }

  .progress {
    font-size: 0.9rem;
    color: #666;
    margin-bottom: 8px;
  }

  .step-icon {
    font-size: 3rem;
    margin-bottom: 8px;
  }

  h2 {
    margin: 0;
    color: #333;
  }

  .why-it-matters {
    background: #e3f2fd;
    padding: 16px 20px;
    border-radius: 8px;
    margin-bottom: 24px;
  }

  .why-it-matters h3 {
    margin: 0 0 8px 0;
    font-size: 0.95rem;
    color: #1565c0;
  }

  .why-it-matters p {
    margin: 0;
    color: #333;
    line-height: 1.5;
  }

  .form-area {
    background: white;
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    padding: 20px;
    margin-bottom: 24px;
  }

  .prompts {
    background: #fff8e1;
    padding: 16px 20px;
    border-radius: 8px;
    margin-bottom: 24px;
  }

  .prompts h3 {
    margin: 0 0 12px 0;
    font-size: 0.95rem;
    color: #f57c00;
  }

  .prompts ul {
    margin: 0;
    padding-left: 20px;
  }

  .prompts li {
    margin: 8px 0;
    color: #333;
  }

  .example {
    background: #f5f5f5;
    padding: 16px 20px;
    border-radius: 8px;
  }

  .example h3 {
    margin: 0 0 8px 0;
    font-size: 0.95rem;
    color: #666;
  }

  .example-text {
    margin: 0;
    font-style: italic;
    color: #333;
  }
</style>
```

**Step 2: Commit**
```bash
git add src/lib/wizard/WizardStep.svelte
git commit -m "feat: add WizardStep component with guidance layout"
```

---

## Task 6: Create GuidedWizard Component

**Files:**
- Create: `src/lib/wizard/GuidedWizard.svelte`

**Step 1: Create the main wizard component**

```svelte
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import WizardStep from './WizardStep.svelte';
  import { prioritySteps, secondarySteps } from './wizardContent';
  import FinancialSection from '../sections/FinancialSection.svelte';
  import InsuranceSection from '../sections/InsuranceSection.svelte';
  import LegalSection from '../sections/LegalSection.svelte';
  import MedicalSection from '../sections/MedicalSection.svelte';
  import ContactsSection from '../sections/ContactsSection.svelte';
  import BillsSection from '../sections/BillsSection.svelte';
  import PropertySection from '../sections/PropertySection.svelte';
  import DigitalSection from '../sections/DigitalSection.svelte';
  import HouseholdSection from '../sections/HouseholdSection.svelte';
  import PersonalSection from '../sections/PersonalSection.svelte';
  import PetsSection from '../sections/PetsSection.svelte';

  const dispatch = createEventDispatcher();

  let currentStepIndex = 0;
  let phase: 'priority' | 'askContinue' | 'secondary' | 'complete' = 'priority';

  $: steps = phase === 'secondary' ? secondarySteps : prioritySteps;
  $: currentStep = steps[currentStepIndex];
  $: totalSteps = steps.length;

  function next() {
    if (currentStepIndex < steps.length - 1) {
      currentStepIndex++;
    } else if (phase === 'priority') {
      phase = 'askContinue';
    } else if (phase === 'secondary') {
      phase = 'complete';
    }
  }

  function back() {
    if (currentStepIndex > 0) {
      currentStepIndex--;
    }
  }

  function skip() {
    next();
  }

  function continueWithMore() {
    phase = 'secondary';
    currentStepIndex = 0;
  }

  function finishSetup() {
    phase = 'complete';
  }

  function exitWizard() {
    dispatch('exit');
  }

  function startOver() {
    phase = 'priority';
    currentStepIndex = 0;
  }
</script>

<div class="wizard">
  <header class="wizard-header">
    <h1>Guided Setup</h1>
    <button class="exit-link" on:click={exitWizard}>Exit to Full View</button>
  </header>

  <main class="wizard-content">
    {#if phase === 'askContinue'}
      <div class="continue-prompt">
        <div class="prompt-icon">🎉</div>
        <h2>Great progress!</h2>
        <p>You've completed the 5 most important categories. Would you like to continue with additional categories?</p>
        <div class="continue-actions">
          <button class="btn btn-secondary" on:click={finishSetup}>Finish & Review</button>
          <button class="btn btn-primary" on:click={continueWithMore}>Continue with More Categories</button>
        </div>
      </div>
    {:else if phase === 'complete'}
      <div class="complete-prompt">
        <div class="prompt-icon">✅</div>
        <h2>Setup Complete!</h2>
        <p>You've finished the guided setup. Your information has been saved automatically.</p>
        <p>You can always come back to add more details or update information.</p>
        <div class="complete-actions">
          <button class="btn btn-secondary" on:click={startOver}>Start Over</button>
          <button class="btn btn-primary" on:click={exitWizard}>Go to Full View</button>
        </div>
      </div>
    {:else}
      <WizardStep step={currentStep} currentStep={currentStepIndex + 1} totalSteps={totalSteps}>
        {#if currentStep.id === 'financial'}
          <FinancialSection />
        {:else if currentStep.id === 'insurance'}
          <InsuranceSection />
        {:else if currentStep.id === 'legal'}
          <LegalSection />
        {:else if currentStep.id === 'medical'}
          <MedicalSection />
        {:else if currentStep.id === 'contacts'}
          <ContactsSection />
        {:else if currentStep.id === 'bills'}
          <BillsSection />
        {:else if currentStep.id === 'property'}
          <PropertySection />
        {:else if currentStep.id === 'digital'}
          <DigitalSection />
        {:else if currentStep.id === 'household'}
          <HouseholdSection />
        {:else if currentStep.id === 'personal'}
          <PersonalSection />
        {:else if currentStep.id === 'pets'}
          <PetsSection />
        {/if}
      </WizardStep>
    {/if}
  </main>

  {#if phase !== 'askContinue' && phase !== 'complete'}
    <footer class="wizard-footer">
      <div class="footer-left">
        {#if currentStepIndex > 0}
          <button class="btn btn-secondary" on:click={back}>← Back</button>
        {/if}
      </div>
      <div class="footer-right">
        <button class="skip-link" on:click={skip}>Skip this section</button>
        <button class="btn btn-primary" on:click={next}>
          {currentStepIndex === steps.length - 1 ? (phase === 'priority' ? 'Continue' : 'Finish') : 'Next →'}
        </button>
      </div>
    </footer>
  {/if}
</div>

<style>
  .wizard {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: #f5f5f5;
  }

  .wizard-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 24px;
    background: white;
    border-bottom: 1px solid #e0e0e0;
  }

  .wizard-header h1 {
    margin: 0;
    font-size: 1.25rem;
    color: #333;
  }

  .exit-link {
    background: none;
    border: none;
    color: #1976d2;
    cursor: pointer;
    font-size: 0.9rem;
  }

  .exit-link:hover {
    text-decoration: underline;
  }

  .wizard-content {
    flex: 1;
    overflow-y: auto;
    padding: 24px;
  }

  .wizard-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 24px;
    background: white;
    border-top: 1px solid #e0e0e0;
  }

  .footer-left, .footer-right {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .skip-link {
    background: none;
    border: none;
    color: #666;
    cursor: pointer;
    font-size: 0.9rem;
  }

  .skip-link:hover {
    color: #333;
  }

  .btn {
    padding: 10px 20px;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.95rem;
  }

  .btn-primary {
    background: #1976d2;
    color: white;
  }

  .btn-primary:hover {
    background: #1565c0;
  }

  .btn-secondary {
    background: #e0e0e0;
    color: #333;
  }

  .btn-secondary:hover {
    background: #d0d0d0;
  }

  .continue-prompt, .complete-prompt {
    text-align: center;
    max-width: 500px;
    margin: 60px auto;
    padding: 40px;
    background: white;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  }

  .prompt-icon {
    font-size: 4rem;
    margin-bottom: 16px;
  }

  .continue-prompt h2, .complete-prompt h2 {
    margin: 0 0 16px 0;
    color: #333;
  }

  .continue-prompt p, .complete-prompt p {
    margin: 0 0 24px 0;
    color: #666;
    line-height: 1.5;
  }

  .continue-actions, .complete-actions {
    display: flex;
    justify-content: center;
    gap: 12px;
  }
</style>
```

**Step 2: Commit**
```bash
git add src/lib/wizard/GuidedWizard.svelte
git commit -m "feat: add GuidedWizard component with navigation and phase flow"
```

---

## Task 7: Add isDocumentEmpty Helper to Store

**Files:**
- Modify: `src/lib/stores/document.ts`

**Step 1: Add helper function after the store definition (after line 134)**

```typescript
export function isDocumentEmpty(doc: LegacyDocument | null): boolean {
  if (!doc) return true;

  // Check if any section has data
  const hasFinancial = doc.financial.bank_accounts.length > 0 ||
    doc.financial.credit_cards.length > 0 ||
    doc.financial.investments.length > 0 ||
    doc.financial.debts.length > 0;

  const hasInsurance = doc.insurance.policies.length > 0;
  const hasBills = doc.bills.bills.length > 0;
  const hasProperty = doc.property.properties.length > 0 ||
    doc.property.vehicles.length > 0 ||
    doc.property.valuables.length > 0;
  const hasLegal = doc.legal.trusts.length > 0 ||
    !!doc.legal.will_location ||
    !!doc.legal.power_of_attorney;
  const hasDigital = doc.digital.email_accounts.length > 0 ||
    doc.digital.social_media.length > 0;
  const hasHousehold = doc.household.maintenance_items.length > 0 ||
    doc.household.contractors.length > 0 ||
    doc.household.how_things_work.length > 0;
  const hasPersonal = doc.personal.messages.length > 0 ||
    !!doc.personal.funeral_preferences ||
    !!doc.personal.obituary_notes;
  const hasContacts = doc.contacts.emergency_contacts.length > 0 ||
    doc.contacts.family.length > 0 ||
    doc.contacts.professionals.length > 0;
  const hasMedical = doc.medical.family_members.length > 0;
  const hasPets = doc.pets.pets.length > 0;

  return !hasFinancial && !hasInsurance && !hasBills && !hasProperty &&
    !hasLegal && !hasDigital && !hasHousehold && !hasPersonal &&
    !hasContacts && !hasMedical && !hasPets;
}
```

**Step 2: Commit**
```bash
git add src/lib/stores/document.ts
git commit -m "feat: add isDocumentEmpty helper function"
```

---

## Task 8: Integrate Wizard into App.svelte

**Files:**
- Modify: `src/App.svelte`

**Step 1: Add imports at top of script (after line 16)**

```typescript
import GuidedWizard from './lib/wizard/GuidedWizard.svelte';
import { isDocumentEmpty } from './lib/stores/document';
```

**Step 2: Add wizard state (after line 24)**

```typescript
let isGuidedMode = false;
let hasCheckedEmpty = false;
```

**Step 3: Modify onMount to check for empty document (replace lines 45-47)**

```typescript
  onMount(async () => {
    await document.load();
    // Auto-enter guided mode for empty documents
    if (!hasCheckedEmpty) {
      hasCheckedEmpty = true;
      // Use setTimeout to check after store is populated
      setTimeout(() => {
        const doc = $document;
        if (isDocumentEmpty(doc)) {
          isGuidedMode = true;
        }
      }, 100);
    }
  });
```

**Step 4: Add function to toggle wizard mode (after onMount)**

```typescript
  function enterGuidedMode() {
    isGuidedMode = true;
  }

  function exitGuidedMode() {
    isGuidedMode = false;
  }
```

**Step 5: Add Guided Setup button to sidebar-footer (after line 73, before Export button)**

```svelte
      <button class="btn btn-outline" on:click={enterGuidedMode}>
        Guided Setup
      </button>
```

**Step 6: Wrap main content with wizard conditional (modify lines 50-124)**

Replace the entire `<main class="app">` block with:

```svelte
{#if isGuidedMode}
  <GuidedWizard on:exit={exitGuidedMode} />
{:else}
  <main class="app">
    <aside class="sidebar">
      <div class="logo">
        <h1>honey-did</h1>
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
            <span class="nav-status" data-status={getSectionStatus(section.id)}></span>
          </button>
        {/each}
      </nav>
      <div class="sidebar-footer">
        <button class="btn btn-outline" on:click={enterGuidedMode}>
          Guided Setup
        </button>
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
        <h2>{sections.find((s) => s.id === currentSection)?.label}</h2>
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
        {/if}
      </div>
    </section>
  </main>
{/if}
```

**Step 7: Add btn-outline style (add to style section)**

```css
  .btn-outline {
    background: white;
    color: #1976d2;
    border: 2px solid #1976d2;
  }

  .btn-outline:hover {
    background: #e3f2fd;
  }
```

**Step 8: Commit**
```bash
git add src/App.svelte
git commit -m "feat: integrate GuidedWizard with auto-start for empty documents"
```

---

## Task 9: Build and Verify

**Step 1: Build frontend**
```bash
cd /workspace/honey-did && npm run build
```

Expected: Build succeeds with no errors

**Step 2: Commit final**
```bash
git add -A
git commit -m "feat: complete save dialog and guided wizard implementation"
```

---

## Summary

**Feature 1 (Save Dialog):** Tasks 1-3
- Add tauri-plugin-dialog
- Create save_export_with_dialog command
- Update ExportDialog to use native dialog

**Feature 2 (Guided Wizard):** Tasks 4-8
- Create wizard content data
- Create WizardStep component
- Create GuidedWizard component
- Add isDocumentEmpty helper
- Integrate into App.svelte

**Task 9:** Final build verification
