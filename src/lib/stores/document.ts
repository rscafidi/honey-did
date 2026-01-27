import { writable } from 'svelte/store';
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
  field_definitions: FieldDefinition[];
  items: CustomItem[];
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

function createDocumentStore() {
  const { subscribe, set, update } = writable<LegacyDocument | null>(null);

  // Debounce save operations to prevent lag when typing
  let saveTimeout: ReturnType<typeof setTimeout> | null = null;
  let pendingDocument: LegacyDocument | null = null;

  const debouncedSave = (doc: LegacyDocument) => {
    pendingDocument = doc;
    if (saveTimeout) {
      clearTimeout(saveTimeout);
    }
    saveTimeout = setTimeout(() => {
      if (pendingDocument) {
        invoke('update_document', { document: pendingDocument }).catch(console.error);
        pendingDocument = null;
      }
      saveTimeout = null;
    }, 300); // Save 300ms after last change
  };

  return {
    subscribe,
    load: async () => {
      try {
        const doc = await invoke<LegacyDocument>('get_document');
        set(doc);
      } catch (e) {
        console.error('Failed to load document:', e);
      }
    },
    save: async (doc: LegacyDocument) => {
      // Immediate save - cancel any pending debounced save
      if (saveTimeout) {
        clearTimeout(saveTimeout);
        saveTimeout = null;
        pendingDocument = null;
      }
      try {
        await invoke('update_document', { document: doc });
        set(doc);
      } catch (e) {
        console.error('Failed to save document:', e);
      }
    },
    updateSection: async <K extends keyof LegacyDocument>(
      section: K,
      data: LegacyDocument[K]
    ) => {
      update((doc) => {
        if (doc) {
          const wasEmpty = isDocumentEmpty(doc);
          const updated = { ...doc, [section]: data };
          const nowHasData = !isDocumentEmpty(updated);

          // Trigger password prompt if transitioning from empty to having data
          if (wasEmpty && nowHasData && passwordRequiredCallback) {
            passwordRequiredCallback();
          }

          // Debounce the save to prevent lag when typing
          debouncedSave(updated);
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
