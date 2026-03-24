import { describe, it, expect, vi, beforeEach } from 'vitest';
import {
  isDocumentEmpty,
  getFieldElements,
  migrateSubsection,
  debounce,
  type LegacyDocument,
  type CustomSubsection,
  type FormElement,
  type FormElementField,
} from './document';

// --- Helper to create an empty document ---
function createEmptyDoc(): LegacyDocument {
  return {
    meta: { creator_name: '', created_at: '', updated_at: '' },
    financial: { bank_accounts: [], credit_cards: [], investments: [], debts: [], notes: '' },
    insurance: { policies: [], notes: '' },
    bills: { bills: [], notes: '' },
    property: { properties: [], vehicles: [], valuables: [], notes: '' },
    legal: { will_location: '', attorney: { name: '', relationship: '', phone: '', email: '', notes: '' }, power_of_attorney: '', trusts: [], notes: '' },
    digital: { email_accounts: [], social_media: [], password_manager: { name: '', master_password_hint: '', recovery_method: '', notes: '' }, notes: '' },
    household: { maintenance_items: [], contractors: [], how_things_work: [], notes: '' },
    personal: { funeral_preferences: '', obituary_notes: '', messages: [], notes: '' },
    contacts: { emergency_contacts: [], family: [], professionals: [], notes: '' },
    medical: { family_members: [], notes: '' },
    pets: { pets: [], notes: '' },
    custom_sections: [],
  };
}

// ==============================
// isDocumentEmpty
// ==============================
describe('isDocumentEmpty', () => {
  it('returns true for null', () => {
    expect(isDocumentEmpty(null)).toBe(true);
  });

  it('returns true for a fresh empty document', () => {
    expect(isDocumentEmpty(createEmptyDoc())).toBe(true);
  });

  it('detects bank accounts', () => {
    const doc = createEmptyDoc();
    doc.financial.bank_accounts.push({ name: 'Checking', institution: 'Bank' });
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects credit cards', () => {
    const doc = createEmptyDoc();
    doc.financial.credit_cards.push({ name: 'Visa' });
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects investments', () => {
    const doc = createEmptyDoc();
    doc.financial.investments.push({ name: '401k' });
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects debts', () => {
    const doc = createEmptyDoc();
    doc.financial.debts.push({ name: 'Mortgage' });
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects insurance policies', () => {
    const doc = createEmptyDoc();
    doc.insurance.policies.push({ policy_type: 'Life' });
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects bills', () => {
    const doc = createEmptyDoc();
    doc.bills.bills.push({ name: 'Electric' });
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects properties', () => {
    const doc = createEmptyDoc();
    doc.property.properties.push({ name: 'Home' });
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects vehicles', () => {
    const doc = createEmptyDoc();
    doc.property.vehicles.push({ name: 'Car' });
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects valuables', () => {
    const doc = createEmptyDoc();
    doc.property.valuables.push({ name: 'Ring' });
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects will location', () => {
    const doc = createEmptyDoc();
    doc.legal.will_location = 'Safe deposit box';
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects power of attorney', () => {
    const doc = createEmptyDoc();
    doc.legal.power_of_attorney = 'John Doe';
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects trusts', () => {
    const doc = createEmptyDoc();
    doc.legal.trusts.push({ name: 'Family Trust' });
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects email accounts', () => {
    const doc = createEmptyDoc();
    doc.digital.email_accounts.push({ name: 'Gmail' });
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects social media', () => {
    const doc = createEmptyDoc();
    doc.digital.social_media.push({ name: 'Twitter' });
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects maintenance items', () => {
    const doc = createEmptyDoc();
    doc.household.maintenance_items.push({ name: 'HVAC Filter' });
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects contractors', () => {
    const doc = createEmptyDoc();
    doc.household.contractors.push({ name: 'Plumber' });
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects how things work', () => {
    const doc = createEmptyDoc();
    doc.household.how_things_work.push({ name: 'Sprinkler' });
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects funeral preferences', () => {
    const doc = createEmptyDoc();
    doc.personal.funeral_preferences = 'Cremation';
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects obituary notes', () => {
    const doc = createEmptyDoc();
    doc.personal.obituary_notes = 'Some notes';
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects messages', () => {
    const doc = createEmptyDoc();
    doc.personal.messages.push({ recipient: 'Jane', message: 'Hello' });
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects emergency contacts', () => {
    const doc = createEmptyDoc();
    doc.contacts.emergency_contacts.push({ name: 'Mom' });
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects family contacts', () => {
    const doc = createEmptyDoc();
    doc.contacts.family.push({ name: 'Dad' });
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects professional contacts', () => {
    const doc = createEmptyDoc();
    doc.contacts.professionals.push({ name: 'Lawyer' });
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects family members (medical)', () => {
    const doc = createEmptyDoc();
    doc.medical.family_members.push({ name: 'Self' });
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects pets', () => {
    const doc = createEmptyDoc();
    doc.pets.pets.push({ name: 'Buddy' });
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('detects custom sections with items', () => {
    const doc = createEmptyDoc();
    doc.custom_sections = [{
      id: '1',
      name: 'Test',
      subsections: [{
        id: 's1',
        name: 'Sub',
        form_elements: [],
        items: [{ id: 'i1', values: { foo: 'bar' } }],
      }],
    }];
    expect(isDocumentEmpty(doc)).toBe(false);
  });

  it('treats custom sections without items as empty', () => {
    const doc = createEmptyDoc();
    doc.custom_sections = [{
      id: '1',
      name: 'Test',
      subsections: [{
        id: 's1',
        name: 'Sub',
        form_elements: [],
        items: [],
      }],
    }];
    expect(isDocumentEmpty(doc)).toBe(true);
  });

  it('ignores notes-only sections', () => {
    const doc = createEmptyDoc();
    doc.financial.notes = 'Some notes but no items';
    doc.insurance.notes = 'More notes';
    expect(isDocumentEmpty(doc)).toBe(true);
  });
});

// ==============================
// getFieldElements
// ==============================
describe('getFieldElements', () => {
  it('returns empty array for empty input', () => {
    expect(getFieldElements([])).toEqual([]);
  });

  it('filters out dividers and headers', () => {
    const elements: FormElement[] = [
      { type: 'field', id: '1', name: 'Name', field_type: 'text' },
      { type: 'divider', id: '2' },
      { type: 'header', id: '3', text: 'Section' },
      { type: 'field', id: '4', name: 'Age', field_type: 'number' },
    ];
    const fields = getFieldElements(elements);
    expect(fields).toHaveLength(2);
    expect(fields[0].name).toBe('Name');
    expect(fields[1].name).toBe('Age');
  });

  it('returns all elements when all are fields', () => {
    const elements: FormElementField[] = [
      { type: 'field', id: '1', name: 'A', field_type: 'text' },
      { type: 'field', id: '2', name: 'B', field_type: 'boolean' },
      { type: 'field', id: '3', name: 'C', field_type: 'date' },
    ];
    expect(getFieldElements(elements)).toHaveLength(3);
  });

  it('preserves field_type information', () => {
    const elements: FormElement[] = [
      { type: 'field', id: '1', name: 'Toggle', field_type: 'boolean' },
      { type: 'field', id: '2', name: 'When', field_type: 'date' },
      { type: 'field', id: '3', name: 'Count', field_type: 'number' },
      { type: 'field', id: '4', name: 'Desc', field_type: 'text' },
    ];
    const fields = getFieldElements(elements);
    expect(fields.map((f) => f.field_type)).toEqual(['boolean', 'date', 'number', 'text']);
  });
});

// ==============================
// migrateSubsection
// ==============================
describe('migrateSubsection', () => {
  it('returns unchanged subsection if form_elements already populated', () => {
    const sub: CustomSubsection = {
      id: '1',
      name: 'Test',
      form_elements: [{ type: 'field', id: 'f1', name: 'Name', field_type: 'text' }],
      items: [],
    };
    const result = migrateSubsection(sub);
    expect(result.form_elements).toEqual(sub.form_elements);
  });

  it('converts legacy field_definitions to form_elements', () => {
    const sub: CustomSubsection = {
      id: '1',
      name: 'Test',
      form_elements: [],
      field_definitions: [
        { id: 'f1', name: 'Name', field_type: 'text' },
        { id: 'f2', name: 'Count', field_type: 'number' },
      ],
      items: [],
    };
    const result = migrateSubsection(sub);
    expect(result.form_elements).toHaveLength(2);
    expect(result.form_elements[0]).toEqual({
      type: 'field',
      id: 'f1',
      name: 'Name',
      field_type: 'text',
    });
    expect(result.form_elements[1]).toEqual({
      type: 'field',
      id: 'f2',
      name: 'Count',
      field_type: 'number',
    });
  });

  it('returns empty form_elements when both are empty', () => {
    const sub: CustomSubsection = {
      id: '1',
      name: 'Test',
      form_elements: [],
      field_definitions: [],
      items: [],
    };
    const result = migrateSubsection(sub);
    expect(result.form_elements).toEqual([]);
  });

  it('handles missing field_definitions gracefully', () => {
    const sub: CustomSubsection = {
      id: '1',
      name: 'Test',
      form_elements: [],
      items: [],
    };
    const result = migrateSubsection(sub);
    expect(result.form_elements).toEqual([]);
  });

  it('preserves all field types during migration', () => {
    const sub: CustomSubsection = {
      id: '1',
      name: 'Test',
      form_elements: [],
      field_definitions: [
        { id: 'f1', name: 'Text', field_type: 'text' },
        { id: 'f2', name: 'Num', field_type: 'number' },
        { id: 'f3', name: 'Date', field_type: 'date' },
        { id: 'f4', name: 'Bool', field_type: 'boolean' },
      ],
      items: [],
    };
    const result = migrateSubsection(sub);
    const types = result.form_elements
      .filter((el): el is FormElementField => el.type === 'field')
      .map((el) => el.field_type);
    expect(types).toEqual(['text', 'number', 'date', 'boolean']);
  });

  it('preserves items during migration', () => {
    const sub: CustomSubsection = {
      id: '1',
      name: 'Test',
      form_elements: [],
      field_definitions: [{ id: 'f1', name: 'Name', field_type: 'text' }],
      items: [{ id: 'i1', values: { f1: 'hello' } }],
    };
    const result = migrateSubsection(sub);
    expect(result.items).toEqual(sub.items);
  });
});

// ==============================
// debounce
// ==============================
describe('debounce', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  it('calls the function after the delay', () => {
    const fn = vi.fn();
    const debounced = debounce(fn, 100);
    debounced();
    expect(fn).not.toHaveBeenCalled();
    vi.advanceTimersByTime(100);
    expect(fn).toHaveBeenCalledOnce();
  });

  it('resets timer on subsequent calls', () => {
    const fn = vi.fn();
    const debounced = debounce(fn, 100);
    debounced();
    vi.advanceTimersByTime(50);
    debounced(); // reset
    vi.advanceTimersByTime(50);
    expect(fn).not.toHaveBeenCalled();
    vi.advanceTimersByTime(50);
    expect(fn).toHaveBeenCalledOnce();
  });

  it('passes arguments to the debounced function', () => {
    const fn = vi.fn();
    const debounced = debounce(fn, 50);
    debounced('a', 'b');
    vi.advanceTimersByTime(50);
    expect(fn).toHaveBeenCalledWith('a', 'b');
  });

  it('only fires once for rapid calls', () => {
    const fn = vi.fn();
    const debounced = debounce(fn, 100);
    for (let i = 0; i < 10; i++) {
      debounced();
    }
    vi.advanceTimersByTime(100);
    expect(fn).toHaveBeenCalledOnce();
  });

  it('fires again after delay for separate bursts', () => {
    const fn = vi.fn();
    const debounced = debounce(fn, 100);
    debounced();
    vi.advanceTimersByTime(100);
    expect(fn).toHaveBeenCalledOnce();

    debounced();
    vi.advanceTimersByTime(100);
    expect(fn).toHaveBeenCalledTimes(2);
  });
});
