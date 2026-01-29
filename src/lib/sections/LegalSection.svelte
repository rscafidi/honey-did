<script lang="ts">
  import { onDestroy } from 'svelte';
  import { document, legalStore } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';
  import CustomSubsections from '../components/CustomSubsections.svelte';

  const defaultLegal = {
    will_location: '',
    attorney: { name: '', relationship: '', phone: '', email: '', notes: '' },
    power_of_attorney: '',
    trusts: [] as any[],
    notes: ''
  };

  // Local-first state: edits stay here, only flushed to store on discrete actions or debounced
  let local = { ...defaultLegal };
  let hasPendingChanges = false;
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  // Sync from store ONLY when we don't have pending local changes
  const unsub = legalStore.subscribe((value) => {
    if (!hasPendingChanges) {
      local = value ?? defaultLegal;
    }
  });
  onDestroy(() => {
    // Flush any pending changes before unmount
    if (hasPendingChanges) {
      if (debounceTimer) clearTimeout(debounceTimer);
      document.updateSection('legal', local);
    }
    unsub();
  });

  function scheduleFlush() {
    hasPendingChanges = true;
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      document.updateSection('legal', local);
      setTimeout(() => { hasPendingChanges = false; }, 0);
      debounceTimer = null;
    }, 300);
  }

  function flushNow(updatedLocal: typeof local) {
    local = updatedLocal;
    if (debounceTimer) { clearTimeout(debounceTimer); debounceTimer = null; }
    hasPendingChanges = false;
    document.updateSection('legal', local);
  }

  function updateField(field: string, value: string) {
    local = { ...local, [field]: value };
    scheduleFlush();
  }

  function updateAttorney(field: string, value: string) {
    local = { ...local, attorney: { ...local.attorney, [field]: value } };
    scheduleFlush();
  }

  function addTrust() {
    flushNow({
      ...local,
      trusts: [...local.trusts, { name: '', trustee: '', notes: '' }]
    });
  }

  function removeTrust(index: number) {
    flushNow({
      ...local,
      trusts: local.trusts.filter((_: any, i: number) => i !== index)
    });
  }

  function updateTrust(index: number, field: string, value: string) {
    const trusts = [...local.trusts];
    trusts[index] = { ...trusts[index], [field]: value };
    local = { ...local, trusts };
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
    <h3>Will & Estate Documents</h3>
    <FormField label="Will Location" value={local.will_location} placeholder="Where is your will stored?" on:change={(e) => updateField('will_location', e.detail.value)} />
    <FormField label="Power of Attorney" value={local.power_of_attorney} placeholder="Who has power of attorney?" on:change={(e) => updateField('power_of_attorney', e.detail.value)} />
  </div>

  <div class="subsection">
    <h3>Attorney</h3>
    <div class="attorney-card">
      <FormField label="Name" value={local.attorney?.name || ''} on:change={(e) => updateAttorney('name', e.detail.value)} />
      <FormField label="Firm/Relationship" value={local.attorney?.relationship || ''} on:change={(e) => updateAttorney('relationship', e.detail.value)} />
      <FormField label="Phone" value={local.attorney?.phone || ''} on:change={(e) => updateAttorney('phone', e.detail.value)} />
      <FormField label="Email" value={local.attorney?.email || ''} on:change={(e) => updateAttorney('email', e.detail.value)} />
      <FormField label="Notes" type="textarea" value={local.attorney?.notes || ''} on:change={(e) => updateAttorney('notes', e.detail.value)} />
    </div>
  </div>

  <div class="subsection">
    <h3>Trusts</h3>
    {#each local.trusts as trust, i}
      <ItemCard title={trust.name || 'New Trust'} on:delete={() => removeTrust(i)}>
        <FormField label="Trust Name" value={trust.name} on:change={(e) => updateTrust(i, 'name', e.detail.value)} />
        <FormField label="Trustee" value={trust.trustee} on:change={(e) => updateTrust(i, 'trustee', e.detail.value)} />
        <FormField label="Notes" type="textarea" value={trust.notes} on:change={(e) => updateTrust(i, 'notes', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Trust" on:click={addTrust} />
  </div>

  <NotesField value={local.notes} on:change={updateNotes} />

  <CustomSubsections parentId="legal" />
</div>

<style>
  .section { max-width: 800px; }
  .subsection { margin-bottom: 32px; }
  h3 { margin: 0 0 16px 0; color: var(--text-primary); font-size: 1.1rem; }
  .attorney-card { background: var(--bg-tertiary); padding: 16px; border-radius: 8px; }
</style>
