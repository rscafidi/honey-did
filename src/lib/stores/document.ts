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

function createDocumentStore() {
  const { subscribe, set, update } = writable<LegacyDocument | null>(null);

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
          const updated = { ...doc, [section]: data };
          invoke('update_document', { document: updated }).catch(console.error);
          return updated;
        }
        return doc;
      });
    },
  };
}

export const document = createDocumentStore();
