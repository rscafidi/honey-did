# Form Performance Optimization

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Eliminate form stutter and input delay by splitting the monolithic document store into per-section stores and adding local-first editing to built-in section components.

**Architecture:** Two complementary changes: (1) Per-section derived stores that use reference equality to skip notifications when unrelated sections change, preventing unnecessary reactive cascades across unrelated components. (2) Local-first editing in built-in sections so field changes stay in component-local state and only flush to the store on discrete actions (add/remove) or debounced intervals, matching the pattern already used by CustomSectionEditor.

**Tech Stack:** Svelte 4 writable/derived stores, TypeScript

---

### Task 1: Add per-section stores to document.ts

**Files:**
- Modify: `src/lib/stores/document.ts`

**What to do:**

Add a `sectionStore` factory function and pre-built section stores at the bottom of `document.ts` (before the closing of the file, after the `isDocumentEmpty` function).

The factory creates a `Readable` store that subscribes to the main `document` store but only notifies its own subscribers when the specific section's value changes by reference. Since `updateSection` does `{ ...doc, [section]: data }`, only the changed section gets a new reference — all other sections keep their original references, so `!==` correctly filters out no-op updates.

Add this code after the `isDocumentEmpty` function (after line 324):

```typescript
import { writable, type Readable } from 'svelte/store';

// (at top of file, update the existing import to include Readable)

/** Creates a store that only emits when a specific section's reference changes */
function createSectionStore<K extends keyof LegacyDocument>(
  key: K,
  defaultValue: LegacyDocument[K]
): Readable<LegacyDocument[K]> {
  let currentValue: LegacyDocument[K] = defaultValue;
  const { subscribe, set } = writable<LegacyDocument[K]>(defaultValue);

  document.subscribe(($doc) => {
    const newValue = $doc?.[key] ?? defaultValue;
    if (newValue !== currentValue) {
      currentValue = newValue;
      set(newValue);
    }
  });

  return { subscribe };
}

// Pre-built section stores — components import these instead of subscribing to $document directly
export const financialStore = createSectionStore('financial', { bank_accounts: [], credit_cards: [], investments: [], debts: [], notes: '' });
export const insuranceStore = createSectionStore('insurance', { policies: [], notes: '' });
export const billsStore = createSectionStore('bills', { bills: [], notes: '' });
export const propertyStore = createSectionStore('property', { properties: [], vehicles: [], valuables: [], notes: '' });
export const legalStore = createSectionStore('legal', { will_location: '', attorney: { name: '', relationship: '', phone: '', email: '', notes: '' }, power_of_attorney: '', trusts: [], notes: '' });
export const digitalStore = createSectionStore('digital', { email_accounts: [], social_media: [], password_manager: { name: '', master_password_hint: '', recovery_method: '', notes: '' }, notes: '' });
export const householdStore = createSectionStore('household', { maintenance_items: [], contractors: [], how_things_work: [], notes: '' });
export const personalStore = createSectionStore('personal', { funeral_preferences: '', obituary_notes: '', messages: [], notes: '' });
export const contactsStore = createSectionStore('contacts', { emergency_contacts: [], family: [], professionals: [], notes: '' });
export const medicalStore = createSectionStore('medical', { family_members: [], notes: '' });
export const petsStore = createSectionStore('pets', { pets: [], notes: '' });
export const customSectionsStore = createSectionStore('custom_sections', []);
```

**Important:** The `import { writable }` at line 1 must be updated to `import { writable, type Readable } from 'svelte/store'`.

**Commit:** `feat: add per-section stores with reference-equality filtering`

---

### Task 2: Update App.svelte to use customSectionsStore

**Files:**
- Modify: `src/App.svelte`

**What to do:**

1. Add `customSectionsStore` to the import from `../lib/stores/document`:
   ```typescript
   import { document, isDocumentEmpty, setPasswordRequired, type CustomSection, customSectionsStore } from './lib/stores/document';
   ```

2. Change the reactive declaration at line 68 from:
   ```typescript
   $: customTopLevelSections = ($document?.custom_sections || []).filter(s => !s.parent);
   ```
   to:
   ```typescript
   $: customTopLevelSections = ($customSectionsStore || []).filter((s: CustomSection) => !s.parent);
   ```

This means `customTopLevelSections` only recalculates when `custom_sections` actually changes, not when any built-in section is updated.

**Do NOT change** any of the handler functions (`addCustomSection`, `deleteCustomSection`, etc.) — they still use `$document` and `document.updateSection()` for writes, which is correct. Only the reactive read is changed.

**Commit:** `perf: use customSectionsStore in App.svelte to reduce reactive recalculations`

---

### Task 3: Convert FinancialSection to local-first editing (template)

**Files:**
- Modify: `src/lib/sections/FinancialSection.svelte`

**What to do:**

This is the template pattern. All other built-in sections will follow the same structure.

Replace the entire `<script>` block with:

```typescript
<script lang="ts">
  import { onDestroy } from 'svelte';
  import { document, financialStore } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';
  import CustomSubsections from '../components/CustomSubsections.svelte';

  const defaultFinancial = {
    bank_accounts: [] as any[],
    credit_cards: [] as any[],
    investments: [] as any[],
    debts: [] as any[],
    notes: ''
  };

  // Local-first state: edits stay here, only flushed to store on discrete actions or debounced
  let local = { ...defaultFinancial };
  let hasPendingChanges = false;
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  // Sync from store ONLY when we don't have pending local changes
  const unsub = financialStore.subscribe((value) => {
    if (!hasPendingChanges) {
      local = value ?? defaultFinancial;
    }
  });
  onDestroy(() => {
    // Flush any pending changes before unmount
    if (hasPendingChanges) {
      if (debounceTimer) clearTimeout(debounceTimer);
      document.updateSection('financial', local);
    }
    unsub();
  });

  function scheduleFlush() {
    hasPendingChanges = true;
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      document.updateSection('financial', local);
      setTimeout(() => { hasPendingChanges = false; }, 0);
      debounceTimer = null;
    }, 300);
  }

  function flushNow(updatedLocal: typeof local) {
    local = updatedLocal;
    if (debounceTimer) { clearTimeout(debounceTimer); debounceTimer = null; }
    hasPendingChanges = false;
    document.updateSection('financial', local);
  }

  // --- Bank Accounts ---
  function addBankAccount() {
    flushNow({
      ...local,
      bank_accounts: [...local.bank_accounts, {
        name: '', institution: '', account_type: 'Checking', last_four: '', notes: ''
      }]
    });
  }

  function removeBankAccount(index: number) {
    flushNow({
      ...local,
      bank_accounts: local.bank_accounts.filter((_: any, i: number) => i !== index)
    });
  }

  function updateBankAccount(index: number, field: string, value: string) {
    const accounts = [...local.bank_accounts];
    accounts[index] = { ...accounts[index], [field]: value };
    local = { ...local, bank_accounts: accounts };
    scheduleFlush();
  }

  // --- Credit Cards ---
  function addCreditCard() {
    flushNow({
      ...local,
      credit_cards: [...local.credit_cards, {
        name: '', issuer: '', last_four: '', notes: ''
      }]
    });
  }

  function removeCreditCard(index: number) {
    flushNow({
      ...local,
      credit_cards: local.credit_cards.filter((_: any, i: number) => i !== index)
    });
  }

  function updateCreditCard(index: number, field: string, value: string) {
    const cards = [...local.credit_cards];
    cards[index] = { ...cards[index], [field]: value };
    local = { ...local, credit_cards: cards };
    scheduleFlush();
  }

  // --- Investments ---
  function addInvestment() {
    flushNow({
      ...local,
      investments: [...local.investments, {
        name: '', institution: '', account_type: '', notes: ''
      }]
    });
  }

  function removeInvestment(index: number) {
    flushNow({
      ...local,
      investments: local.investments.filter((_: any, i: number) => i !== index)
    });
  }

  function updateInvestment(index: number, field: string, value: string) {
    const investments = [...local.investments];
    investments[index] = { ...investments[index], [field]: value };
    local = { ...local, investments };
    scheduleFlush();
  }

  // --- Debts ---
  function addDebt() {
    flushNow({
      ...local,
      debts: [...local.debts, {
        name: '', lender: '', notes: ''
      }]
    });
  }

  function removeDebt(index: number) {
    flushNow({
      ...local,
      debts: local.debts.filter((_: any, i: number) => i !== index)
    });
  }

  function updateDebt(index: number, field: string, value: string) {
    const debts = [...local.debts];
    debts[index] = { ...debts[index], [field]: value };
    local = { ...local, debts };
    scheduleFlush();
  }

  function updateNotes(e: Event) {
    const target = e.target as HTMLTextAreaElement;
    local = { ...local, notes: target.value };
    scheduleFlush();
  }
</script>
```

Then in the template, replace all references from `financial.` to `local.`:
- `{#each financial.bank_accounts as account, i}` → `{#each local.bank_accounts as account, i}`
- `{#each financial.credit_cards as card, i}` → `{#each local.credit_cards as card, i}`
- `{#each financial.investments as investment, i}` → `{#each local.investments as investment, i}`
- `{#each financial.debts as debt, i}` → `{#each local.debts as debt, i}`
- `<NotesField value={financial.notes}` → `<NotesField value={local.notes}`

**Commit:** `perf: convert FinancialSection to local-first editing with section store`

---

### Task 4: Convert remaining built-in sections to local-first editing

**Files:**
- Modify: `src/lib/sections/InsuranceSection.svelte`
- Modify: `src/lib/sections/BillsSection.svelte`
- Modify: `src/lib/sections/PropertySection.svelte`
- Modify: `src/lib/sections/LegalSection.svelte`
- Modify: `src/lib/sections/DigitalSection.svelte`
- Modify: `src/lib/sections/HouseholdSection.svelte`
- Modify: `src/lib/sections/PersonalSection.svelte`
- Modify: `src/lib/sections/ContactsSection.svelte`
- Modify: `src/lib/sections/MedicalSection.svelte`
- Modify: `src/lib/sections/PetsSection.svelte`

**Pattern:** Each section follows the exact same pattern as FinancialSection in Task 3:

1. Import `onDestroy` and the section-specific store (e.g., `insuranceStore`) instead of using `$document` reactively
2. Replace the reactive `$: sectionData = $document?.sectionKey ?? default` with:
   - A `local` variable initialized from the section store subscription
   - `hasPendingChanges` flag + `debounceTimer` for debounced flushing
   - `unsub` subscription with pending-change guard
   - `onDestroy` cleanup that flushes + unsubscribes
   - `scheduleFlush()` for field updates (debounced 300ms)
   - `flushNow()` for discrete actions (add/remove)
3. Change all update functions from `document.updateSection('key', { ...sectionData, ... })` to updating `local` and calling either `scheduleFlush()` (field edits) or `flushNow()` (add/remove)
4. In the template, replace all references from `sectionData.` to `local.`

**Section-specific notes:**

- **LegalSection**: Has `updateField()` and `updateAttorney()` helpers — both become `scheduleFlush()` calls
- **DigitalSection**: Has `updatePasswordManager()` helper — becomes `scheduleFlush()` call
- **MedicalSection**: Has nested doctors/medications arrays with custom `on:change` handlers — same pattern, just update `local` and `scheduleFlush()`
- **PetsSection**: Same nested pattern as Medical for medications
- **WelcomeScreenSection**: Already has its own debouncing — SKIP this file, leave it as-is

**Commit:** `perf: convert all built-in sections to local-first editing with section stores`

---

### Task 5: Update CustomSubsections to use customSectionsStore

**Files:**
- Modify: `src/lib/components/CustomSubsections.svelte`

**What to do:**

1. Import `customSectionsStore` from the store:
   ```typescript
   import { document, customSectionsStore, type CustomSection, type CustomSubsection } from '../stores/document';
   ```

2. Change the reactive declarations from:
   ```typescript
   $: customSection = ($document?.custom_sections || []).find(s => s.parent === parentId);
   ```
   to:
   ```typescript
   $: customSection = ($customSectionsStore || []).find((s: CustomSection) => s.parent === parentId);
   ```

3. Leave all the write operations using `$document` and `document.updateSection()` as-is — they need the full document for writes.

**Commit:** `perf: use customSectionsStore in CustomSubsections to reduce reactive recalculations`

---

### Task 6: Build verification

**Run:** `cd /workspace/honey-did && npm run build`

Verify the build succeeds with no TypeScript errors. Fix any type issues that arise.

**Commit:** (only if fixes needed) `fix: resolve type errors from performance refactor`
