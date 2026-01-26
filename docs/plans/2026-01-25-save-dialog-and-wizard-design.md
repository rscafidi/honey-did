# Save Dialog & Guided Wizard Design

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Improve export UX with native save dialog and add guided wizard mode for new users.

**Architecture:** Two independent features - (1) Replace filename field with Tauri native save dialog, (2) Add dual-mode interface with step-by-step wizard for guidance.

**Tech Stack:** Svelte 4, Tauri 2.0 (tauri-plugin-dialog), TypeScript

---

## Feature 1: Interactive File Save Dialog

### Overview

Replace the current filename text field with Tauri's native save dialog. When user clicks "Export File", a system save dialog opens with a suggested filename including the current date.

### Changes to ExportDialog.svelte

- Remove the `fileName` state variable and input field
- Remove the "File will be saved to your current directory" hint
- Keep passphrase fields, strength meter, and print checkbox as-is

### Changes to Rust backend

- Modify `save_export` command to use `tauri_plugin_dialog::DialogExt`
- Generate default filename with date: `honey-did-YYYY-MM-DD.html`
- Show native save dialog with `.html` filter
- If user cancels dialog, return without error (no-op)
- If user confirms, write encrypted HTML to chosen path

### User flow

1. User fills passphrase and clicks "Export File"
2. Native save dialog opens with `honey-did-2026-01-25.html` suggested
3. User picks location, optionally renames, clicks Save
4. File is written, success toast shown

---

## Feature 2: Guided Wizard Mode

### Overview

A step-by-step wizard that guides new users through document creation with explanations, examples, and thoughtful prompts. The existing full view remains available for returning users.

### Entry Points

- Empty document on launch → automatically enters guided mode
- "Guided Setup" button in main toolbar → enters guided mode anytime
- Wizard "Exit to Full View" link → leaves wizard, shows full interface

### Category Prioritization

**Priority categories (Phase 1):**
1. Financial - bank accounts, investments, debts
2. Insurance - policies and coverage
3. Legal - wills, power of attorney, trusts
4. Medical - conditions, medications, providers, directives
5. Contacts - key people to reach

**Secondary categories (Phase 2, opt-in):**
- Bills, Property, Digital Life, Household, Personal, Pets

### Wizard Navigation

- "Next" button advances to next category
- "Skip" link bypasses current category
- "Back" button returns to previous
- Progress indicator shows current position (e.g., "Step 2 of 5")
- After step 5: "Continue with more categories?" or "Finish & Review"

### Wizard Step Layout

```
[Progress: Step 2 of 5 - Insurance]

Why this matters:
"Insurance policies provide financial protection for your family.
Without this information, claims could be delayed or missed entirely."

[Form fields for the category - same as full view]

Things to consider:
- Do you have life insurance through your employer?
- Are there any policies with cash value that could be borrowed against?
- Who are the beneficiaries on each policy?

Example entry:
"Term Life - Northwestern Mutual - Policy #12345 - $500k coverage - Spouse is beneficiary"

[Back]                    [Skip this section]  [Next]
```

### Sample Prompts by Category

**Financial:**
- Any accounts your partner doesn't know about?
- Safety deposit box?
- Cryptocurrency or digital assets?

**Insurance:**
- Life insurance through employer?
- Policies with cash value?

**Legal:**
- Where are original documents stored?
- Who is your estate attorney?

**Medical:**
- Allergies or drug interactions?
- Advance directive preferences?

**Contacts:**
- Who should be called first?
- Anyone who should NOT be contacted?

### Technical Implementation

**New components:**
- `GuidedWizard.svelte` - Main wizard container with navigation and progress
- `WizardStep.svelte` - Reusable step wrapper with explanation/prompts layout
- `wizardContent.ts` - Data file with all explanations, examples, and prompts per category

**State management:**
- Add `isGuidedMode: boolean` to app state
- Add `wizardStep: number` to track position
- Reuse existing category form components inside wizard steps (no duplication)

**Integration with existing forms:**
- Extract current category forms into standalone components if not already
- Wizard wraps same form components with guidance UI
- Data saves to same store - no separate wizard data model

**App.svelte changes:**
- Check if document is empty on mount → set `isGuidedMode = true`
- Add "Guided Setup" button to toolbar
- Conditionally render `<GuidedWizard />` or current full view based on mode

**Persistence:**
- Wizard progress auto-saves to same local storage as full view
- If user exits wizard mid-way, data is preserved
- Re-entering wizard resumes from where they left off (or restarts if they choose)

---

## Success Criteria

1. Export shows native save dialog with date-based filename
2. New users (empty document) see guided wizard on first launch
3. Wizard walks through 5 priority categories with explanations and prompts
4. Users can opt-in to 6 additional categories or finish early
5. "Guided Setup" button available in toolbar for returning users
6. All data entered in wizard persists to same storage as full view
