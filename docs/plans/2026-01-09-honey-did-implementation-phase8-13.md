# honey-did Implementation Plan - Phases 8-13

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Complete the honey-did desktop app by implementing section components, export/import functionality, browser decryption, and testing.

**Current State:** Phases 1-7 complete (Rust backend, app shell with navigation)

---

## Phase 8: Section Components

### Task 8.1: Create Reusable Form Components

**Files:**
- Create: `src/lib/components/ItemCard.svelte`
- Create: `src/lib/components/AddButton.svelte`
- Create: `src/lib/components/NotesField.svelte`

**Step 1: Create ItemCard component**

Create `src/lib/components/ItemCard.svelte`:

```svelte
<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let title: string = '';
  export let editable: boolean = true;

  const dispatch = createEventDispatcher();
</script>

<div class="item-card">
  <div class="item-header">
    <span class="item-title">{title || 'Untitled'}</span>
    {#if editable}
      <button class="delete-btn" on:click={() => dispatch('delete')} title="Delete">×</button>
    {/if}
  </div>
  <div class="item-content">
    <slot />
  </div>
</div>

<style>
  .item-card {
    background: white;
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    margin-bottom: 12px;
  }

  .item-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background: #f8f9fa;
    border-bottom: 1px solid #e0e0e0;
    border-radius: 8px 8px 0 0;
  }

  .item-title {
    font-weight: 600;
    color: #333;
  }

  .delete-btn {
    background: none;
    border: none;
    font-size: 1.5rem;
    color: #999;
    cursor: pointer;
    padding: 0;
    line-height: 1;
  }

  .delete-btn:hover {
    color: #dc3545;
  }

  .item-content {
    padding: 16px;
  }
</style>
```

**Step 2: Create AddButton component**

Create `src/lib/components/AddButton.svelte`:

```svelte
<script lang="ts">
  export let label: string = 'Add Item';
</script>

<button class="add-btn" on:click>
  <span class="plus">+</span>
  <span>{label}</span>
</button>

<style>
  .add-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 16px;
    background: #f0f7ff;
    border: 2px dashed #1976d2;
    border-radius: 8px;
    color: #1976d2;
    font-size: 0.95rem;
    cursor: pointer;
    width: 100%;
    justify-content: center;
  }

  .add-btn:hover {
    background: #e3f2fd;
  }

  .plus {
    font-size: 1.25rem;
    font-weight: bold;
  }
</style>
```

**Step 3: Create NotesField component**

Create `src/lib/components/NotesField.svelte`:

```svelte
<script lang="ts">
  export let value: string = '';
  export let placeholder: string = 'Add any additional notes...';
</script>

<div class="notes-field">
  <label>Section Notes</label>
  <textarea
    bind:value
    {placeholder}
    rows="3"
    on:change
  ></textarea>
</div>

<style>
  .notes-field {
    margin-top: 20px;
    padding-top: 20px;
    border-top: 1px solid #e0e0e0;
  }

  label {
    display: block;
    font-weight: 600;
    margin-bottom: 8px;
    color: #666;
  }

  textarea {
    width: 100%;
    padding: 12px;
    border: 1px solid #ddd;
    border-radius: 6px;
    font-family: inherit;
    font-size: 0.95rem;
    resize: vertical;
  }

  textarea:focus {
    outline: none;
    border-color: #1976d2;
  }
</style>
```

**Verification:**
```bash
npm run build
```

**Commit:**
```bash
git add src/lib/components/
git commit -m "feat: add reusable form components (ItemCard, AddButton, NotesField)"
```

---

### Task 8.2: Create FormField Component

**Files:**
- Create: `src/lib/components/FormField.svelte`

**Step 1: Create FormField component**

Create `src/lib/components/FormField.svelte`:

```svelte
<script lang="ts">
  export let label: string;
  export let value: string = '';
  export let type: 'text' | 'textarea' | 'checkbox' = 'text';
  export let placeholder: string = '';
  export let checked: boolean = false;
</script>

<div class="form-field" class:checkbox={type === 'checkbox'}>
  <label>
    {#if type === 'checkbox'}
      <input type="checkbox" bind:checked on:change />
      <span>{label}</span>
    {:else}
      <span class="label-text">{label}</span>
      {#if type === 'textarea'}
        <textarea bind:value {placeholder} rows="2" on:change></textarea>
      {:else}
        <input type="text" bind:value {placeholder} on:change />
      {/if}
    {/if}
  </label>
</div>

<style>
  .form-field {
    margin-bottom: 12px;
  }

  .label-text {
    display: block;
    font-size: 0.85rem;
    color: #666;
    margin-bottom: 4px;
  }

  input[type="text"], textarea {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 0.95rem;
  }

  input:focus, textarea:focus {
    outline: none;
    border-color: #1976d2;
  }

  .checkbox label {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
  }

  .checkbox input {
    width: 18px;
    height: 18px;
  }
</style>
```

**Verification:**
```bash
npm run build
```

**Commit:**
```bash
git add src/lib/components/FormField.svelte
git commit -m "feat: add FormField component"
```

---

### Task 8.3: Create FinancialSection Component

**Files:**
- Create: `src/lib/sections/FinancialSection.svelte`

**Step 1: Create FinancialSection component**

Create `src/lib/sections/FinancialSection.svelte`:

```svelte
<script lang="ts">
  import { document } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';

  $: financial = $document?.financial ?? {
    bank_accounts: [],
    credit_cards: [],
    investments: [],
    debts: [],
    notes: ''
  };

  function addBankAccount() {
    const updated = {
      ...financial,
      bank_accounts: [...financial.bank_accounts, {
        name: '',
        institution: '',
        account_type: 'Checking',
        last_four: '',
        notes: ''
      }]
    };
    document.updateSection('financial', updated);
  }

  function removeBankAccount(index: number) {
    const updated = {
      ...financial,
      bank_accounts: financial.bank_accounts.filter((_, i) => i !== index)
    };
    document.updateSection('financial', updated);
  }

  function updateBankAccount(index: number, field: string, value: string) {
    const accounts = [...financial.bank_accounts];
    accounts[index] = { ...accounts[index], [field]: value };
    document.updateSection('financial', { ...financial, bank_accounts: accounts });
  }

  function addCreditCard() {
    const updated = {
      ...financial,
      credit_cards: [...financial.credit_cards, {
        name: '',
        issuer: '',
        last_four: '',
        notes: ''
      }]
    };
    document.updateSection('financial', updated);
  }

  function removeCreditCard(index: number) {
    const updated = {
      ...financial,
      credit_cards: financial.credit_cards.filter((_, i) => i !== index)
    };
    document.updateSection('financial', updated);
  }

  function updateCreditCard(index: number, field: string, value: string) {
    const cards = [...financial.credit_cards];
    cards[index] = { ...cards[index], [field]: value };
    document.updateSection('financial', { ...financial, credit_cards: cards });
  }

  function addInvestment() {
    const updated = {
      ...financial,
      investments: [...financial.investments, {
        name: '',
        institution: '',
        account_type: '',
        notes: ''
      }]
    };
    document.updateSection('financial', updated);
  }

  function removeInvestment(index: number) {
    const updated = {
      ...financial,
      investments: financial.investments.filter((_, i) => i !== index)
    };
    document.updateSection('financial', updated);
  }

  function updateInvestment(index: number, field: string, value: string) {
    const investments = [...financial.investments];
    investments[index] = { ...investments[index], [field]: value };
    document.updateSection('financial', { ...financial, investments });
  }

  function addDebt() {
    const updated = {
      ...financial,
      debts: [...financial.debts, {
        name: '',
        lender: '',
        notes: ''
      }]
    };
    document.updateSection('financial', updated);
  }

  function removeDebt(index: number) {
    const updated = {
      ...financial,
      debts: financial.debts.filter((_, i) => i !== index)
    };
    document.updateSection('financial', updated);
  }

  function updateDebt(index: number, field: string, value: string) {
    const debts = [...financial.debts];
    debts[index] = { ...debts[index], [field]: value };
    document.updateSection('financial', { ...financial, debts });
  }

  function updateNotes(e: Event) {
    const target = e.target as HTMLTextAreaElement;
    document.updateSection('financial', { ...financial, notes: target.value });
  }
</script>

<div class="section">
  <div class="subsection">
    <h3>Bank Accounts</h3>
    {#each financial.bank_accounts as account, i}
      <ItemCard title={account.name || 'New Account'} on:delete={() => removeBankAccount(i)}>
        <FormField label="Account Name" value={account.name} on:change={(e) => updateBankAccount(i, 'name', e.target.value)} />
        <FormField label="Institution" value={account.institution} on:change={(e) => updateBankAccount(i, 'institution', e.target.value)} />
        <FormField label="Account Type" value={account.account_type} on:change={(e) => updateBankAccount(i, 'account_type', e.target.value)} />
        <FormField label="Last 4 Digits" value={account.last_four} on:change={(e) => updateBankAccount(i, 'last_four', e.target.value)} />
        <FormField label="Notes" type="textarea" value={account.notes} on:change={(e) => updateBankAccount(i, 'notes', e.target.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Bank Account" on:click={addBankAccount} />
  </div>

  <div class="subsection">
    <h3>Credit Cards</h3>
    {#each financial.credit_cards as card, i}
      <ItemCard title={card.name || 'New Card'} on:delete={() => removeCreditCard(i)}>
        <FormField label="Card Name" value={card.name} on:change={(e) => updateCreditCard(i, 'name', e.target.value)} />
        <FormField label="Issuer" value={card.issuer} on:change={(e) => updateCreditCard(i, 'issuer', e.target.value)} />
        <FormField label="Last 4 Digits" value={card.last_four} on:change={(e) => updateCreditCard(i, 'last_four', e.target.value)} />
        <FormField label="Notes" type="textarea" value={card.notes} on:change={(e) => updateCreditCard(i, 'notes', e.target.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Credit Card" on:click={addCreditCard} />
  </div>

  <div class="subsection">
    <h3>Investments</h3>
    {#each financial.investments as investment, i}
      <ItemCard title={investment.name || 'New Investment'} on:delete={() => removeInvestment(i)}>
        <FormField label="Account Name" value={investment.name} on:change={(e) => updateInvestment(i, 'name', e.target.value)} />
        <FormField label="Institution" value={investment.institution} on:change={(e) => updateInvestment(i, 'institution', e.target.value)} />
        <FormField label="Account Type" value={investment.account_type} placeholder="401k, IRA, Brokerage, etc." on:change={(e) => updateInvestment(i, 'account_type', e.target.value)} />
        <FormField label="Notes" type="textarea" value={investment.notes} on:change={(e) => updateInvestment(i, 'notes', e.target.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Investment Account" on:click={addInvestment} />
  </div>

  <div class="subsection">
    <h3>Debts & Loans</h3>
    {#each financial.debts as debt, i}
      <ItemCard title={debt.name || 'New Debt'} on:delete={() => removeDebt(i)}>
        <FormField label="Description" value={debt.name} on:change={(e) => updateDebt(i, 'name', e.target.value)} />
        <FormField label="Lender" value={debt.lender} on:change={(e) => updateDebt(i, 'lender', e.target.value)} />
        <FormField label="Notes" type="textarea" value={debt.notes} on:change={(e) => updateDebt(i, 'notes', e.target.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Debt/Loan" on:click={addDebt} />
  </div>

  <NotesField value={financial.notes} on:change={updateNotes} />
</div>

<style>
  .section {
    max-width: 800px;
  }

  .subsection {
    margin-bottom: 32px;
  }

  h3 {
    margin: 0 0 16px 0;
    color: #333;
    font-size: 1.1rem;
  }
</style>
```

**Verification:**
```bash
npm run build
```

**Commit:**
```bash
git add src/lib/sections/FinancialSection.svelte
git commit -m "feat: add FinancialSection component"
```

---

### Task 8.4: Create InsuranceSection Component

**Files:**
- Create: `src/lib/sections/InsuranceSection.svelte`

**Step 1: Create InsuranceSection component**

Create `src/lib/sections/InsuranceSection.svelte`:

```svelte
<script lang="ts">
  import { document } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';

  $: insurance = $document?.insurance ?? { policies: [], notes: '' };

  function addPolicy() {
    const updated = {
      ...insurance,
      policies: [...insurance.policies, {
        policy_type: '',
        provider: '',
        policy_number: '',
        contact: '',
        notes: ''
      }]
    };
    document.updateSection('insurance', updated);
  }

  function removePolicy(index: number) {
    const updated = {
      ...insurance,
      policies: insurance.policies.filter((_, i) => i !== index)
    };
    document.updateSection('insurance', updated);
  }

  function updatePolicy(index: number, field: string, value: string) {
    const policies = [...insurance.policies];
    policies[index] = { ...policies[index], [field]: value };
    document.updateSection('insurance', { ...insurance, policies });
  }

  function updateNotes(e: Event) {
    const target = e.target as HTMLTextAreaElement;
    document.updateSection('insurance', { ...insurance, notes: target.value });
  }
</script>

<div class="section">
  <p class="intro">Document all insurance policies so your family knows what coverage exists and how to file claims.</p>

  {#each insurance.policies as policy, i}
    <ItemCard title={policy.policy_type || 'New Policy'} on:delete={() => removePolicy(i)}>
      <FormField label="Policy Type" value={policy.policy_type} placeholder="Life, Health, Auto, Home, etc." on:change={(e) => updatePolicy(i, 'policy_type', e.target.value)} />
      <FormField label="Provider/Company" value={policy.provider} on:change={(e) => updatePolicy(i, 'provider', e.target.value)} />
      <FormField label="Policy Number" value={policy.policy_number} on:change={(e) => updatePolicy(i, 'policy_number', e.target.value)} />
      <FormField label="Contact (Phone/Agent)" value={policy.contact} on:change={(e) => updatePolicy(i, 'contact', e.target.value)} />
      <FormField label="Notes" type="textarea" value={policy.notes} placeholder="Coverage details, beneficiaries, etc." on:change={(e) => updatePolicy(i, 'notes', e.target.value)} />
    </ItemCard>
  {/each}

  <AddButton label="Add Insurance Policy" on:click={addPolicy} />
  <NotesField value={insurance.notes} on:change={updateNotes} />
</div>

<style>
  .section { max-width: 800px; }
  .intro { color: #666; margin-bottom: 20px; }
</style>
```

**Verification:**
```bash
npm run build
```

**Commit:**
```bash
git add src/lib/sections/InsuranceSection.svelte
git commit -m "feat: add InsuranceSection component"
```

---

### Task 8.5: Create BillsSection Component

**Files:**
- Create: `src/lib/sections/BillsSection.svelte`

**Step 1: Create BillsSection component**

Create `src/lib/sections/BillsSection.svelte`:

```svelte
<script lang="ts">
  import { document } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';

  $: bills = $document?.bills ?? { bills: [], notes: '' };

  function addBill() {
    const updated = {
      ...bills,
      bills: [...bills.bills, {
        name: '',
        provider: '',
        amount: '',
        due_day: '',
        autopay: false,
        notes: ''
      }]
    };
    document.updateSection('bills', updated);
  }

  function removeBill(index: number) {
    const updated = {
      ...bills,
      bills: bills.bills.filter((_, i) => i !== index)
    };
    document.updateSection('bills', updated);
  }

  function updateBill(index: number, field: string, value: string | boolean) {
    const billsList = [...bills.bills];
    billsList[index] = { ...billsList[index], [field]: value };
    document.updateSection('bills', { ...bills, bills: billsList });
  }

  function updateNotes(e: Event) {
    const target = e.target as HTMLTextAreaElement;
    document.updateSection('bills', { ...bills, notes: target.value });
  }
</script>

<div class="section">
  <p class="intro">List recurring bills and subscriptions so your family knows what needs to be paid or cancelled.</p>

  {#each bills.bills as bill, i}
    <ItemCard title={bill.name || 'New Bill'} on:delete={() => removeBill(i)}>
      <FormField label="Bill Name" value={bill.name} placeholder="Mortgage, Electric, Netflix, etc." on:change={(e) => updateBill(i, 'name', e.target.value)} />
      <FormField label="Provider" value={bill.provider} on:change={(e) => updateBill(i, 'provider', e.target.value)} />
      <FormField label="Approximate Amount" value={bill.amount} placeholder="$100/month" on:change={(e) => updateBill(i, 'amount', e.target.value)} />
      <FormField label="Due Day" value={bill.due_day} placeholder="15th of each month" on:change={(e) => updateBill(i, 'due_day', e.target.value)} />
      <FormField label="Auto-pay enabled" type="checkbox" checked={bill.autopay} on:change={(e) => updateBill(i, 'autopay', e.target.checked)} />
      <FormField label="Notes" type="textarea" value={bill.notes} placeholder="How to pay, account info hints, etc." on:change={(e) => updateBill(i, 'notes', e.target.value)} />
    </ItemCard>
  {/each}

  <AddButton label="Add Bill/Subscription" on:click={addBill} />
  <NotesField value={bills.notes} on:change={updateNotes} />
</div>

<style>
  .section { max-width: 800px; }
  .intro { color: #666; margin-bottom: 20px; }
</style>
```

**Verification:**
```bash
npm run build
```

**Commit:**
```bash
git add src/lib/sections/BillsSection.svelte
git commit -m "feat: add BillsSection component"
```

---

### Task 8.6: Create PropertySection Component

**Files:**
- Create: `src/lib/sections/PropertySection.svelte`

**Step 1: Create PropertySection component**

Create `src/lib/sections/PropertySection.svelte`:

```svelte
<script lang="ts">
  import { document } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';

  $: property = $document?.property ?? { properties: [], vehicles: [], valuables: [], notes: '' };

  function addProperty() {
    const updated = { ...property, properties: [...property.properties, { name: '', address: '', notes: '' }] };
    document.updateSection('property', updated);
  }

  function removeProperty(index: number) {
    const updated = { ...property, properties: property.properties.filter((_, i) => i !== index) };
    document.updateSection('property', updated);
  }

  function updateProperty(index: number, field: string, value: string) {
    const items = [...property.properties];
    items[index] = { ...items[index], [field]: value };
    document.updateSection('property', { ...property, properties: items });
  }

  function addVehicle() {
    const updated = { ...property, vehicles: [...property.vehicles, { name: '', details: '', notes: '' }] };
    document.updateSection('property', updated);
  }

  function removeVehicle(index: number) {
    const updated = { ...property, vehicles: property.vehicles.filter((_, i) => i !== index) };
    document.updateSection('property', updated);
  }

  function updateVehicle(index: number, field: string, value: string) {
    const items = [...property.vehicles];
    items[index] = { ...items[index], [field]: value };
    document.updateSection('property', { ...property, vehicles: items });
  }

  function addValuable() {
    const updated = { ...property, valuables: [...property.valuables, { name: '', location: '', notes: '' }] };
    document.updateSection('property', updated);
  }

  function removeValuable(index: number) {
    const updated = { ...property, valuables: property.valuables.filter((_, i) => i !== index) };
    document.updateSection('property', updated);
  }

  function updateValuable(index: number, field: string, value: string) {
    const items = [...property.valuables];
    items[index] = { ...items[index], [field]: value };
    document.updateSection('property', { ...property, valuables: items });
  }

  function updateNotes(e: Event) {
    document.updateSection('property', { ...property, notes: (e.target as HTMLTextAreaElement).value });
  }
</script>

<div class="section">
  <div class="subsection">
    <h3>Real Estate</h3>
    {#each property.properties as prop, i}
      <ItemCard title={prop.name || 'New Property'} on:delete={() => removeProperty(i)}>
        <FormField label="Property Name" value={prop.name} placeholder="Primary home, Rental, etc." on:change={(e) => updateProperty(i, 'name', e.target.value)} />
        <FormField label="Address" value={prop.address} on:change={(e) => updateProperty(i, 'address', e.target.value)} />
        <FormField label="Notes" type="textarea" value={prop.notes} placeholder="Mortgage info, deed location, etc." on:change={(e) => updateProperty(i, 'notes', e.target.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Property" on:click={addProperty} />
  </div>

  <div class="subsection">
    <h3>Vehicles</h3>
    {#each property.vehicles as vehicle, i}
      <ItemCard title={vehicle.name || 'New Vehicle'} on:delete={() => removeVehicle(i)}>
        <FormField label="Vehicle" value={vehicle.name} placeholder="2020 Honda Accord" on:change={(e) => updateVehicle(i, 'name', e.target.value)} />
        <FormField label="Details" value={vehicle.details} placeholder="VIN, license plate, loan info" on:change={(e) => updateVehicle(i, 'details', e.target.value)} />
        <FormField label="Notes" type="textarea" value={vehicle.notes} on:change={(e) => updateVehicle(i, 'notes', e.target.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Vehicle" on:click={addVehicle} />
  </div>

  <div class="subsection">
    <h3>Valuables & Storage</h3>
    {#each property.valuables as valuable, i}
      <ItemCard title={valuable.name || 'New Item'} on:delete={() => removeValuable(i)}>
        <FormField label="Item" value={valuable.name} placeholder="Jewelry, safe deposit box, etc." on:change={(e) => updateValuable(i, 'name', e.target.value)} />
        <FormField label="Location" value={valuable.location} on:change={(e) => updateValuable(i, 'location', e.target.value)} />
        <FormField label="Notes" type="textarea" value={valuable.notes} on:change={(e) => updateValuable(i, 'notes', e.target.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Valuable/Storage" on:click={addValuable} />
  </div>

  <NotesField value={property.notes} on:change={updateNotes} />
</div>

<style>
  .section { max-width: 800px; }
  .subsection { margin-bottom: 32px; }
  h3 { margin: 0 0 16px 0; color: #333; font-size: 1.1rem; }
</style>
```

**Verification:**
```bash
npm run build
```

**Commit:**
```bash
git add src/lib/sections/PropertySection.svelte
git commit -m "feat: add PropertySection component"
```

---

### Task 8.7: Create LegalSection Component

**Files:**
- Create: `src/lib/sections/LegalSection.svelte`

**Step 1: Create LegalSection component**

Create `src/lib/sections/LegalSection.svelte`:

```svelte
<script lang="ts">
  import { document } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';

  const emptyContact = { name: '', relationship: '', phone: '', email: '', notes: '' };

  $: legal = $document?.legal ?? {
    will_location: '',
    attorney: { ...emptyContact },
    power_of_attorney: '',
    trusts: [],
    notes: ''
  };

  function updateField(field: string, value: string) {
    document.updateSection('legal', { ...legal, [field]: value });
  }

  function updateAttorney(field: string, value: string) {
    document.updateSection('legal', { ...legal, attorney: { ...legal.attorney, [field]: value } });
  }

  function addTrust() {
    const updated = { ...legal, trusts: [...legal.trusts, { name: '', trustee: '', notes: '' }] };
    document.updateSection('legal', updated);
  }

  function removeTrust(index: number) {
    const updated = { ...legal, trusts: legal.trusts.filter((_, i) => i !== index) };
    document.updateSection('legal', updated);
  }

  function updateTrust(index: number, field: string, value: string) {
    const trusts = [...legal.trusts];
    trusts[index] = { ...trusts[index], [field]: value };
    document.updateSection('legal', { ...legal, trusts });
  }

  function updateNotes(e: Event) {
    document.updateSection('legal', { ...legal, notes: (e.target as HTMLTextAreaElement).value });
  }
</script>

<div class="section">
  <div class="subsection">
    <h3>Will & Estate Documents</h3>
    <FormField label="Will Location" value={legal.will_location} placeholder="Where is your will stored?" on:change={(e) => updateField('will_location', e.target.value)} />
    <FormField label="Power of Attorney" value={legal.power_of_attorney} placeholder="Who has power of attorney?" on:change={(e) => updateField('power_of_attorney', e.target.value)} />
  </div>

  <div class="subsection">
    <h3>Attorney</h3>
    <div class="attorney-card">
      <FormField label="Name" value={legal.attorney.name} on:change={(e) => updateAttorney('name', e.target.value)} />
      <FormField label="Firm/Relationship" value={legal.attorney.relationship} on:change={(e) => updateAttorney('relationship', e.target.value)} />
      <FormField label="Phone" value={legal.attorney.phone} on:change={(e) => updateAttorney('phone', e.target.value)} />
      <FormField label="Email" value={legal.attorney.email} on:change={(e) => updateAttorney('email', e.target.value)} />
      <FormField label="Notes" type="textarea" value={legal.attorney.notes} on:change={(e) => updateAttorney('notes', e.target.value)} />
    </div>
  </div>

  <div class="subsection">
    <h3>Trusts</h3>
    {#each legal.trusts as trust, i}
      <ItemCard title={trust.name || 'New Trust'} on:delete={() => removeTrust(i)}>
        <FormField label="Trust Name" value={trust.name} on:change={(e) => updateTrust(i, 'name', e.target.value)} />
        <FormField label="Trustee" value={trust.trustee} on:change={(e) => updateTrust(i, 'trustee', e.target.value)} />
        <FormField label="Notes" type="textarea" value={trust.notes} on:change={(e) => updateTrust(i, 'notes', e.target.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Trust" on:click={addTrust} />
  </div>

  <NotesField value={legal.notes} on:change={updateNotes} />
</div>

<style>
  .section { max-width: 800px; }
  .subsection { margin-bottom: 32px; }
  h3 { margin: 0 0 16px 0; color: #333; font-size: 1.1rem; }
  .attorney-card { background: #f8f9fa; padding: 16px; border-radius: 8px; }
</style>
```

**Verification:**
```bash
npm run build
```

**Commit:**
```bash
git add src/lib/sections/LegalSection.svelte
git commit -m "feat: add LegalSection component"
```

---

### Task 8.8: Create DigitalSection Component

**Files:**
- Create: `src/lib/sections/DigitalSection.svelte`

**Step 1: Create DigitalSection component**

Create `src/lib/sections/DigitalSection.svelte`:

```svelte
<script lang="ts">
  import { document } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';

  const emptyAccount = { name: '', username: '', recovery_hint: '', notes: '' };
  const emptyPwManager = { name: '', master_password_hint: '', recovery_method: '', notes: '' };

  $: digital = $document?.digital ?? {
    email_accounts: [],
    social_media: [],
    password_manager: { ...emptyPwManager },
    notes: ''
  };

  function addEmail() {
    const updated = { ...digital, email_accounts: [...digital.email_accounts, { ...emptyAccount }] };
    document.updateSection('digital', updated);
  }

  function removeEmail(index: number) {
    const updated = { ...digital, email_accounts: digital.email_accounts.filter((_, i) => i !== index) };
    document.updateSection('digital', updated);
  }

  function updateEmail(index: number, field: string, value: string) {
    const accounts = [...digital.email_accounts];
    accounts[index] = { ...accounts[index], [field]: value };
    document.updateSection('digital', { ...digital, email_accounts: accounts });
  }

  function addSocial() {
    const updated = { ...digital, social_media: [...digital.social_media, { ...emptyAccount }] };
    document.updateSection('digital', updated);
  }

  function removeSocial(index: number) {
    const updated = { ...digital, social_media: digital.social_media.filter((_, i) => i !== index) };
    document.updateSection('digital', updated);
  }

  function updateSocial(index: number, field: string, value: string) {
    const accounts = [...digital.social_media];
    accounts[index] = { ...accounts[index], [field]: value };
    document.updateSection('digital', { ...digital, social_media: accounts });
  }

  function updatePasswordManager(field: string, value: string) {
    document.updateSection('digital', { ...digital, password_manager: { ...digital.password_manager, [field]: value } });
  }

  function updateNotes(e: Event) {
    document.updateSection('digital', { ...digital, notes: (e.target as HTMLTextAreaElement).value });
  }
</script>

<div class="section">
  <div class="subsection highlight">
    <h3>Password Manager (Important!)</h3>
    <p class="hint">If you use a password manager, this is the most important section. Access to this unlocks everything else.</p>
    <div class="pw-card">
      <FormField label="Password Manager" value={digital.password_manager.name} placeholder="1Password, LastPass, Bitwarden, etc." on:change={(e) => updatePasswordManager('name', e.target.value)} />
      <FormField label="Master Password Hint" value={digital.password_manager.master_password_hint} placeholder="A hint only your family would understand" on:change={(e) => updatePasswordManager('master_password_hint', e.target.value)} />
      <FormField label="Recovery Method" value={digital.password_manager.recovery_method} placeholder="Emergency kit location, recovery key, etc." on:change={(e) => updatePasswordManager('recovery_method', e.target.value)} />
      <FormField label="Notes" type="textarea" value={digital.password_manager.notes} on:change={(e) => updatePasswordManager('notes', e.target.value)} />
    </div>
  </div>

  <div class="subsection">
    <h3>Email Accounts</h3>
    {#each digital.email_accounts as account, i}
      <ItemCard title={account.name || 'New Email'} on:delete={() => removeEmail(i)}>
        <FormField label="Service" value={account.name} placeholder="Gmail, Outlook, etc." on:change={(e) => updateEmail(i, 'name', e.target.value)} />
        <FormField label="Email/Username" value={account.username} on:change={(e) => updateEmail(i, 'username', e.target.value)} />
        <FormField label="Recovery Hint" value={account.recovery_hint} placeholder="Recovery phone, backup email, etc." on:change={(e) => updateEmail(i, 'recovery_hint', e.target.value)} />
        <FormField label="Notes" type="textarea" value={account.notes} on:change={(e) => updateEmail(i, 'notes', e.target.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Email Account" on:click={addEmail} />
  </div>

  <div class="subsection">
    <h3>Social Media</h3>
    {#each digital.social_media as account, i}
      <ItemCard title={account.name || 'New Account'} on:delete={() => removeSocial(i)}>
        <FormField label="Service" value={account.name} placeholder="Facebook, Twitter, LinkedIn, etc." on:change={(e) => updateSocial(i, 'name', e.target.value)} />
        <FormField label="Username" value={account.username} on:change={(e) => updateSocial(i, 'username', e.target.value)} />
        <FormField label="Recovery Hint" value={account.recovery_hint} on:change={(e) => updateSocial(i, 'recovery_hint', e.target.value)} />
        <FormField label="Notes" type="textarea" value={account.notes} placeholder="Memorial settings, legacy contact, etc." on:change={(e) => updateSocial(i, 'notes', e.target.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Social Media Account" on:click={addSocial} />
  </div>

  <NotesField value={digital.notes} on:change={updateNotes} />
</div>

<style>
  .section { max-width: 800px; }
  .subsection { margin-bottom: 32px; }
  .subsection.highlight { background: #e3f2fd; padding: 20px; border-radius: 8px; margin-bottom: 32px; }
  h3 { margin: 0 0 16px 0; color: #333; font-size: 1.1rem; }
  .hint { color: #1565c0; font-size: 0.9rem; margin-bottom: 16px; }
  .pw-card { background: white; padding: 16px; border-radius: 8px; }
</style>
```

**Verification:**
```bash
npm run build
```

**Commit:**
```bash
git add src/lib/sections/DigitalSection.svelte
git commit -m "feat: add DigitalSection component"
```

---

### Task 8.9: Create HouseholdSection Component

**Files:**
- Create: `src/lib/sections/HouseholdSection.svelte`

**Step 1: Create HouseholdSection component**

Create `src/lib/sections/HouseholdSection.svelte`:

```svelte
<script lang="ts">
  import { document } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';

  const emptyContact = { name: '', relationship: '', phone: '', email: '', notes: '' };

  $: household = $document?.household ?? {
    maintenance_items: [],
    contractors: [],
    how_things_work: [],
    notes: ''
  };

  function addMaintenance() {
    const updated = { ...household, maintenance_items: [...household.maintenance_items, { name: '', frequency: '', last_done: '', notes: '' }] };
    document.updateSection('household', updated);
  }

  function removeMaintenance(index: number) {
    const updated = { ...household, maintenance_items: household.maintenance_items.filter((_, i) => i !== index) };
    document.updateSection('household', updated);
  }

  function updateMaintenance(index: number, field: string, value: string) {
    const items = [...household.maintenance_items];
    items[index] = { ...items[index], [field]: value };
    document.updateSection('household', { ...household, maintenance_items: items });
  }

  function addContractor() {
    const updated = { ...household, contractors: [...household.contractors, { ...emptyContact }] };
    document.updateSection('household', updated);
  }

  function removeContractor(index: number) {
    const updated = { ...household, contractors: household.contractors.filter((_, i) => i !== index) };
    document.updateSection('household', updated);
  }

  function updateContractor(index: number, field: string, value: string) {
    const items = [...household.contractors];
    items[index] = { ...items[index], [field]: value };
    document.updateSection('household', { ...household, contractors: items });
  }

  function addHowTo() {
    const updated = { ...household, how_things_work: [...household.how_things_work, { name: '', instructions: '' }] };
    document.updateSection('household', updated);
  }

  function removeHowTo(index: number) {
    const updated = { ...household, how_things_work: household.how_things_work.filter((_, i) => i !== index) };
    document.updateSection('household', updated);
  }

  function updateHowTo(index: number, field: string, value: string) {
    const items = [...household.how_things_work];
    items[index] = { ...items[index], [field]: value };
    document.updateSection('household', { ...household, how_things_work: items });
  }

  function updateNotes(e: Event) {
    document.updateSection('household', { ...household, notes: (e.target as HTMLTextAreaElement).value });
  }
</script>

<div class="section">
  <div class="subsection">
    <h3>Maintenance Tasks</h3>
    <p class="hint">Regular maintenance that needs to happen to keep the house running.</p>
    {#each household.maintenance_items as item, i}
      <ItemCard title={item.name || 'New Task'} on:delete={() => removeMaintenance(i)}>
        <FormField label="Task" value={item.name} placeholder="Change HVAC filter, service furnace, etc." on:change={(e) => updateMaintenance(i, 'name', e.target.value)} />
        <FormField label="Frequency" value={item.frequency} placeholder="Monthly, Annually, etc." on:change={(e) => updateMaintenance(i, 'frequency', e.target.value)} />
        <FormField label="Last Done" value={item.last_done} placeholder="January 2024" on:change={(e) => updateMaintenance(i, 'last_done', e.target.value)} />
        <FormField label="Notes" type="textarea" value={item.notes} on:change={(e) => updateMaintenance(i, 'notes', e.target.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Maintenance Task" on:click={addMaintenance} />
  </div>

  <div class="subsection">
    <h3>Contractors & Service Providers</h3>
    {#each household.contractors as contractor, i}
      <ItemCard title={contractor.name || 'New Contractor'} on:delete={() => removeContractor(i)}>
        <FormField label="Name/Company" value={contractor.name} on:change={(e) => updateContractor(i, 'name', e.target.value)} />
        <FormField label="Service" value={contractor.relationship} placeholder="Plumber, Electrician, Lawn care, etc." on:change={(e) => updateContractor(i, 'relationship', e.target.value)} />
        <FormField label="Phone" value={contractor.phone} on:change={(e) => updateContractor(i, 'phone', e.target.value)} />
        <FormField label="Email" value={contractor.email} on:change={(e) => updateContractor(i, 'email', e.target.value)} />
        <FormField label="Notes" type="textarea" value={contractor.notes} on:change={(e) => updateContractor(i, 'notes', e.target.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Contractor" on:click={addContractor} />
  </div>

  <div class="subsection">
    <h3>How Things Work</h3>
    <p class="hint">Explain things only you know how to do around the house.</p>
    {#each household.how_things_work as howto, i}
      <ItemCard title={howto.name || 'New How-To'} on:delete={() => removeHowTo(i)}>
        <FormField label="What" value={howto.name} placeholder="Turn off water main, reset breaker, etc." on:change={(e) => updateHowTo(i, 'name', e.target.value)} />
        <FormField label="Instructions" type="textarea" value={howto.instructions} placeholder="Step-by-step instructions..." on:change={(e) => updateHowTo(i, 'instructions', e.target.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add How-To" on:click={addHowTo} />
  </div>

  <NotesField value={household.notes} on:change={updateNotes} />
</div>

<style>
  .section { max-width: 800px; }
  .subsection { margin-bottom: 32px; }
  h3 { margin: 0 0 8px 0; color: #333; font-size: 1.1rem; }
  .hint { color: #666; font-size: 0.9rem; margin-bottom: 16px; }
</style>
```

**Verification:**
```bash
npm run build
```

**Commit:**
```bash
git add src/lib/sections/HouseholdSection.svelte
git commit -m "feat: add HouseholdSection component"
```

---

### Task 8.10: Create PersonalSection Component

**Files:**
- Create: `src/lib/sections/PersonalSection.svelte`

**Step 1: Create PersonalSection component**

Create `src/lib/sections/PersonalSection.svelte`:

```svelte
<script lang="ts">
  import { document } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';

  $: personal = $document?.personal ?? {
    funeral_preferences: '',
    obituary_notes: '',
    messages: [],
    notes: ''
  };

  function updateField(field: string, value: string) {
    document.updateSection('personal', { ...personal, [field]: value });
  }

  function addMessage() {
    const updated = { ...personal, messages: [...personal.messages, { recipient: '', message: '' }] };
    document.updateSection('personal', updated);
  }

  function removeMessage(index: number) {
    const updated = { ...personal, messages: personal.messages.filter((_, i) => i !== index) };
    document.updateSection('personal', updated);
  }

  function updateMessage(index: number, field: string, value: string) {
    const messages = [...personal.messages];
    messages[index] = { ...messages[index], [field]: value };
    document.updateSection('personal', { ...personal, messages });
  }

  function updateNotes(e: Event) {
    document.updateSection('personal', { ...personal, notes: (e.target as HTMLTextAreaElement).value });
  }
</script>

<div class="section">
  <p class="intro">This section is for your personal wishes and messages to loved ones. Take your time with this one.</p>

  <div class="subsection">
    <h3>Funeral Preferences</h3>
    <FormField
      label=""
      type="textarea"
      value={personal.funeral_preferences}
      placeholder="Burial vs cremation, service preferences, music, readings, any specific wishes..."
      on:change={(e) => updateField('funeral_preferences', e.target.value)}
    />
  </div>

  <div class="subsection">
    <h3>Obituary Notes</h3>
    <FormField
      label=""
      type="textarea"
      value={personal.obituary_notes}
      placeholder="Key life events, achievements, family members to mention, tone you'd prefer..."
      on:change={(e) => updateField('obituary_notes', e.target.value)}
    />
  </div>

  <div class="subsection">
    <h3>Personal Messages</h3>
    <p class="hint">Leave messages for specific people. These will be included in the document they receive.</p>
    {#each personal.messages as msg, i}
      <ItemCard title={msg.recipient || 'New Message'} on:delete={() => removeMessage(i)}>
        <FormField label="To" value={msg.recipient} placeholder="Name of recipient" on:change={(e) => updateMessage(i, 'recipient', e.target.value)} />
        <FormField label="Message" type="textarea" value={msg.message} placeholder="Your message to them..." on:change={(e) => updateMessage(i, 'message', e.target.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Personal Message" on:click={addMessage} />
  </div>

  <NotesField value={personal.notes} on:change={updateNotes} />
</div>

<style>
  .section { max-width: 800px; }
  .intro { color: #666; margin-bottom: 24px; font-style: italic; }
  .subsection { margin-bottom: 32px; }
  h3 { margin: 0 0 12px 0; color: #333; font-size: 1.1rem; }
  .hint { color: #666; font-size: 0.9rem; margin-bottom: 16px; }
</style>
```

**Verification:**
```bash
npm run build
```

**Commit:**
```bash
git add src/lib/sections/PersonalSection.svelte
git commit -m "feat: add PersonalSection component"
```

---

### Task 8.11: Create ContactsSection Component

**Files:**
- Create: `src/lib/sections/ContactsSection.svelte`

**Step 1: Create ContactsSection component**

Create `src/lib/sections/ContactsSection.svelte`:

```svelte
<script lang="ts">
  import { document } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';

  const emptyContact = { name: '', relationship: '', phone: '', email: '', notes: '' };

  $: contacts = $document?.contacts ?? {
    emergency_contacts: [],
    family: [],
    professionals: [],
    notes: ''
  };

  function addContact(list: 'emergency_contacts' | 'family' | 'professionals') {
    const updated = { ...contacts, [list]: [...contacts[list], { ...emptyContact }] };
    document.updateSection('contacts', updated);
  }

  function removeContact(list: 'emergency_contacts' | 'family' | 'professionals', index: number) {
    const updated = { ...contacts, [list]: contacts[list].filter((_, i) => i !== index) };
    document.updateSection('contacts', updated);
  }

  function updateContact(list: 'emergency_contacts' | 'family' | 'professionals', index: number, field: string, value: string) {
    const items = [...contacts[list]];
    items[index] = { ...items[index], [field]: value };
    document.updateSection('contacts', { ...contacts, [list]: items });
  }

  function updateNotes(e: Event) {
    document.updateSection('contacts', { ...contacts, notes: (e.target as HTMLTextAreaElement).value });
  }
</script>

<div class="section">
  <div class="subsection emergency">
    <h3>Emergency Contacts</h3>
    <p class="hint">Who should be called first in an emergency?</p>
    {#each contacts.emergency_contacts as contact, i}
      <ItemCard title={contact.name || 'New Contact'} on:delete={() => removeContact('emergency_contacts', i)}>
        <FormField label="Name" value={contact.name} on:change={(e) => updateContact('emergency_contacts', i, 'name', e.target.value)} />
        <FormField label="Relationship" value={contact.relationship} on:change={(e) => updateContact('emergency_contacts', i, 'relationship', e.target.value)} />
        <FormField label="Phone" value={contact.phone} on:change={(e) => updateContact('emergency_contacts', i, 'phone', e.target.value)} />
        <FormField label="Email" value={contact.email} on:change={(e) => updateContact('emergency_contacts', i, 'email', e.target.value)} />
        <FormField label="Notes" type="textarea" value={contact.notes} placeholder="When to call, what they can help with..." on:change={(e) => updateContact('emergency_contacts', i, 'notes', e.target.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Emergency Contact" on:click={() => addContact('emergency_contacts')} />
  </div>

  <div class="subsection">
    <h3>Family Members</h3>
    {#each contacts.family as contact, i}
      <ItemCard title={contact.name || 'New Family Member'} on:delete={() => removeContact('family', i)}>
        <FormField label="Name" value={contact.name} on:change={(e) => updateContact('family', i, 'name', e.target.value)} />
        <FormField label="Relationship" value={contact.relationship} on:change={(e) => updateContact('family', i, 'relationship', e.target.value)} />
        <FormField label="Phone" value={contact.phone} on:change={(e) => updateContact('family', i, 'phone', e.target.value)} />
        <FormField label="Email" value={contact.email} on:change={(e) => updateContact('family', i, 'email', e.target.value)} />
        <FormField label="Notes" type="textarea" value={contact.notes} on:change={(e) => updateContact('family', i, 'notes', e.target.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Family Member" on:click={() => addContact('family')} />
  </div>

  <div class="subsection">
    <h3>Professionals</h3>
    <p class="hint">Accountant, financial advisor, doctor, etc.</p>
    {#each contacts.professionals as contact, i}
      <ItemCard title={contact.name || 'New Professional'} on:delete={() => removeContact('professionals', i)}>
        <FormField label="Name" value={contact.name} on:change={(e) => updateContact('professionals', i, 'name', e.target.value)} />
        <FormField label="Role/Specialty" value={contact.relationship} on:change={(e) => updateContact('professionals', i, 'relationship', e.target.value)} />
        <FormField label="Phone" value={contact.phone} on:change={(e) => updateContact('professionals', i, 'phone', e.target.value)} />
        <FormField label="Email" value={contact.email} on:change={(e) => updateContact('professionals', i, 'email', e.target.value)} />
        <FormField label="Notes" type="textarea" value={contact.notes} on:change={(e) => updateContact('professionals', i, 'notes', e.target.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Professional Contact" on:click={() => addContact('professionals')} />
  </div>

  <NotesField value={contacts.notes} on:change={updateNotes} />
</div>

<style>
  .section { max-width: 800px; }
  .subsection { margin-bottom: 32px; }
  .subsection.emergency { background: #fff3e0; padding: 20px; border-radius: 8px; }
  h3 { margin: 0 0 8px 0; color: #333; font-size: 1.1rem; }
  .hint { color: #666; font-size: 0.9rem; margin-bottom: 16px; }
</style>
```

**Verification:**
```bash
npm run build
```

**Commit:**
```bash
git add src/lib/sections/ContactsSection.svelte
git commit -m "feat: add ContactsSection component"
```

---

### Task 8.12: Create MedicalSection Component

**Files:**
- Create: `src/lib/sections/MedicalSection.svelte`

**Step 1: Create MedicalSection component**

Create `src/lib/sections/MedicalSection.svelte`:

```svelte
<script lang="ts">
  import { document } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';

  const emptyContact = { name: '', relationship: '', phone: '', email: '', notes: '' };
  const emptyMedication = { name: '', dosage: '', frequency: '', prescriber: '', notes: '' };

  $: medical = $document?.medical ?? { family_members: [], notes: '' };

  function addFamilyMember() {
    const updated = {
      ...medical,
      family_members: [...medical.family_members, {
        name: '',
        doctors: [],
        medications: [],
        conditions: [],
        allergies: [],
        pharmacy: { ...emptyContact },
        notes: ''
      }]
    };
    document.updateSection('medical', updated);
  }

  function removeFamilyMember(index: number) {
    const updated = { ...medical, family_members: medical.family_members.filter((_, i) => i !== index) };
    document.updateSection('medical', updated);
  }

  function updateFamilyMember(index: number, field: string, value: any) {
    const members = [...medical.family_members];
    members[index] = { ...members[index], [field]: value };
    document.updateSection('medical', { ...medical, family_members: members });
  }

  function updateNotes(e: Event) {
    document.updateSection('medical', { ...medical, notes: (e.target as HTMLTextAreaElement).value });
  }

  function parseList(value: string): string[] {
    return value.split(',').map(s => s.trim()).filter(s => s);
  }

  function formatList(arr: string[]): string {
    return arr.join(', ');
  }
</script>

<div class="section">
  <p class="intro">Document medical information for each family member. This can be critical in emergencies.</p>

  {#each medical.family_members as member, i}
    <ItemCard title={member.name || 'New Family Member'} on:delete={() => removeFamilyMember(i)}>
      <FormField label="Name" value={member.name} on:change={(e) => updateFamilyMember(i, 'name', e.target.value)} />

      <FormField
        label="Conditions (comma-separated)"
        value={formatList(member.conditions || [])}
        placeholder="Diabetes, High blood pressure, etc."
        on:change={(e) => updateFamilyMember(i, 'conditions', parseList(e.target.value))}
      />

      <FormField
        label="Allergies (comma-separated)"
        value={formatList(member.allergies || [])}
        placeholder="Penicillin, Peanuts, etc."
        on:change={(e) => updateFamilyMember(i, 'allergies', parseList(e.target.value))}
      />

      <div class="sub-section">
        <h4>Medications</h4>
        {#each member.medications || [] as med, j}
          <div class="med-row">
            <input placeholder="Medication" value={med.name} on:change={(e) => {
              const meds = [...(member.medications || [])];
              meds[j] = { ...meds[j], name: e.target.value };
              updateFamilyMember(i, 'medications', meds);
            }} />
            <input placeholder="Dosage" value={med.dosage} on:change={(e) => {
              const meds = [...(member.medications || [])];
              meds[j] = { ...meds[j], dosage: e.target.value };
              updateFamilyMember(i, 'medications', meds);
            }} />
            <input placeholder="Frequency" value={med.frequency} on:change={(e) => {
              const meds = [...(member.medications || [])];
              meds[j] = { ...meds[j], frequency: e.target.value };
              updateFamilyMember(i, 'medications', meds);
            }} />
            <button class="remove-btn" on:click={() => {
              const meds = (member.medications || []).filter((_, k) => k !== j);
              updateFamilyMember(i, 'medications', meds);
            }}>×</button>
          </div>
        {/each}
        <button class="add-small" on:click={() => {
          const meds = [...(member.medications || []), { ...emptyMedication }];
          updateFamilyMember(i, 'medications', meds);
        }}>+ Add Medication</button>
      </div>

      <div class="sub-section">
        <h4>Pharmacy</h4>
        <FormField label="Name" value={member.pharmacy?.name || ''} on:change={(e) => updateFamilyMember(i, 'pharmacy', { ...member.pharmacy, name: e.target.value })} />
        <FormField label="Phone" value={member.pharmacy?.phone || ''} on:change={(e) => updateFamilyMember(i, 'pharmacy', { ...member.pharmacy, phone: e.target.value })} />
      </div>

      <FormField label="Notes" type="textarea" value={member.notes} on:change={(e) => updateFamilyMember(i, 'notes', e.target.value)} />
    </ItemCard>
  {/each}

  <AddButton label="Add Family Member" on:click={addFamilyMember} />
  <NotesField value={medical.notes} on:change={updateNotes} />
</div>

<style>
  .section { max-width: 800px; }
  .intro { color: #666; margin-bottom: 24px; }
  .sub-section { margin: 16px 0; padding: 12px; background: #f8f9fa; border-radius: 6px; }
  .sub-section h4 { margin: 0 0 12px 0; font-size: 0.95rem; color: #555; }
  .med-row { display: flex; gap: 8px; margin-bottom: 8px; }
  .med-row input { flex: 1; padding: 6px 10px; border: 1px solid #ddd; border-radius: 4px; }
  .remove-btn { background: none; border: none; color: #999; font-size: 1.2rem; cursor: pointer; padding: 0 8px; }
  .remove-btn:hover { color: #dc3545; }
  .add-small { background: none; border: none; color: #1976d2; cursor: pointer; font-size: 0.9rem; padding: 4px 0; }
  .add-small:hover { text-decoration: underline; }
</style>
```

**Verification:**
```bash
npm run build
```

**Commit:**
```bash
git add src/lib/sections/MedicalSection.svelte
git commit -m "feat: add MedicalSection component"
```

---

### Task 8.13: Create PetsSection Component

**Files:**
- Create: `src/lib/sections/PetsSection.svelte`

**Step 1: Create PetsSection component**

Create `src/lib/sections/PetsSection.svelte`:

```svelte
<script lang="ts">
  import { document } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';

  const emptyContact = { name: '', relationship: '', phone: '', email: '', notes: '' };
  const emptyMedication = { name: '', dosage: '', frequency: '', prescriber: '', notes: '' };

  $: pets = $document?.pets ?? { pets: [], notes: '' };

  function addPet() {
    const updated = {
      ...pets,
      pets: [...pets.pets, {
        name: '',
        species: '',
        breed: '',
        vet: { ...emptyContact },
        medications: [],
        feeding: '',
        care_notes: ''
      }]
    };
    document.updateSection('pets', updated);
  }

  function removePet(index: number) {
    const updated = { ...pets, pets: pets.pets.filter((_, i) => i !== index) };
    document.updateSection('pets', updated);
  }

  function updatePet(index: number, field: string, value: any) {
    const petList = [...pets.pets];
    petList[index] = { ...petList[index], [field]: value };
    document.updateSection('pets', { ...pets, pets: petList });
  }

  function updateNotes(e: Event) {
    document.updateSection('pets', { ...pets, notes: (e.target as HTMLTextAreaElement).value });
  }
</script>

<div class="section">
  <p class="intro">Make sure your pets are cared for by documenting their needs and care providers.</p>

  {#each pets.pets as pet, i}
    <ItemCard title={pet.name || 'New Pet'} on:delete={() => removePet(i)}>
      <div class="row">
        <FormField label="Name" value={pet.name} on:change={(e) => updatePet(i, 'name', e.target.value)} />
        <FormField label="Species" value={pet.species} placeholder="Dog, Cat, etc." on:change={(e) => updatePet(i, 'species', e.target.value)} />
        <FormField label="Breed" value={pet.breed} on:change={(e) => updatePet(i, 'breed', e.target.value)} />
      </div>

      <div class="sub-section">
        <h4>Veterinarian</h4>
        <FormField label="Clinic/Vet Name" value={pet.vet?.name || ''} on:change={(e) => updatePet(i, 'vet', { ...pet.vet, name: e.target.value })} />
        <FormField label="Phone" value={pet.vet?.phone || ''} on:change={(e) => updatePet(i, 'vet', { ...pet.vet, phone: e.target.value })} />
        <FormField label="Notes" value={pet.vet?.notes || ''} on:change={(e) => updatePet(i, 'vet', { ...pet.vet, notes: e.target.value })} />
      </div>

      <div class="sub-section">
        <h4>Medications</h4>
        {#each pet.medications || [] as med, j}
          <div class="med-row">
            <input placeholder="Medication" value={med.name} on:change={(e) => {
              const meds = [...(pet.medications || [])];
              meds[j] = { ...meds[j], name: e.target.value };
              updatePet(i, 'medications', meds);
            }} />
            <input placeholder="Dosage" value={med.dosage} on:change={(e) => {
              const meds = [...(pet.medications || [])];
              meds[j] = { ...meds[j], dosage: e.target.value };
              updatePet(i, 'medications', meds);
            }} />
            <input placeholder="Frequency" value={med.frequency} on:change={(e) => {
              const meds = [...(pet.medications || [])];
              meds[j] = { ...meds[j], frequency: e.target.value };
              updatePet(i, 'medications', meds);
            }} />
            <button class="remove-btn" on:click={() => {
              const meds = (pet.medications || []).filter((_, k) => k !== j);
              updatePet(i, 'medications', meds);
            }}>×</button>
          </div>
        {/each}
        <button class="add-small" on:click={() => {
          const meds = [...(pet.medications || []), { ...emptyMedication }];
          updatePet(i, 'medications', meds);
        }}>+ Add Medication</button>
      </div>

      <FormField label="Feeding Instructions" type="textarea" value={pet.feeding} placeholder="What, when, and how much to feed..." on:change={(e) => updatePet(i, 'feeding', e.target.value)} />
      <FormField label="Care Notes" type="textarea" value={pet.care_notes} placeholder="Walks, grooming, behavior notes, favorite things..." on:change={(e) => updatePet(i, 'care_notes', e.target.value)} />
    </ItemCard>
  {/each}

  <AddButton label="Add Pet" on:click={addPet} />
  <NotesField value={pets.notes} on:change={updateNotes} />
</div>

<style>
  .section { max-width: 800px; }
  .intro { color: #666; margin-bottom: 24px; }
  .row { display: grid; grid-template-columns: repeat(3, 1fr); gap: 12px; }
  .sub-section { margin: 16px 0; padding: 12px; background: #f8f9fa; border-radius: 6px; }
  .sub-section h4 { margin: 0 0 12px 0; font-size: 0.95rem; color: #555; }
  .med-row { display: flex; gap: 8px; margin-bottom: 8px; }
  .med-row input { flex: 1; padding: 6px 10px; border: 1px solid #ddd; border-radius: 4px; }
  .remove-btn { background: none; border: none; color: #999; font-size: 1.2rem; cursor: pointer; padding: 0 8px; }
  .remove-btn:hover { color: #dc3545; }
  .add-small { background: none; border: none; color: #1976d2; cursor: pointer; font-size: 0.9rem; padding: 4px 0; }
  .add-small:hover { text-decoration: underline; }
</style>
```

**Verification:**
```bash
npm run build
```

**Commit:**
```bash
git add src/lib/sections/PetsSection.svelte
git commit -m "feat: add PetsSection component"
```

---

### Task 8.14: Integrate Section Components into App.svelte

**Files:**
- Modify: `src/App.svelte`

**Step 1: Import and render section components**

Update `src/App.svelte` to import all section components and render them based on currentSection:

Add imports at top of script:
```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { document } from './lib/stores/document';
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

  // ... rest of existing script
</script>
```

Replace the content-body placeholder with:
```svelte
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
```

**Verification:**
```bash
npm run build
```

**Commit:**
```bash
git add src/App.svelte
git commit -m "feat: integrate all section components into main app"
```

---

## Phase 9: Export Dialog

### Task 9.1: Create ExportDialog Component

**Files:**
- Create: `src/lib/components/ExportDialog.svelte`

**Step 1: Create ExportDialog component**

Create `src/lib/components/ExportDialog.svelte`:

```svelte
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { save } from '@tauri-apps/plugin-dialog';
  import { writeTextFile } from '@tauri-apps/plugin-fs';

  export let isOpen = false;

  const dispatch = createEventDispatcher();

  let passphrase = '';
  let confirmPassphrase = '';
  let includePrint = false;
  let isExporting = false;
  let error = '';

  $: passphraseStrength = calculateStrength(passphrase);
  $: passphrasesMatch = passphrase === confirmPassphrase;
  $: canExport = passphrase.length >= 8 && passphrasesMatch && !isExporting;

  function calculateStrength(pass: string): { score: number; label: string; color: string } {
    if (!pass) return { score: 0, label: '', color: '#ddd' };
    let score = 0;
    if (pass.length >= 8) score += 1;
    if (pass.length >= 12) score += 1;
    if (pass.length >= 16) score += 1;
    if (/[a-z]/.test(pass) && /[A-Z]/.test(pass)) score += 1;
    if (/\d/.test(pass)) score += 1;
    if (/[^a-zA-Z0-9]/.test(pass)) score += 1;
    if (pass.includes('-') && pass.split('-').length >= 3) score += 2; // Passphrase bonus

    if (score <= 2) return { score, label: 'Weak', color: '#dc3545' };
    if (score <= 4) return { score, label: 'Fair', color: '#ffc107' };
    if (score <= 6) return { score, label: 'Good', color: '#28a745' };
    return { score, label: 'Strong', color: '#1976d2' };
  }

  async function generatePassphrase() {
    try {
      passphrase = await invoke<string>('generate_passphrase');
      confirmPassphrase = passphrase;
    } catch (e) {
      error = `Failed to generate passphrase: ${e}`;
    }
  }

  async function handleExport() {
    if (!canExport) return;

    error = '';
    isExporting = true;

    try {
      const html = await invoke<string>('export_html', { passphrase });

      const filePath = await save({
        filters: [{ name: 'HTML', extensions: ['html'] }],
        defaultPath: 'honey-did-legacy-document.html'
      });

      if (filePath) {
        await writeTextFile(filePath, html);

        if (includePrint) {
          // Open print dialog for the generated content
          const printWindow = window.open('', '_blank');
          if (printWindow) {
            printWindow.document.write(html);
            printWindow.document.close();
            printWindow.print();
          }
        }

        dispatch('exported', { filePath });
        close();
      }
    } catch (e) {
      error = `Export failed: ${e}`;
    } finally {
      isExporting = false;
    }
  }

  function close() {
    passphrase = '';
    confirmPassphrase = '';
    includePrint = false;
    error = '';
    dispatch('close');
  }
</script>

{#if isOpen}
  <div class="overlay" on:click={close} on:keydown={(e) => e.key === 'Escape' && close()}>
    <div class="dialog" on:click|stopPropagation>
      <h2>Create Your Secure File</h2>

      <div class="form">
        <div class="field">
          <label>Choose a passphrase</label>
          <div class="passphrase-input">
            <input
              type="text"
              bind:value={passphrase}
              placeholder="Enter a memorable passphrase"
            />
            <button type="button" class="generate-btn" on:click={generatePassphrase}>
              Generate
            </button>
          </div>
          {#if passphrase}
            <div class="strength-meter">
              <div class="strength-bar" style="width: {passphraseStrength.score * 12.5}%; background: {passphraseStrength.color}"></div>
            </div>
            <span class="strength-label" style="color: {passphraseStrength.color}">{passphraseStrength.label}</span>
          {/if}
        </div>

        <div class="field">
          <label>Confirm passphrase</label>
          <input
            type="password"
            bind:value={confirmPassphrase}
            placeholder="Re-enter passphrase"
          />
          {#if confirmPassphrase && !passphrasesMatch}
            <span class="error-text">Passphrases don't match</span>
          {/if}
        </div>

        <label class="checkbox-field">
          <input type="checkbox" bind:checked={includePrint} />
          <span>Also print a physical copy</span>
        </label>

        {#if includePrint}
          <p class="warning">
            Printed copies can be found by anyone. Store securely.
          </p>
        {/if}

        {#if error}
          <p class="error-message">{error}</p>
        {/if}
      </div>

      <div class="actions">
        <button type="button" class="btn-secondary" on:click={close}>Cancel</button>
        <button
          type="button"
          class="btn-primary"
          on:click={handleExport}
          disabled={!canExport}
        >
          {isExporting ? 'Exporting...' : 'Export File'}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .dialog {
    background: white;
    border-radius: 12px;
    padding: 24px;
    width: 100%;
    max-width: 450px;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  }

  h2 {
    margin: 0 0 20px 0;
    color: #333;
  }

  .form {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .field label {
    display: block;
    margin-bottom: 6px;
    font-weight: 500;
    color: #333;
  }

  .field input {
    width: 100%;
    padding: 10px 12px;
    border: 2px solid #ddd;
    border-radius: 6px;
    font-size: 1rem;
  }

  .field input:focus {
    outline: none;
    border-color: #1976d2;
  }

  .passphrase-input {
    display: flex;
    gap: 8px;
  }

  .passphrase-input input {
    flex: 1;
  }

  .generate-btn {
    padding: 10px 16px;
    background: #e3f2fd;
    border: none;
    border-radius: 6px;
    color: #1976d2;
    cursor: pointer;
    white-space: nowrap;
  }

  .generate-btn:hover {
    background: #bbdefb;
  }

  .strength-meter {
    height: 4px;
    background: #e0e0e0;
    border-radius: 2px;
    margin-top: 8px;
    overflow: hidden;
  }

  .strength-bar {
    height: 100%;
    transition: width 0.3s, background 0.3s;
  }

  .strength-label {
    font-size: 0.85rem;
    margin-top: 4px;
    display: block;
  }

  .error-text {
    color: #dc3545;
    font-size: 0.85rem;
    margin-top: 4px;
  }

  .checkbox-field {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
  }

  .checkbox-field input {
    width: 18px;
    height: 18px;
  }

  .warning {
    padding: 10px 12px;
    background: #fff3cd;
    border-radius: 6px;
    font-size: 0.9rem;
    color: #856404;
    margin: 0;
  }

  .error-message {
    color: #dc3545;
    background: #f8d7da;
    padding: 10px 12px;
    border-radius: 6px;
    margin: 0;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    margin-top: 24px;
  }

  .btn-primary, .btn-secondary {
    padding: 10px 20px;
    border: none;
    border-radius: 6px;
    font-size: 1rem;
    cursor: pointer;
  }

  .btn-primary {
    background: #1976d2;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: #1565c0;
  }

  .btn-primary:disabled {
    background: #90caf9;
    cursor: not-allowed;
  }

  .btn-secondary {
    background: #e0e0e0;
    color: #333;
  }

  .btn-secondary:hover {
    background: #d0d0d0;
  }
</style>
```

**Verification:**
```bash
npm run build
```

**Commit:**
```bash
git add src/lib/components/ExportDialog.svelte
git commit -m "feat: add ExportDialog component with passphrase generation"
```

---

### Task 9.2: Integrate ExportDialog into App.svelte

**Files:**
- Modify: `src/App.svelte`

**Step 1: Add ExportDialog import and usage**

Add to imports in `src/App.svelte`:
```svelte
import ExportDialog from './lib/components/ExportDialog.svelte';
```

Add at end of template (before closing `</main>`):
```svelte
<ExportDialog
  bind:isOpen={showExportDialog}
  on:close={() => (showExportDialog = false)}
  on:exported={(e) => {
    console.log('Exported to:', e.detail.filePath);
  }}
/>
```

**Verification:**
```bash
npm run build
```

**Commit:**
```bash
git add src/App.svelte
git commit -m "feat: integrate ExportDialog into main app"
```

---

## Phase 10: Print Functionality

### Task 10.1: Add Print HTML Generation to Rust

**Files:**
- Modify: `src-tauri/src/export.rs`
- Modify: `src-tauri/src/main.rs`

**Step 1: Add generate_print_html function**

Add to `src-tauri/src/export.rs`:

```rust
/// Generates a printable HTML document (unencrypted, for direct printing)
pub fn generate_print_html(document: &LegacyDocument) -> String {
    // Generate sections HTML
    let financial_html = render_financial(&document.financial);
    let insurance_html = render_insurance(&document.insurance);
    let bills_html = render_bills(&document.bills);
    let property_html = render_property(&document.property);
    let legal_html = render_legal(&document.legal);
    let digital_html = render_digital(&document.digital);
    let household_html = render_household(&document.household);
    let personal_html = render_personal(&document.personal);
    let contacts_html = render_contacts(&document.contacts);
    let medical_html = render_medical(&document.medical);
    let pets_html = render_pets(&document.pets);

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Legacy Document - {}</title>
    <style>
        body {{ font-family: Georgia, serif; line-height: 1.6; max-width: 800px; margin: 0 auto; padding: 20px; }}
        h1 {{ text-align: center; border-bottom: 2px solid #333; padding-bottom: 10px; }}
        h2 {{ color: #333; border-bottom: 1px solid #ccc; padding-bottom: 5px; margin-top: 30px; }}
        .item {{ background: #f9f9f9; padding: 10px; margin: 10px 0; border-left: 3px solid #333; }}
        .item-title {{ font-weight: bold; }}
        .notes {{ background: #fffbe6; padding: 10px; margin: 10px 0; font-style: italic; }}
        @media print {{ body {{ max-width: none; }} }}
    </style>
</head>
<body>
    <h1>Legacy Document</h1>
    <p style="text-align: center;">Created by {} on {}</p>
    {}{}{}{}{}{}{}{}{}{}{}
</body>
</html>"#,
        escape_html(&document.meta.creator_name),
        escape_html(&document.meta.creator_name),
        &document.meta.created_at,
        financial_html, insurance_html, bills_html, property_html, legal_html,
        digital_html, household_html, personal_html, contacts_html, medical_html, pets_html
    )
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

// Helper functions for rendering each section (simplified)
fn render_financial(s: &crate::models::FinancialSection) -> String {
    let mut html = String::from("<h2>Financial</h2>");
    for acc in &s.bank_accounts {
        html.push_str(&format!(
            r#"<div class="item"><div class="item-title">{} - {}</div><div>Last 4: {}</div><div>{}</div></div>"#,
            escape_html(&acc.name), escape_html(&acc.institution), escape_html(&acc.last_four), escape_html(&acc.notes)
        ));
    }
    if !s.notes.is_empty() {
        html.push_str(&format!(r#"<div class="notes">{}</div>"#, escape_html(&s.notes)));
    }
    html
}

fn render_insurance(s: &crate::models::InsuranceSection) -> String {
    let mut html = String::from("<h2>Insurance</h2>");
    for p in &s.policies {
        html.push_str(&format!(
            r#"<div class="item"><div class="item-title">{} - {}</div><div>Policy: {}</div><div>{}</div></div>"#,
            escape_html(&p.policy_type), escape_html(&p.provider), escape_html(&p.policy_number), escape_html(&p.notes)
        ));
    }
    html
}

fn render_bills(s: &crate::models::BillsSection) -> String {
    let mut html = String::from("<h2>Bills & Subscriptions</h2>");
    for b in &s.bills {
        html.push_str(&format!(
            r#"<div class="item"><div class="item-title">{}</div><div>{} - Due: {}</div><div>Autopay: {}</div></div>"#,
            escape_html(&b.name), escape_html(&b.amount), escape_html(&b.due_day), if b.autopay { "Yes" } else { "No" }
        ));
    }
    html
}

fn render_property(s: &crate::models::PropertySection) -> String {
    let mut html = String::from("<h2>Property & Assets</h2>");
    for p in &s.properties {
        html.push_str(&format!(r#"<div class="item"><div class="item-title">{}</div><div>{}</div></div>"#,
            escape_html(&p.name), escape_html(&p.address)));
    }
    for v in &s.vehicles {
        html.push_str(&format!(r#"<div class="item"><div class="item-title">{}</div><div>{}</div></div>"#,
            escape_html(&v.name), escape_html(&v.details)));
    }
    html
}

fn render_legal(s: &crate::models::LegalSection) -> String {
    let mut html = String::from("<h2>Legal</h2>");
    if !s.will_location.is_empty() {
        html.push_str(&format!(r#"<div class="item"><div class="item-title">Will Location</div><div>{}</div></div>"#,
            escape_html(&s.will_location)));
    }
    html
}

fn render_digital(s: &crate::models::DigitalSection) -> String {
    let mut html = String::from("<h2>Digital Life</h2>");
    if !s.password_manager.name.is_empty() {
        html.push_str(&format!(r#"<div class="item" style="background:#e3f2fd;"><div class="item-title">Password Manager: {}</div><div>Hint: {}</div></div>"#,
            escape_html(&s.password_manager.name), escape_html(&s.password_manager.master_password_hint)));
    }
    html
}

fn render_household(s: &crate::models::HouseholdSection) -> String {
    let mut html = String::from("<h2>Household</h2>");
    for h in &s.how_things_work {
        html.push_str(&format!(r#"<div class="item"><div class="item-title">{}</div><div>{}</div></div>"#,
            escape_html(&h.name), escape_html(&h.instructions)));
    }
    html
}

fn render_personal(s: &crate::models::PersonalSection) -> String {
    let mut html = String::from("<h2>Personal Wishes</h2>");
    if !s.funeral_preferences.is_empty() {
        html.push_str(&format!(r#"<div class="item"><div class="item-title">Funeral Preferences</div><div>{}</div></div>"#,
            escape_html(&s.funeral_preferences)));
    }
    html
}

fn render_contacts(s: &crate::models::ContactsSection) -> String {
    let mut html = String::from("<h2>Emergency Contacts</h2>");
    for c in &s.emergency_contacts {
        html.push_str(&format!(r#"<div class="item"><div class="item-title">{} ({})</div><div>{} - {}</div></div>"#,
            escape_html(&c.name), escape_html(&c.relationship), escape_html(&c.phone), escape_html(&c.email)));
    }
    html
}

fn render_medical(s: &crate::models::MedicalSection) -> String {
    let mut html = String::from("<h2>Medical</h2>");
    for m in &s.family_members {
        html.push_str(&format!(r#"<div class="item"><div class="item-title">{}</div></div>"#, escape_html(&m.name)));
    }
    html
}

fn render_pets(s: &crate::models::PetsSection) -> String {
    let mut html = String::from("<h2>Pets</h2>");
    for p in &s.pets {
        html.push_str(&format!(r#"<div class="item"><div class="item-title">{} ({} - {})</div><div>Feeding: {}</div></div>"#,
            escape_html(&p.name), escape_html(&p.species), escape_html(&p.breed), escape_html(&p.feeding)));
    }
    html
}
```

**Step 2: Add print command to main.rs**

Add to `src-tauri/src/main.rs`:

```rust
#[tauri::command]
fn get_print_html(state: State<AppState>) -> Result<String, String> {
    let doc = state.document.lock().map_err(|e| e.to_string())?;
    Ok(export::generate_print_html(&doc))
}
```

Update invoke_handler:
```rust
.invoke_handler(tauri::generate_handler![
    get_document,
    update_document,
    export_html,
    import_file,
    generate_passphrase,
    get_print_html,
])
```

**Verification:**
```bash
cd src-tauri && cargo check
```

**Commit:**
```bash
git add src-tauri/src/export.rs src-tauri/src/main.rs
git commit -m "feat: add print HTML generation"
```

---

## Phase 11: Browser-Compatible Encryption

### Task 11.1: Update Encryption for Web Crypto Compatibility

**Files:**
- Modify: `src-tauri/src/encryption.rs`

**Step 1: Add PBKDF2-based encryption for browser compatibility**

Add to `src-tauri/src/encryption.rs`:

```rust
use ring::pbkdf2;

const PBKDF2_ITERATIONS: u32 = 600_000;

/// Encrypts using PBKDF2 key derivation (compatible with Web Crypto API)
pub fn encrypt_for_browser(plaintext: &str, passphrase: &str) -> Result<EncryptedPayload, EncryptionError> {
    let salt = generate_salt();
    let mut nonce_bytes = [0u8; 12];
    use rand::RngCore;
    OsRng.fill_bytes(&mut nonce_bytes);

    // Derive key using PBKDF2 (Web Crypto compatible)
    let mut key_bytes = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        std::num::NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(),
        &salt,
        passphrase.as_bytes(),
        &mut key_bytes,
    );

    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)
        .map_err(|_| EncryptionError::Encryption("Failed to create key".into()))?;
    let key = LessSafeKey::new(unbound_key);

    let nonce = Nonce::assume_unique_for_key(nonce_bytes);
    let mut in_out = plaintext.as_bytes().to_vec();
    key.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
        .map_err(|_| EncryptionError::Encryption("Encryption failed".into()))?;

    Ok(EncryptedPayload {
        salt: BASE64.encode(salt),
        nonce: BASE64.encode(nonce_bytes),
        ciphertext: BASE64.encode(in_out),
    })
}
```

**Step 2: Update generate_encrypted_html to use browser-compatible encryption**

In `src-tauri/src/export.rs`, change the encrypt call:

```rust
// In generate_encrypted_html function, change:
let encrypted = encrypt(&json, passphrase)?;
// to:
let encrypted = crate::encryption::encrypt_for_browser(&json, passphrase)?;
```

**Verification:**
```bash
cd src-tauri && cargo check
```

**Commit:**
```bash
git add src-tauri/src/encryption.rs src-tauri/src/export.rs
git commit -m "feat: add browser-compatible PBKDF2 encryption"
```

---

### Task 11.2: Update HTML Template with Working Decryption

**Files:**
- Modify: `src-tauri/src/export.rs`

**Step 1: Update the JavaScript decryption in HTML template**

Update the `generate_html_template` function in `src-tauri/src/export.rs` to include working Web Crypto decryption:

Replace the JavaScript section in the template with:

```javascript
async function deriveKey(passphrase, salt) {
    const encoder = new TextEncoder();
    const keyMaterial = await crypto.subtle.importKey(
        'raw', encoder.encode(passphrase), 'PBKDF2', false, ['deriveBits', 'deriveKey']
    );
    return await crypto.subtle.deriveKey(
        { name: 'PBKDF2', salt: salt, iterations: 600000, hash: 'SHA-256' },
        keyMaterial,
        { name: 'AES-GCM', length: 256 },
        false,
        ['decrypt']
    );
}

async function unlock(e) {
    e.preventDefault();
    const passphrase = document.getElementById('passphrase').value;
    const errorEl = document.getElementById('error');
    errorEl.style.display = 'none';

    try {
        const salt = Uint8Array.from(atob(ENCRYPTED_DATA.salt), c => c.charCodeAt(0));
        const nonce = Uint8Array.from(atob(ENCRYPTED_DATA.nonce), c => c.charCodeAt(0));
        const ciphertext = Uint8Array.from(atob(ENCRYPTED_DATA.ciphertext), c => c.charCodeAt(0));

        const key = await deriveKey(passphrase, salt);
        const decrypted = await crypto.subtle.decrypt(
            { name: 'AES-GCM', iv: nonce },
            key,
            ciphertext
        );

        const decoder = new TextDecoder();
        const json = decoder.decode(decrypted);
        const data = JSON.parse(json);
        renderDocument(data);
    } catch (err) {
        errorEl.textContent = 'Incorrect passphrase. Please try again.';
        errorEl.style.display = 'block';
    }
}
```

**Verification:**
```bash
cd src-tauri && cargo check
```

**Commit:**
```bash
git add src-tauri/src/export.rs
git commit -m "feat: add working Web Crypto decryption to HTML template"
```

---

## Phase 12: Import Functionality

### Task 12.1: Add Browser-Compatible Decryption to Rust

**Files:**
- Modify: `src-tauri/src/encryption.rs`

**Step 1: Add decrypt_from_browser function**

Add to `src-tauri/src/encryption.rs`:

```rust
/// Decrypts data encrypted with encrypt_for_browser (PBKDF2 + AES-GCM)
pub fn decrypt_from_browser(payload: &EncryptedPayload, passphrase: &str) -> Result<String, EncryptionError> {
    let salt = BASE64
        .decode(&payload.salt)
        .map_err(|_| EncryptionError::InvalidData("Invalid salt".into()))?;
    let nonce_bytes: [u8; 12] = BASE64
        .decode(&payload.nonce)
        .map_err(|_| EncryptionError::InvalidData("Invalid nonce".into()))?
        .try_into()
        .map_err(|_| EncryptionError::InvalidData("Nonce wrong length".into()))?;
    let mut ciphertext = BASE64
        .decode(&payload.ciphertext)
        .map_err(|_| EncryptionError::InvalidData("Invalid ciphertext".into()))?;

    let mut key_bytes = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        std::num::NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(),
        &salt,
        passphrase.as_bytes(),
        &mut key_bytes,
    );

    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)
        .map_err(|_| EncryptionError::Decryption("Failed to create key".into()))?;
    let key = LessSafeKey::new(unbound_key);

    let nonce = Nonce::assume_unique_for_key(nonce_bytes);
    let plaintext = key
        .open_in_place(nonce, Aad::empty(), &mut ciphertext)
        .map_err(|_| EncryptionError::Decryption("Decryption failed - wrong passphrase?".into()))?;

    String::from_utf8(plaintext.to_vec())
        .map_err(|_| EncryptionError::Decryption("Invalid UTF-8".into()))
}
```

**Verification:**
```bash
cd src-tauri && cargo check
```

**Commit:**
```bash
git add src-tauri/src/encryption.rs
git commit -m "feat: add browser-compatible decryption for import"
```

---

### Task 12.2: Implement Import Command

**Files:**
- Modify: `src-tauri/src/main.rs`
- Modify: `src-tauri/Cargo.toml`

**Step 1: Add regex dependency**

Add to `src-tauri/Cargo.toml`:

```toml
regex = "1"
```

**Step 2: Update import_file command**

Replace the stub in `src-tauri/src/main.rs`:

```rust
use regex::Regex;

#[tauri::command]
fn import_file(encrypted_html: String, passphrase: String) -> Result<LegacyDocument, String> {
    let re = Regex::new(r#"const ENCRYPTED_DATA = (\{[^;]+\});"#)
        .map_err(|e| format!("Regex error: {}", e))?;

    let captures = re.captures(&encrypted_html)
        .ok_or("Could not find encrypted data in HTML file")?;

    let encrypted_json = captures.get(1)
        .ok_or("Could not extract encrypted data")?
        .as_str();

    let payload: encryption::EncryptedPayload = serde_json::from_str(encrypted_json)
        .map_err(|e| format!("Invalid encrypted data format: {}", e))?;

    let decrypted_json = encryption::decrypt_from_browser(&payload, &passphrase)
        .map_err(|e| format!("Decryption failed: {}", e))?;

    let document: LegacyDocument = serde_json::from_str(&decrypted_json)
        .map_err(|e| format!("Invalid document format: {}", e))?;

    Ok(document)
}
```

**Verification:**
```bash
cd src-tauri && cargo check
```

**Commit:**
```bash
git add src-tauri/Cargo.toml src-tauri/src/main.rs
git commit -m "feat: implement import_file command with HTML parsing"
```

---

### Task 12.3: Create ImportDialog Component

**Files:**
- Create: `src/lib/components/ImportDialog.svelte`

**Step 1: Create ImportDialog component**

Create `src/lib/components/ImportDialog.svelte`:

```svelte
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';
  import { readTextFile } from '@tauri-apps/plugin-fs';
  import { document as documentStore, type LegacyDocument } from '../stores/document';

  export let isOpen = false;

  const dispatch = createEventDispatcher();

  let passphrase = '';
  let selectedFile = '';
  let isImporting = false;
  let error = '';

  async function selectFile() {
    try {
      const selected = await open({
        filters: [{ name: 'HTML', extensions: ['html'] }],
        multiple: false
      });

      if (selected && typeof selected === 'string') {
        selectedFile = selected;
        error = '';
      }
    } catch (e) {
      error = `Failed to select file: ${e}`;
    }
  }

  async function handleImport() {
    if (!selectedFile || !passphrase) return;

    error = '';
    isImporting = true;

    try {
      const htmlContent = await readTextFile(selectedFile);
      const doc = await invoke<LegacyDocument>('import_file', {
        encryptedHtml: htmlContent,
        passphrase
      });

      await documentStore.save(doc);
      dispatch('imported', { document: doc });
      close();
    } catch (e) {
      error = `Import failed: ${e}`;
    } finally {
      isImporting = false;
    }
  }

  function close() {
    passphrase = '';
    selectedFile = '';
    error = '';
    dispatch('close');
  }
</script>

{#if isOpen}
  <div class="overlay" on:click={close} on:keydown={(e) => e.key === 'Escape' && close()}>
    <div class="dialog" on:click|stopPropagation>
      <h2>Import Legacy Document</h2>

      <div class="form">
        <div class="field">
          <label>Select File</label>
          <div class="file-select">
            <input type="text" value={selectedFile} readonly placeholder="No file selected" />
            <button type="button" on:click={selectFile}>Browse...</button>
          </div>
        </div>

        <div class="field">
          <label>Passphrase</label>
          <input
            type="password"
            bind:value={passphrase}
            placeholder="Enter the file's passphrase"
          />
        </div>

        {#if error}
          <p class="error-message">{error}</p>
        {/if}

        <p class="warning">
          Warning: Importing will replace your current document data.
        </p>
      </div>

      <div class="actions">
        <button type="button" class="btn-secondary" on:click={close}>Cancel</button>
        <button
          type="button"
          class="btn-primary"
          on:click={handleImport}
          disabled={!selectedFile || !passphrase || isImporting}
        >
          {isImporting ? 'Importing...' : 'Import'}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay { position: fixed; inset: 0; background: rgba(0, 0, 0, 0.5); display: flex; align-items: center; justify-content: center; z-index: 1000; }
  .dialog { background: white; border-radius: 12px; padding: 24px; width: 100%; max-width: 450px; }
  h2 { margin: 0 0 20px 0; }
  .form { display: flex; flex-direction: column; gap: 16px; }
  .field label { display: block; margin-bottom: 6px; font-weight: 500; }
  .field input { width: 100%; padding: 10px 12px; border: 2px solid #ddd; border-radius: 6px; }
  .file-select { display: flex; gap: 8px; }
  .file-select input { flex: 1; background: #f5f5f5; }
  .file-select button { padding: 10px 16px; background: #e0e0e0; border: none; border-radius: 6px; cursor: pointer; }
  .warning { padding: 10px 12px; background: #fff3cd; border-radius: 6px; font-size: 0.9rem; color: #856404; margin: 0; }
  .error-message { color: #dc3545; background: #f8d7da; padding: 10px 12px; border-radius: 6px; margin: 0; }
  .actions { display: flex; justify-content: flex-end; gap: 12px; margin-top: 24px; }
  .btn-primary, .btn-secondary { padding: 10px 20px; border: none; border-radius: 6px; cursor: pointer; }
  .btn-primary { background: #1976d2; color: white; }
  .btn-primary:disabled { background: #90caf9; cursor: not-allowed; }
  .btn-secondary { background: #e0e0e0; }
</style>
```

**Verification:**
```bash
npm run build
```

**Commit:**
```bash
git add src/lib/components/ImportDialog.svelte
git commit -m "feat: add ImportDialog component"
```

---

### Task 12.4: Integrate ImportDialog into App.svelte

**Files:**
- Modify: `src/App.svelte`

**Step 1: Add ImportDialog import and state**

Add to imports:
```svelte
import ImportDialog from './lib/components/ImportDialog.svelte';
```

Add state variable:
```svelte
let showImportDialog = false;
```

Update Import File button:
```svelte
<button class="btn btn-secondary" on:click={() => (showImportDialog = true)}>
  Import File
</button>
```

Add ImportDialog at end of template:
```svelte
<ImportDialog
  bind:isOpen={showImportDialog}
  on:close={() => (showImportDialog = false)}
  on:imported={() => document.load()}
/>
```

**Verification:**
```bash
npm run build
```

**Commit:**
```bash
git add src/App.svelte
git commit -m "feat: integrate ImportDialog into main app"
```

---

## Phase 13: Integration Testing

### Task 13.1: Add Encryption Module Tests

**Files:**
- Modify: `src-tauri/src/encryption.rs`

**Step 1: Add tests for browser-compatible encryption**

Add to the tests module:

```rust
#[test]
fn test_browser_encrypt_decrypt_roundtrip() {
    let plaintext = "Hello, this is secret data!";
    let passphrase = "correct-horse-battery-staple";

    let encrypted = encrypt_for_browser(plaintext, passphrase)
        .expect("encryption should succeed");
    let decrypted = decrypt_from_browser(&encrypted, passphrase)
        .expect("decryption should succeed");

    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_browser_wrong_passphrase_fails() {
    let plaintext = "Secret message";
    let encrypted = encrypt_for_browser(plaintext, "correct-passphrase")
        .expect("encryption should succeed");

    let result = decrypt_from_browser(&encrypted, "wrong-passphrase");
    assert!(result.is_err());
}
```

**Verification:**
```bash
cd src-tauri && cargo test
```

**Commit:**
```bash
git add src-tauri/src/encryption.rs
git commit -m "test: add browser-compatible encryption tests"
```

---

### Task 13.2: Create Test Script

**Files:**
- Create: `scripts/test-integration.sh`

**Step 1: Create test script**

Create `scripts/test-integration.sh`:

```bash
#!/bin/bash
set -e

echo "=== honey-did Integration Tests ==="

echo "1. Running Rust tests..."
cd src-tauri && cargo test --all && cd ..

echo "2. Building frontend..."
npm run build

echo "3. Type checking..."
npx svelte-check --tsconfig ./tsconfig.json

echo "=== All tests passed! ==="
```

**Step 2: Make executable**

```bash
chmod +x scripts/test-integration.sh
```

**Verification:**
```bash
./scripts/test-integration.sh
```

**Commit:**
```bash
git add scripts/test-integration.sh
git commit -m "test: add integration test script"
```

---

### Task 13.3: Add npm Test Scripts

**Files:**
- Modify: `package.json`

**Step 1: Add test scripts**

Add to `package.json` scripts:

```json
"test": "./scripts/test-integration.sh",
"test:rust": "cd src-tauri && cargo test",
"test:build": "npm run build && npx svelte-check --tsconfig ./tsconfig.json"
```

**Verification:**
```bash
npm run test:build
```

**Commit:**
```bash
git add package.json
git commit -m "chore: add test scripts to package.json"
```

---

## Summary

This plan covers:

- **Phase 8**: 14 tasks - Reusable form components and all 11 section components
- **Phase 9**: 2 tasks - Export dialog with passphrase generation
- **Phase 10**: 1 task - Print HTML generation
- **Phase 11**: 2 tasks - Browser-compatible PBKDF2 encryption
- **Phase 12**: 4 tasks - Import functionality with HTML parsing
- **Phase 13**: 3 tasks - Integration testing

Total: ~26 tasks, each designed to be completed in 2-5 minutes.
