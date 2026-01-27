<script lang="ts">
  import { document } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';
  import CustomSubsections from '../components/CustomSubsections.svelte';

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
      bills: bills.bills.filter((_: any, i: number) => i !== index)
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
      <FormField label="Bill Name" value={bill.name} placeholder="Mortgage, Electric, Netflix, etc." on:change={(e) => updateBill(i, 'name', e.detail.value)} />
      <FormField label="Provider" value={bill.provider} on:change={(e) => updateBill(i, 'provider', e.detail.value)} />
      <FormField label="Approximate Amount" value={bill.amount} placeholder="$100/month" on:change={(e) => updateBill(i, 'amount', e.detail.value)} />
      <FormField label="Due Day" value={bill.due_day} placeholder="15th of each month" on:change={(e) => updateBill(i, 'due_day', e.detail.value)} />
      <FormField label="Auto-pay enabled" type="checkbox" checked={bill.autopay} on:change={(e) => updateBill(i, 'autopay', e.detail.checked ?? false)} />
      <FormField label="Notes" type="textarea" value={bill.notes} placeholder="How to pay, account info hints, etc." on:change={(e) => updateBill(i, 'notes', e.detail.value)} />
    </ItemCard>
  {/each}

  <AddButton label="Add Bill/Subscription" on:click={addBill} />
  <NotesField value={bills.notes} on:change={updateNotes} />

  <CustomSubsections parentId="bills" />
</div>

<style>
  .section { max-width: 800px; }
  .intro { color: #666; margin-bottom: 20px; }
</style>
