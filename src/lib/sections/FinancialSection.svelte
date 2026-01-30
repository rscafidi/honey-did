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

  // --- Bank Accounts ---
  function addBankAccount() {
    local = { ...local, bank_accounts: [...local.bank_accounts, { name: '', institution: '', account_type: 'Checking', last_four: '', notes: '' }] };
    scheduleFlush();
  }

  function removeBankAccount(index: number) {
    local = { ...local, bank_accounts: local.bank_accounts.filter((_: any, i: number) => i !== index) };
    scheduleFlush();
  }

  function updateBankAccount(index: number, field: string, value: string) {
    const accounts = [...local.bank_accounts];
    accounts[index] = { ...accounts[index], [field]: value };
    local = { ...local, bank_accounts: accounts };
    scheduleFlush();
  }

  // --- Credit Cards ---
  function addCreditCard() {
    local = { ...local, credit_cards: [...local.credit_cards, { name: '', issuer: '', last_four: '', notes: '' }] };
    scheduleFlush();
  }

  function removeCreditCard(index: number) {
    local = { ...local, credit_cards: local.credit_cards.filter((_: any, i: number) => i !== index) };
    scheduleFlush();
  }

  function updateCreditCard(index: number, field: string, value: string) {
    const cards = [...local.credit_cards];
    cards[index] = { ...cards[index], [field]: value };
    local = { ...local, credit_cards: cards };
    scheduleFlush();
  }

  // --- Investments ---
  function addInvestment() {
    local = { ...local, investments: [...local.investments, { name: '', institution: '', account_type: '', notes: '' }] };
    scheduleFlush();
  }

  function removeInvestment(index: number) {
    local = { ...local, investments: local.investments.filter((_: any, i: number) => i !== index) };
    scheduleFlush();
  }

  function updateInvestment(index: number, field: string, value: string) {
    const investments = [...local.investments];
    investments[index] = { ...investments[index], [field]: value };
    local = { ...local, investments };
    scheduleFlush();
  }

  // --- Debts ---
  function addDebt() {
    local = { ...local, debts: [...local.debts, { name: '', lender: '', notes: '' }] };
    scheduleFlush();
  }

  function removeDebt(index: number) {
    local = { ...local, debts: local.debts.filter((_: any, i: number) => i !== index) };
    scheduleFlush();
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

<div class="section">
  <div class="subsection">
    <h3>Bank Accounts</h3>
    {#each local.bank_accounts as account, i}
      <ItemCard title={account.name || 'New Account'} on:delete={() => removeBankAccount(i)}>
        <FormField label="Account Name" value={account.name} on:change={(e) => updateBankAccount(i, 'name', e.detail.value)} />
        <FormField label="Institution" value={account.institution} on:change={(e) => updateBankAccount(i, 'institution', e.detail.value)} />
        <FormField label="Account Type" value={account.account_type} on:change={(e) => updateBankAccount(i, 'account_type', e.detail.value)} />
        <FormField label="Last 4 Digits" value={account.last_four} on:change={(e) => updateBankAccount(i, 'last_four', e.detail.value)} />
        <FormField label="Notes" type="textarea" value={account.notes} on:change={(e) => updateBankAccount(i, 'notes', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Bank Account" on:click={addBankAccount} />
  </div>

  <div class="subsection">
    <h3>Credit Cards</h3>
    {#each local.credit_cards as card, i}
      <ItemCard title={card.name || 'New Card'} on:delete={() => removeCreditCard(i)}>
        <FormField label="Card Name" value={card.name} on:change={(e) => updateCreditCard(i, 'name', e.detail.value)} />
        <FormField label="Issuer" value={card.issuer} on:change={(e) => updateCreditCard(i, 'issuer', e.detail.value)} />
        <FormField label="Last 4 Digits" value={card.last_four} on:change={(e) => updateCreditCard(i, 'last_four', e.detail.value)} />
        <FormField label="Notes" type="textarea" value={card.notes} on:change={(e) => updateCreditCard(i, 'notes', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Credit Card" on:click={addCreditCard} />
  </div>

  <div class="subsection">
    <h3>Investments</h3>
    {#each local.investments as investment, i}
      <ItemCard title={investment.name || 'New Investment'} on:delete={() => removeInvestment(i)}>
        <FormField label="Account Name" value={investment.name} on:change={(e) => updateInvestment(i, 'name', e.detail.value)} />
        <FormField label="Institution" value={investment.institution} on:change={(e) => updateInvestment(i, 'institution', e.detail.value)} />
        <FormField label="Account Type" value={investment.account_type} placeholder="401k, IRA, Brokerage, etc." on:change={(e) => updateInvestment(i, 'account_type', e.detail.value)} />
        <FormField label="Notes" type="textarea" value={investment.notes} on:change={(e) => updateInvestment(i, 'notes', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Investment Account" on:click={addInvestment} />
  </div>

  <div class="subsection">
    <h3>Debts & Loans</h3>
    {#each local.debts as debt, i}
      <ItemCard title={debt.name || 'New Debt'} on:delete={() => removeDebt(i)}>
        <FormField label="Description" value={debt.name} on:change={(e) => updateDebt(i, 'name', e.detail.value)} />
        <FormField label="Lender" value={debt.lender} on:change={(e) => updateDebt(i, 'lender', e.detail.value)} />
        <FormField label="Notes" type="textarea" value={debt.notes} on:change={(e) => updateDebt(i, 'notes', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Debt/Loan" on:click={addDebt} />
  </div>

  <NotesField value={local.notes} on:change={updateNotes} />

  <CustomSubsections parentId="financial" />
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
    color: var(--text-primary);
    font-size: 1.1rem;
  }

  @media (max-width: 768px) {
    .section {
      max-width: 100%;
    }
  }
</style>
