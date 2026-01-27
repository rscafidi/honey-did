<script lang="ts">
  import { document } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';
  import CustomSubsections from '../components/CustomSubsections.svelte';

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
      bank_accounts: financial.bank_accounts.filter((_: any, i: number) => i !== index)
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
      credit_cards: financial.credit_cards.filter((_: any, i: number) => i !== index)
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
      investments: financial.investments.filter((_: any, i: number) => i !== index)
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
      debts: financial.debts.filter((_: any, i: number) => i !== index)
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
    {#each financial.credit_cards as card, i}
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
    {#each financial.investments as investment, i}
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
    {#each financial.debts as debt, i}
      <ItemCard title={debt.name || 'New Debt'} on:delete={() => removeDebt(i)}>
        <FormField label="Description" value={debt.name} on:change={(e) => updateDebt(i, 'name', e.detail.value)} />
        <FormField label="Lender" value={debt.lender} on:change={(e) => updateDebt(i, 'lender', e.detail.value)} />
        <FormField label="Notes" type="textarea" value={debt.notes} on:change={(e) => updateDebt(i, 'notes', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Debt/Loan" on:click={addDebt} />
  </div>

  <NotesField value={financial.notes} on:change={updateNotes} />

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
    color: #333;
    font-size: 1.1rem;
  }
</style>
