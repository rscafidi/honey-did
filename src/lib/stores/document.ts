import { writable, type Readable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface LegacyDocument {
  meta: DocumentMeta;
  financial: FinancialSection;
  insurance: InsuranceSection;
  bills: BillsSection;
  property: PropertySection;
  legal: LegalSection;
  digital: DigitalSection;
  household: HouseholdSection;
  personal: PersonalSection;
  contacts: ContactsSection;
  medical: MedicalSection;
  pets: PetsSection;
  welcome_screen?: WelcomeScreen;
  custom_sections?: CustomSection[];
}

export interface DocumentMeta {
  creator_name: string;
  created_at: string;
  updated_at: string;
}

// Simplified interfaces - full types match Rust models
export interface FinancialSection {
  bank_accounts: any[];
  credit_cards: any[];
  investments: any[];
  debts: any[];
  notes: string;
}

export interface InsuranceSection {
  policies: any[];
  notes: string;
}

export interface BillsSection {
  bills: any[];
  notes: string;
}

export interface PropertySection {
  properties: any[];
  vehicles: any[];
  valuables: any[];
  notes: string;
}

export interface LegalSection {
  will_location: string;
  attorney: any;
  power_of_attorney: string;
  trusts: any[];
  notes: string;
}

export interface DigitalSection {
  email_accounts: any[];
  social_media: any[];
  password_manager: any;
  notes: string;
}

export interface HouseholdSection {
  maintenance_items: any[];
  contractors: any[];
  how_things_work: any[];
  notes: string;
}

export interface PersonalSection {
  funeral_preferences: string;
  obituary_notes: string;
  messages: any[];
  notes: string;
}

export interface ContactsSection {
  emergency_contacts: any[];
  family: any[];
  professionals: any[];
  notes: string;
}

export interface MedicalSection {
  family_members: any[];
  notes: string;
}

export interface PetsSection {
  pets: any[];
  notes: string;
}

export type SlideType = 'message' | 'question';

export interface MessageSlide {
  id: string;
  type: SlideType;
  text: string;
  answer?: string;
  transition: { type: 'click' } | { type: 'auto'; seconds: number };
}

export interface WelcomeScreen {
  enabled: boolean;
  slides: MessageSlide[];
  fallback_passphrase?: string;
}

// Custom Sections
export interface CustomSection {
  id: string;
  name: string;
  parent?: string;  // undefined = top-level, "financial" = subsection of financial
  subsections: CustomSubsection[];
}

export interface CustomSubsection {
  id: string;
  name: string;
  form_elements: FormElement[];
  field_definitions?: FieldDefinition[];  // legacy, kept for migration
  items: CustomItem[];
}

// Discriminated union for form elements
export type FormElement = FormElementField | FormElementDivider | FormElementHeader;

export interface FormElementField {
  type: 'field';
  id: string;
  name: string;
  field_type: FieldType;
}

export interface FormElementDivider {
  type: 'divider';
  id: string;
}

export interface FormElementHeader {
  type: 'header';
  id: string;
  text: string;
}

export interface FieldDefinition {
  id: string;
  name: string;
  field_type: FieldType;
}

export type FieldType = 'text' | 'number' | 'date' | 'boolean';

export interface CustomItem {
  id: string;
  values: Record<string, string>;  // field_id -> value
}

/** Extract only field elements from form_elements */
export function getFieldElements(elements: FormElement[]): FormElementField[] {
  return elements.filter((el): el is FormElementField => el.type === 'field');
}

/** Migrate a subsection from old field_definitions to form_elements */
export function migrateSubsection(sub: CustomSubsection): CustomSubsection {
  if (sub.form_elements && sub.form_elements.length > 0) return sub;
  if (!sub.field_definitions || sub.field_definitions.length === 0) {
    return { ...sub, form_elements: sub.form_elements || [] };
  }
  // Convert legacy field_definitions into FormElementField entries
  const form_elements: FormElement[] = sub.field_definitions.map((fd) => ({
    type: 'field' as const,
    id: fd.id,
    name: fd.name,
    field_type: fd.field_type,
  }));
  return { ...sub, form_elements };
}

/** Create a debounced version of a function */
export function debounce<T extends (...args: any[]) => void>(fn: T, ms: number): T {
  let timeout: ReturnType<typeof setTimeout> | null = null;
  return ((...args: any[]) => {
    if (timeout) clearTimeout(timeout);
    timeout = setTimeout(() => fn(...args), ms);
  }) as T;
}

function createDocumentStore() {
  const { subscribe, set, update } = writable<LegacyDocument | null>(null);

  // Cache whether document has data to avoid expensive isDocumentEmpty on every keystroke
  let hasDataCached = false;
  // Dirty flag: true when in-memory state differs from what's on disk
  let isDirty = false;
  // Safety-net idle timer: saves after 30s of no edits (crash protection)
  let idleSaveTimer: ReturnType<typeof setTimeout> | null = null;
  // Prevent concurrent saves
  let isSaving = false;

  function scheduleIdleSave() {
    if (idleSaveTimer) clearTimeout(idleSaveTimer);
    idleSaveTimer = setTimeout(() => {
      idleSaveTimer = null;
      saveToDisk();
    }, 30_000); // 30s after last edit
  }

  /** Persist the current in-memory document to disk via Tauri.
   *  Called on lifecycle events (blur, visibility change, section switch, app close).
   *  Safe to call multiple times — no-ops if nothing is dirty. */
  function saveToDisk(): Promise<void> {
    if (!isDirty || isSaving) return Promise.resolve();
    let currentDoc: LegacyDocument | null = null;
    // Read current value synchronously from store
    const unsub = subscribe((doc) => { currentDoc = doc; });
    unsub();
    if (!currentDoc) return Promise.resolve();
    isDirty = false;
    isSaving = true;
    if (idleSaveTimer) { clearTimeout(idleSaveTimer); idleSaveTimer = null; }
    return invoke('update_document', { document: currentDoc })
      .catch((e) => {
        console.error('Failed to save document:', e);
        // Re-mark dirty so it gets retried on next lifecycle event
        isDirty = true;
      })
      .finally(() => { isSaving = false; });
  }

  return {
    subscribe,
    /** Persist to disk if there are unsaved changes. */
    saveToDisk,
    /** Whether the in-memory document has unsaved changes. */
    get dirty() { return isDirty; },
    load: async () => {
      try {
        const doc = await invoke<LegacyDocument>('get_document');
        // Migrate any legacy custom subsections to form_elements
        if (doc.custom_sections) {
          doc.custom_sections = doc.custom_sections.map((section) => ({
            ...section,
            subsections: section.subsections.map(migrateSubsection),
          }));
        }
        hasDataCached = !isDocumentEmpty(doc);
        isDirty = false;
        set(doc);
      } catch (e) {
        console.error('Failed to load document:', e);
      }
    },
    save: async (doc: LegacyDocument) => {
      // Immediate save (used by import, etc.)
      if (idleSaveTimer) { clearTimeout(idleSaveTimer); idleSaveTimer = null; }
      try {
        await invoke('update_document', { document: doc });
        isDirty = false;
        set(doc);
      } catch (e) {
        console.error('Failed to save document:', e);
      }
    },
    updateSection: <K extends keyof LegacyDocument>(
      section: K,
      data: LegacyDocument[K]
    ) => {
      update((doc) => {
        if (doc) {
          const updated = { ...doc, [section]: data };

          // Only check isDocumentEmpty when we haven't cached that data exists yet
          if (!hasDataCached) {
            const nowHasData = !isDocumentEmpty(updated);
            if (nowHasData) {
              hasDataCached = true;
              if (passwordRequiredCallback) {
                passwordRequiredCallback();
              }
            }
          }

          isDirty = true;
          scheduleIdleSave();
          return updated;
        }
        return doc;
      });
    },
  };
}

export const document = createDocumentStore();

// Password requirement callback - called when user tries to modify data
let passwordRequiredCallback: (() => void) | null = null;

export function setPasswordRequired(callback: () => void): void {
  passwordRequiredCallback = callback;
}

export function triggerPasswordRequired(): void {
  if (passwordRequiredCallback) {
    passwordRequiredCallback();
  }
}

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
  const hasCustomSections = (doc.custom_sections || []).some(section =>
    section.subsections.some(sub => sub.items.length > 0)
  );

  return !hasFinancial && !hasInsurance && !hasBills && !hasProperty &&
    !hasLegal && !hasDigital && !hasHousehold && !hasPersonal &&
    !hasContacts && !hasMedical && !hasPets && !hasCustomSections;
}

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
