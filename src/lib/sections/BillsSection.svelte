<script lang="ts">
  import { onDestroy } from 'svelte';
  import { document, billsStore } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';
  import CustomSubsections from '../components/CustomSubsections.svelte';

  const defaultBills = {
    bills: [] as any[],
    notes: ''
  };

  // Local-first state: edits stay here, only flushed to store on discrete actions or debounced
  let local = { ...defaultBills };
  let hasPendingChanges = false;
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  // Sync from store ONLY when we don't have pending local changes
  const unsub = billsStore.subscribe((value) => {
    if (!hasPendingChanges) {
      local = value ?? defaultBills;
    }
  });
  onDestroy(() => {
    // Flush any pending changes before unmount
    if (hasPendingChanges) {
      if (debounceTimer) clearTimeout(debounceTimer);
      document.updateSection('bills', local);
    }
    unsub();
  });

  function scheduleFlush() {
    hasPendingChanges = true;
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      document.updateSection('bills', local);
      setTimeout(() => { hasPendingChanges = false; }, 0);
      debounceTimer = null;
    }, 300);
  }

  function flushNow(updatedLocal: typeof local) {
    local = updatedLocal;
    if (debounceTimer) { clearTimeout(debounceTimer); debounceTimer = null; }
    hasPendingChanges = false;
    document.updateSection('bills', local);
  }

  function addBill() {
    flushNow({
      ...local,
      bills: [...local.bills, {
        name: '',
        provider: '',
        amount: '',
        due_day: '',
        autopay: false,
        notes: ''
      }]
    });
  }

  function removeBill(index: number) {
    flushNow({
      ...local,
      bills: local.bills.filter((_: any, i: number) => i !== index)
    });
  }

  function updateBill(index: number, field: string, value: string | boolean) {
    const billsList = [...local.bills];
    billsList[index] = { ...billsList[index], [field]: value };
    local = { ...local, bills: billsList };
    scheduleFlush();
  }

  function updateNotes(e: Event) {
    const target = e.target as HTMLTextAreaElement;
    local = { ...local, notes: target.value };
    scheduleFlush();
  }
</script>

<div class="section">
  <p class="intro">List recurring bills and subscriptions so your family knows what needs to be paid or cancelled.</p>

  {#each local.bills as bill, i}
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
  <NotesField value={local.notes} on:change={updateNotes} />

  <CustomSubsections parentId="bills" />
</div>

<style>
  .section { max-width: 800px; }
  .intro { color: var(--text-secondary); margin-bottom: 20px; }
</style>
