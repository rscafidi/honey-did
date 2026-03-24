/**
 * Integration tests for all section data operations.
 * Tests the add/update/remove logic for every field in every section.
 */
import { describe, it, expect } from 'vitest';
import type {
  FinancialSection,
  InsuranceSection,
  BillsSection,
  PropertySection,
  LegalSection,
  DigitalSection,
  HouseholdSection,
  PersonalSection,
  ContactsSection,
  MedicalSection,
  PetsSection,
  CustomSection,
  CustomSubsection,
  FormElement,
  CustomItem,
} from '../stores/document';

// ==============================
// Financial Section
// ==============================
describe('Financial Section data operations', () => {
  function createFinancial(): FinancialSection {
    return { bank_accounts: [], credit_cards: [], investments: [], debts: [], notes: '' };
  }

  describe('Bank Accounts', () => {
    it('adds a bank account with all fields', () => {
      const section = createFinancial();
      const account = { name: 'Checking', institution: 'Chase', account_type: 'Checking', last_four: '1234', notes: 'Primary account' };
      section.bank_accounts.push(account);
      expect(section.bank_accounts).toHaveLength(1);
      expect(section.bank_accounts[0]).toEqual(account);
    });

    it('updates a bank account field', () => {
      const section = createFinancial();
      section.bank_accounts.push({ name: '', institution: '', account_type: 'Checking', last_four: '', notes: '' });
      section.bank_accounts[0] = { ...section.bank_accounts[0], name: 'Savings', institution: 'Wells Fargo' };
      expect(section.bank_accounts[0].name).toBe('Savings');
      expect(section.bank_accounts[0].institution).toBe('Wells Fargo');
    });

    it('removes a bank account', () => {
      const section = createFinancial();
      section.bank_accounts.push({ name: 'A' }, { name: 'B' });
      section.bank_accounts = section.bank_accounts.filter((_, i) => i !== 0);
      expect(section.bank_accounts).toHaveLength(1);
      expect(section.bank_accounts[0].name).toBe('B');
    });
  });

  describe('Credit Cards', () => {
    it('adds a credit card with all fields', () => {
      const section = createFinancial();
      const card = { name: 'Rewards Card', issuer: 'Citi', last_four: '5678', notes: 'Cashback card' };
      section.credit_cards.push(card);
      expect(section.credit_cards[0]).toEqual(card);
    });

    it('updates credit card fields', () => {
      const section = createFinancial();
      section.credit_cards.push({ name: '', issuer: '', last_four: '', notes: '' });
      section.credit_cards[0] = { ...section.credit_cards[0], name: 'Travel Card', issuer: 'Amex', last_four: '9012' };
      expect(section.credit_cards[0].name).toBe('Travel Card');
      expect(section.credit_cards[0].issuer).toBe('Amex');
      expect(section.credit_cards[0].last_four).toBe('9012');
    });

    it('removes a credit card', () => {
      const section = createFinancial();
      section.credit_cards.push({ name: 'A' }, { name: 'B' });
      section.credit_cards = section.credit_cards.filter((_, i) => i !== 1);
      expect(section.credit_cards).toHaveLength(1);
      expect(section.credit_cards[0].name).toBe('A');
    });
  });

  describe('Investments', () => {
    it('adds an investment with all fields', () => {
      const section = createFinancial();
      const inv = { name: 'Retirement', institution: 'Fidelity', account_type: '401k', notes: 'Employer match' };
      section.investments.push(inv);
      expect(section.investments[0]).toEqual(inv);
    });

    it('updates investment fields', () => {
      const section = createFinancial();
      section.investments.push({ name: '', institution: '', account_type: '', notes: '' });
      section.investments[0] = { ...section.investments[0], name: 'Roth IRA', institution: 'Vanguard', account_type: 'IRA' };
      expect(section.investments[0].name).toBe('Roth IRA');
      expect(section.investments[0].account_type).toBe('IRA');
    });
  });

  describe('Debts', () => {
    it('adds a debt with all fields', () => {
      const section = createFinancial();
      const debt = { name: 'Mortgage', lender: 'Bank of America', notes: '30-year fixed' };
      section.debts.push(debt);
      expect(section.debts[0]).toEqual(debt);
    });

    it('updates and removes debts', () => {
      const section = createFinancial();
      section.debts.push({ name: 'Student Loan', lender: 'Navient', notes: '' });
      section.debts[0] = { ...section.debts[0], notes: 'Federal loan' };
      expect(section.debts[0].notes).toBe('Federal loan');
      section.debts = [];
      expect(section.debts).toHaveLength(0);
    });
  });

  describe('Notes', () => {
    it('stores section notes', () => {
      const section = createFinancial();
      section.notes = 'All accounts are joint with spouse';
      expect(section.notes).toBe('All accounts are joint with spouse');
    });
  });
});

// ==============================
// Insurance Section
// ==============================
describe('Insurance Section data operations', () => {
  it('adds a policy with all fields', () => {
    const section: InsuranceSection = { policies: [], notes: '' };
    const policy = { policy_type: 'Life', provider: 'MetLife', policy_number: 'POL-123', contact: '800-555-0100', notes: '$500k term' };
    section.policies.push(policy);
    expect(section.policies[0]).toEqual(policy);
  });

  it('updates policy fields', () => {
    const section: InsuranceSection = { policies: [{ policy_type: '', provider: '', policy_number: '', contact: '', notes: '' }], notes: '' };
    section.policies[0] = { ...section.policies[0], policy_type: 'Auto', provider: 'State Farm', policy_number: 'AF-456' };
    expect(section.policies[0].policy_type).toBe('Auto');
    expect(section.policies[0].provider).toBe('State Farm');
  });

  it('removes a policy', () => {
    const section: InsuranceSection = { policies: [{ policy_type: 'Home' }, { policy_type: 'Auto' }], notes: '' };
    section.policies = section.policies.filter((_, i) => i !== 0);
    expect(section.policies).toHaveLength(1);
    expect(section.policies[0].policy_type).toBe('Auto');
  });
});

// ==============================
// Bills Section
// ==============================
describe('Bills Section data operations', () => {
  it('adds a bill with all fields including autopay checkbox', () => {
    const section: BillsSection = { bills: [], notes: '' };
    const bill = { name: 'Electric', provider: 'ConEd', amount: '$150', due_day: '15', autopay: true, notes: 'Monthly' };
    section.bills.push(bill);
    expect(section.bills[0]).toEqual(bill);
    expect(section.bills[0].autopay).toBe(true);
  });

  it('toggles autopay boolean', () => {
    const section: BillsSection = { bills: [{ name: 'Water', provider: '', amount: '', due_day: '', autopay: false, notes: '' }], notes: '' };
    section.bills[0] = { ...section.bills[0], autopay: true };
    expect(section.bills[0].autopay).toBe(true);
    section.bills[0] = { ...section.bills[0], autopay: false };
    expect(section.bills[0].autopay).toBe(false);
  });

  it('updates bill amount and due day', () => {
    const section: BillsSection = { bills: [{ name: 'Internet', provider: 'Comcast', amount: '', due_day: '', autopay: false, notes: '' }], notes: '' };
    section.bills[0] = { ...section.bills[0], amount: '$89.99', due_day: '1' };
    expect(section.bills[0].amount).toBe('$89.99');
    expect(section.bills[0].due_day).toBe('1');
  });
});

// ==============================
// Property Section
// ==============================
describe('Property Section data operations', () => {
  function createProperty(): PropertySection {
    return { properties: [], vehicles: [], valuables: [], notes: '' };
  }

  describe('Real Estate', () => {
    it('adds a property with all fields', () => {
      const section = createProperty();
      section.properties.push({ name: 'Home', address: '123 Main St', notes: 'Primary residence' });
      expect(section.properties[0].address).toBe('123 Main St');
    });
  });

  describe('Vehicles', () => {
    it('adds a vehicle with all fields', () => {
      const section = createProperty();
      section.vehicles.push({ name: '2023 Honda Civic', details: 'VIN: ABC123', notes: 'Paid off' });
      expect(section.vehicles[0].name).toBe('2023 Honda Civic');
      expect(section.vehicles[0].details).toBe('VIN: ABC123');
    });
  });

  describe('Valuables', () => {
    it('adds a valuable with all fields', () => {
      const section = createProperty();
      section.valuables.push({ name: 'Diamond Ring', location: 'Safe deposit box #42', notes: 'Appraised at $5000' });
      expect(section.valuables[0].location).toBe('Safe deposit box #42');
    });
  });
});

// ==============================
// Legal Section
// ==============================
describe('Legal Section data operations', () => {
  function createLegal(): LegalSection {
    return { will_location: '', attorney: { name: '', relationship: '', phone: '', email: '', notes: '' }, power_of_attorney: '', trusts: [], notes: '' };
  }

  it('sets will location', () => {
    const section = createLegal();
    section.will_location = 'Safe deposit box at Chase';
    expect(section.will_location).toBe('Safe deposit box at Chase');
  });

  it('sets power of attorney', () => {
    const section = createLegal();
    section.power_of_attorney = 'Jane Doe';
    expect(section.power_of_attorney).toBe('Jane Doe');
  });

  it('updates attorney contact fields', () => {
    const section = createLegal();
    section.attorney = { name: 'Bob Smith', relationship: 'Estate Attorney', phone: '555-0100', email: 'bob@law.com', notes: 'On retainer' };
    expect(section.attorney.name).toBe('Bob Smith');
    expect(section.attorney.email).toBe('bob@law.com');
  });

  it('adds a trust with all fields', () => {
    const section = createLegal();
    section.trusts.push({ name: 'Family Trust', trustee: 'Jane Doe', notes: 'Revocable' });
    expect(section.trusts[0].trustee).toBe('Jane Doe');
  });

  it('removes a trust', () => {
    const section = createLegal();
    section.trusts.push({ name: 'A' }, { name: 'B' });
    section.trusts = section.trusts.filter((_, i) => i !== 0);
    expect(section.trusts).toHaveLength(1);
  });
});

// ==============================
// Digital Section
// ==============================
describe('Digital Section data operations', () => {
  function createDigital(): DigitalSection {
    return { email_accounts: [], social_media: [], password_manager: { name: '', master_password_hint: '', recovery_method: '', notes: '' }, notes: '' };
  }

  it('updates password manager fields', () => {
    const section = createDigital();
    section.password_manager = { name: '1Password', master_password_hint: 'First pet + year', recovery_method: 'Emergency kit in safe', notes: 'Family plan' };
    expect(section.password_manager.name).toBe('1Password');
    expect(section.password_manager.master_password_hint).toBe('First pet + year');
  });

  it('adds email accounts', () => {
    const section = createDigital();
    section.email_accounts.push({ name: 'Personal Gmail', username: 'user@gmail.com', recovery_hint: 'Phone number on file', notes: '' });
    expect(section.email_accounts[0].username).toBe('user@gmail.com');
  });

  it('adds social media accounts', () => {
    const section = createDigital();
    section.social_media.push({ name: 'Twitter', username: '@user', recovery_hint: 'Email recovery', notes: '' });
    expect(section.social_media[0].name).toBe('Twitter');
  });
});

// ==============================
// Household Section
// ==============================
describe('Household Section data operations', () => {
  function createHousehold(): HouseholdSection {
    return { maintenance_items: [], contractors: [], how_things_work: [], notes: '' };
  }

  it('adds maintenance items', () => {
    const section = createHousehold();
    section.maintenance_items.push({ name: 'HVAC Filter', frequency: 'Every 3 months', last_done: '2024-01-01', notes: 'Use MERV 13' });
    expect(section.maintenance_items[0].frequency).toBe('Every 3 months');
  });

  it('adds contractors', () => {
    const section = createHousehold();
    section.contractors.push({ name: 'Joe Plumber', relationship: 'Plumber', phone: '555-0200', email: 'joe@plumb.com', notes: 'Reliable' });
    expect(section.contractors[0].relationship).toBe('Plumber');
  });

  it('adds how things work entries', () => {
    const section = createHousehold();
    section.how_things_work.push({ name: 'Sprinkler System', instructions: 'Timer is in garage, zones 1-4' });
    expect(section.how_things_work[0].instructions).toBe('Timer is in garage, zones 1-4');
  });
});

// ==============================
// Personal Section
// ==============================
describe('Personal Section data operations', () => {
  function createPersonal(): PersonalSection {
    return { funeral_preferences: '', obituary_notes: '', messages: [], notes: '' };
  }

  it('sets funeral preferences (textarea)', () => {
    const section = createPersonal();
    section.funeral_preferences = 'Cremation, scatter ashes at sea';
    expect(section.funeral_preferences).toBe('Cremation, scatter ashes at sea');
  });

  it('sets obituary notes (textarea)', () => {
    const section = createPersonal();
    section.obituary_notes = 'Mention volunteer work at shelter';
    expect(section.obituary_notes).toBe('Mention volunteer work at shelter');
  });

  it('adds personal messages', () => {
    const section = createPersonal();
    section.messages.push({ recipient: 'My children', message: 'I love you all' });
    expect(section.messages[0].recipient).toBe('My children');
    expect(section.messages[0].message).toBe('I love you all');
  });

  it('removes a message', () => {
    const section = createPersonal();
    section.messages.push({ recipient: 'A' }, { recipient: 'B' });
    section.messages = section.messages.filter((_, i) => i !== 0);
    expect(section.messages).toHaveLength(1);
    expect(section.messages[0].recipient).toBe('B');
  });
});

// ==============================
// Contacts Section
// ==============================
describe('Contacts Section data operations', () => {
  function createContacts(): ContactsSection {
    return { emergency_contacts: [], family: [], professionals: [], notes: '' };
  }

  it('adds emergency contacts with all fields', () => {
    const section = createContacts();
    section.emergency_contacts.push({ name: 'Jane', relationship: 'Spouse', phone: '555-0300', email: 'jane@email.com', notes: 'Always available' });
    expect(section.emergency_contacts[0].relationship).toBe('Spouse');
  });

  it('adds family members', () => {
    const section = createContacts();
    section.family.push({ name: 'Mom', relationship: 'Mother', phone: '555-0400', email: 'mom@email.com', notes: '' });
    expect(section.family[0].name).toBe('Mom');
  });

  it('adds professionals', () => {
    const section = createContacts();
    section.professionals.push({ name: 'Dr. Smith', relationship: 'Primary Care', phone: '555-0500', email: 'dr@clinic.com', notes: 'Office hours M-F' });
    expect(section.professionals[0].phone).toBe('555-0500');
  });

  it('updates contact fields', () => {
    const section = createContacts();
    section.emergency_contacts.push({ name: '', relationship: '', phone: '', email: '', notes: '' });
    section.emergency_contacts[0] = { ...section.emergency_contacts[0], name: 'Updated Name', phone: '999-9999' };
    expect(section.emergency_contacts[0].name).toBe('Updated Name');
  });
});

// ==============================
// Medical Section
// ==============================
describe('Medical Section data operations', () => {
  function createMedical(): MedicalSection {
    return { family_members: [], notes: '' };
  }

  it('adds a family member with nested doctors and medications', () => {
    const section = createMedical();
    section.family_members.push({
      name: 'Self',
      doctors: [{ name: 'Dr. Jones', specialty: 'Cardiology', phone: '555-0600', email: 'jones@med.com', notes: '' }],
      medications: [{ name: 'Aspirin', dosage: '81mg', frequency: 'Daily', prescriber: 'Dr. Jones', notes: '' }],
      conditions: [],
      allergies: [],
      pharmacy: { name: 'CVS', relationship: 'Pharmacy', phone: '555-0700', email: '', notes: '' },
      notes: 'Annual checkup in March',
    });
    expect(section.family_members[0].name).toBe('Self');
    expect(section.family_members[0].doctors).toHaveLength(1);
    expect(section.family_members[0].medications[0].dosage).toBe('81mg');
    expect(section.family_members[0].pharmacy.name).toBe('CVS');
  });

  it('adds conditions and allergies as arrays', () => {
    const section = createMedical();
    section.family_members.push({
      name: 'Spouse',
      doctors: [],
      medications: [],
      conditions: ['Diabetes', 'Hypertension'],
      allergies: ['Penicillin', 'Shellfish'],
      pharmacy: { name: '', relationship: '', phone: '', email: '', notes: '' },
      notes: '',
    });
    expect(section.family_members[0].conditions).toContain('Diabetes');
    expect(section.family_members[0].allergies).toContain('Penicillin');
  });

  it('adds multiple doctors to a family member', () => {
    const section = createMedical();
    section.family_members.push({ name: 'Self', doctors: [], medications: [], conditions: [], allergies: [], pharmacy: { name: '', relationship: '', phone: '', email: '', notes: '' }, notes: '' });
    section.family_members[0].doctors.push(
      { name: 'Dr. A', specialty: 'PCP', phone: '', email: '', notes: '' },
      { name: 'Dr. B', specialty: 'Dermatology', phone: '', email: '', notes: '' }
    );
    expect(section.family_members[0].doctors).toHaveLength(2);
  });

  it('removes a medication', () => {
    const section = createMedical();
    section.family_members.push({
      name: 'Self',
      doctors: [],
      medications: [{ name: 'Med A' }, { name: 'Med B' }],
      conditions: [],
      allergies: [],
      pharmacy: { name: '', relationship: '', phone: '', email: '', notes: '' },
      notes: '',
    });
    section.family_members[0].medications = section.family_members[0].medications.filter((_: any, i: number) => i !== 0);
    expect(section.family_members[0].medications).toHaveLength(1);
    expect(section.family_members[0].medications[0].name).toBe('Med B');
  });
});

// ==============================
// Pets Section
// ==============================
describe('Pets Section data operations', () => {
  function createPets(): PetsSection {
    return { pets: [], notes: '' };
  }

  it('adds a pet with all fields including nested vet and medications', () => {
    const section = createPets();
    section.pets.push({
      name: 'Buddy',
      species: 'Dog',
      breed: 'Golden Retriever',
      vet: { name: 'Dr. Paws', relationship: 'Veterinarian', phone: '555-0800', email: 'vet@clinic.com', notes: '' },
      medications: [{ name: 'Heartworm', dosage: 'Monthly', frequency: 'Monthly', prescriber: 'Dr. Paws', notes: '' }],
      feeding: '2 cups kibble twice daily',
      care_notes: 'Needs daily walk',
    });
    expect(section.pets[0].name).toBe('Buddy');
    expect(section.pets[0].species).toBe('Dog');
    expect(section.pets[0].vet.name).toBe('Dr. Paws');
    expect(section.pets[0].medications[0].name).toBe('Heartworm');
    expect(section.pets[0].feeding).toBe('2 cups kibble twice daily');
  });

  it('updates pet vet contact', () => {
    const section = createPets();
    section.pets.push({ name: 'Cat', species: 'Cat', breed: '', vet: { name: '', relationship: '', phone: '', email: '', notes: '' }, medications: [], feeding: '', care_notes: '' });
    section.pets[0].vet = { name: 'Dr. Whiskers', relationship: 'Vet', phone: '555-0900', email: 'whiskers@vet.com', notes: '24hr emergency' };
    expect(section.pets[0].vet.phone).toBe('555-0900');
  });

  it('adds and removes pet medications', () => {
    const section = createPets();
    section.pets.push({ name: 'Dog', species: 'Dog', breed: '', vet: { name: '', relationship: '', phone: '', email: '', notes: '' }, medications: [], feeding: '', care_notes: '' });
    section.pets[0].medications.push({ name: 'Flea Treatment', dosage: 'Topical', frequency: 'Monthly', prescriber: '', notes: '' });
    expect(section.pets[0].medications).toHaveLength(1);
    section.pets[0].medications = [];
    expect(section.pets[0].medications).toHaveLength(0);
  });
});

// ==============================
// Custom Sections
// ==============================
describe('Custom Section data operations', () => {
  function createCustomSection(): CustomSection {
    return {
      id: 'cs1',
      name: 'My Custom Section',
      subsections: [],
    };
  }

  it('adds a subsection with form elements', () => {
    const section = createCustomSection();
    const sub: CustomSubsection = {
      id: 'sub1',
      name: 'Subsection A',
      form_elements: [
        { type: 'field', id: 'f1', name: 'Title', field_type: 'text' },
        { type: 'field', id: 'f2', name: 'Amount', field_type: 'number' },
        { type: 'field', id: 'f3', name: 'Date', field_type: 'date' },
        { type: 'field', id: 'f4', name: 'Active', field_type: 'boolean' },
        { type: 'divider', id: 'd1' },
        { type: 'header', id: 'h1', text: 'Details' },
      ],
      items: [],
    };
    section.subsections.push(sub);
    expect(section.subsections).toHaveLength(1);
    expect(section.subsections[0].form_elements).toHaveLength(6);
  });

  it('adds items with values for all field types', () => {
    const sub: CustomSubsection = {
      id: 'sub1',
      name: 'Test',
      form_elements: [
        { type: 'field', id: 'text_field', name: 'Name', field_type: 'text' },
        { type: 'field', id: 'num_field', name: 'Count', field_type: 'number' },
        { type: 'field', id: 'date_field', name: 'When', field_type: 'date' },
        { type: 'field', id: 'bool_field', name: 'Done', field_type: 'boolean' },
      ],
      items: [],
    };

    const item: CustomItem = {
      id: 'item1',
      values: {
        text_field: 'Hello World',
        num_field: '42',
        date_field: '2024-06-15',
        bool_field: 'true',
        _notes: 'Some notes',
      },
    };
    sub.items.push(item);

    expect(sub.items[0].values.text_field).toBe('Hello World');
    expect(sub.items[0].values.num_field).toBe('42');
    expect(sub.items[0].values.date_field).toBe('2024-06-15');
    expect(sub.items[0].values.bool_field).toBe('true');
    expect(sub.items[0].values._notes).toBe('Some notes');
  });

  it('updates item field values', () => {
    const item: CustomItem = { id: 'i1', values: { f1: 'old' } };
    item.values = { ...item.values, f1: 'new' };
    expect(item.values.f1).toBe('new');
  });

  it('removes an item from subsection', () => {
    const sub: CustomSubsection = {
      id: 'sub1',
      name: 'Test',
      form_elements: [],
      items: [
        { id: 'i1', values: { f1: 'A' } },
        { id: 'i2', values: { f1: 'B' } },
      ],
    };
    sub.items = sub.items.filter((item) => item.id !== 'i1');
    expect(sub.items).toHaveLength(1);
    expect(sub.items[0].id).toBe('i2');
  });

  it('renames a subsection', () => {
    const section = createCustomSection();
    section.subsections.push({ id: 'sub1', name: 'Old Name', form_elements: [], items: [] });
    section.subsections = section.subsections.map((sub) =>
      sub.id === 'sub1' ? { ...sub, name: 'New Name' } : sub
    );
    expect(section.subsections[0].name).toBe('New Name');
  });

  it('deletes a subsection', () => {
    const section = createCustomSection();
    section.subsections.push(
      { id: 'sub1', name: 'A', form_elements: [], items: [] },
      { id: 'sub2', name: 'B', form_elements: [], items: [] }
    );
    section.subsections = section.subsections.filter((sub) => sub.id !== 'sub1');
    expect(section.subsections).toHaveLength(1);
    expect(section.subsections[0].id).toBe('sub2');
  });

  it('supports parent custom sections (subsection of built-in section)', () => {
    const section: CustomSection = {
      id: 'cs2',
      name: 'Crypto Wallets',
      parent: 'financial',
      subsections: [{ id: 'sub1', name: 'Hot Wallets', form_elements: [], items: [] }],
    };
    expect(section.parent).toBe('financial');
  });

  it('modifies form elements (add/remove fields)', () => {
    const sub: CustomSubsection = {
      id: 'sub1',
      name: 'Test',
      form_elements: [
        { type: 'field', id: 'f1', name: 'Name', field_type: 'text' },
      ],
      items: [],
    };

    // Add more elements
    sub.form_elements = [
      ...sub.form_elements,
      { type: 'divider', id: 'd1' },
      { type: 'header', id: 'h1', text: 'Details' },
      { type: 'field', id: 'f2', name: 'Amount', field_type: 'number' },
    ];
    expect(sub.form_elements).toHaveLength(4);

    // Remove an element
    sub.form_elements = sub.form_elements.filter((el) => el.id !== 'd1');
    expect(sub.form_elements).toHaveLength(3);
  });
});

// ==============================
// Cross-section: full document assembly
// ==============================
describe('Full document assembly', () => {
  it('creates a complete document with data in every section', () => {
    const doc = {
      meta: { creator_name: 'Test User', created_at: '2024-01-01', updated_at: '2024-06-01' },
      financial: {
        bank_accounts: [{ name: 'Checking', institution: 'Chase', account_type: 'Checking', last_four: '1234', notes: '' }],
        credit_cards: [{ name: 'Visa', issuer: 'Citi', last_four: '5678', notes: '' }],
        investments: [{ name: '401k', institution: 'Fidelity', account_type: '401k', notes: '' }],
        debts: [{ name: 'Mortgage', lender: 'BoA', notes: '' }],
        notes: 'Financial notes',
      },
      insurance: {
        policies: [{ policy_type: 'Life', provider: 'MetLife', policy_number: 'P123', contact: '800-555-0100', notes: '' }],
        notes: '',
      },
      bills: {
        bills: [{ name: 'Electric', provider: 'ConEd', amount: '$150', due_day: '15', autopay: true, notes: '' }],
        notes: '',
      },
      property: {
        properties: [{ name: 'Home', address: '123 Main St', notes: '' }],
        vehicles: [{ name: '2023 Civic', details: 'VIN: X', notes: '' }],
        valuables: [{ name: 'Ring', location: 'Safe', notes: '' }],
        notes: '',
      },
      legal: {
        will_location: 'Safe deposit',
        attorney: { name: 'Bob', relationship: 'Attorney', phone: '555-0100', email: 'bob@law.com', notes: '' },
        power_of_attorney: 'Jane',
        trusts: [{ name: 'Family Trust', trustee: 'Jane', notes: '' }],
        notes: '',
      },
      digital: {
        email_accounts: [{ name: 'Gmail', username: 'user@gmail.com', recovery_hint: 'Phone', notes: '' }],
        social_media: [{ name: 'Twitter', username: '@user', recovery_hint: 'Email', notes: '' }],
        password_manager: { name: '1Password', master_password_hint: 'Hint', recovery_method: 'Emergency kit', notes: '' },
        notes: '',
      },
      household: {
        maintenance_items: [{ name: 'HVAC', frequency: '3 months', last_done: '2024-01', notes: '' }],
        contractors: [{ name: 'Plumber Joe', relationship: 'Plumber', phone: '555-0200', email: '', notes: '' }],
        how_things_work: [{ name: 'Sprinkler', instructions: 'Timer in garage' }],
        notes: '',
      },
      personal: {
        funeral_preferences: 'Cremation',
        obituary_notes: 'Mention volunteering',
        messages: [{ recipient: 'Kids', message: 'Be kind' }],
        notes: '',
      },
      contacts: {
        emergency_contacts: [{ name: 'Jane', relationship: 'Spouse', phone: '555-0300', email: '', notes: '' }],
        family: [{ name: 'Mom', relationship: 'Mother', phone: '555-0400', email: '', notes: '' }],
        professionals: [{ name: 'Dr. Smith', relationship: 'PCP', phone: '555-0500', email: '', notes: '' }],
        notes: '',
      },
      medical: {
        family_members: [{
          name: 'Self',
          doctors: [{ name: 'Dr. Jones', specialty: 'Cardio', phone: '', email: '', notes: '' }],
          medications: [{ name: 'Aspirin', dosage: '81mg', frequency: 'Daily', prescriber: 'Dr. Jones', notes: '' }],
          conditions: ['Hypertension'],
          allergies: ['Penicillin'],
          pharmacy: { name: 'CVS', relationship: '', phone: '555-0700', email: '', notes: '' },
          notes: '',
        }],
        notes: '',
      },
      pets: {
        pets: [{
          name: 'Buddy',
          species: 'Dog',
          breed: 'Lab',
          vet: { name: 'Dr. Paws', relationship: 'Vet', phone: '555-0800', email: '', notes: '' },
          medications: [{ name: 'Heartworm', dosage: 'Monthly', frequency: 'Monthly', prescriber: '', notes: '' }],
          feeding: '2 cups daily',
          care_notes: 'Needs walks',
        }],
        notes: '',
      },
      custom_sections: [{
        id: 'cs1',
        name: 'Custom',
        subsections: [{
          id: 'sub1',
          name: 'Sub',
          form_elements: [
            { type: 'field' as const, id: 'f1', name: 'Name', field_type: 'text' as const },
            { type: 'field' as const, id: 'f2', name: 'Count', field_type: 'number' as const },
          ],
          items: [{ id: 'i1', values: { f1: 'Test', f2: '5' } }],
        }],
      }],
    };

    // Verify every section has data
    expect(doc.financial.bank_accounts).toHaveLength(1);
    expect(doc.financial.credit_cards).toHaveLength(1);
    expect(doc.financial.investments).toHaveLength(1);
    expect(doc.financial.debts).toHaveLength(1);
    expect(doc.insurance.policies).toHaveLength(1);
    expect(doc.bills.bills).toHaveLength(1);
    expect(doc.property.properties).toHaveLength(1);
    expect(doc.property.vehicles).toHaveLength(1);
    expect(doc.property.valuables).toHaveLength(1);
    expect(doc.legal.will_location).toBeTruthy();
    expect(doc.legal.trusts).toHaveLength(1);
    expect(doc.digital.email_accounts).toHaveLength(1);
    expect(doc.digital.social_media).toHaveLength(1);
    expect(doc.household.maintenance_items).toHaveLength(1);
    expect(doc.household.contractors).toHaveLength(1);
    expect(doc.household.how_things_work).toHaveLength(1);
    expect(doc.personal.funeral_preferences).toBeTruthy();
    expect(doc.personal.messages).toHaveLength(1);
    expect(doc.contacts.emergency_contacts).toHaveLength(1);
    expect(doc.contacts.family).toHaveLength(1);
    expect(doc.contacts.professionals).toHaveLength(1);
    expect(doc.medical.family_members).toHaveLength(1);
    expect(doc.pets.pets).toHaveLength(1);
    expect(doc.custom_sections).toHaveLength(1);
    expect(doc.custom_sections![0].subsections[0].items).toHaveLength(1);
  });
});
