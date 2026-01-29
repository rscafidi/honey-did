<script lang="ts">
  import { onDestroy } from 'svelte';
  import { document, householdStore } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';
  import CustomSubsections from '../components/CustomSubsections.svelte';

  const defaultHousehold = {
    maintenance_items: [] as any[],
    contractors: [] as any[],
    how_things_work: [] as any[],
    notes: ''
  };

  // Local-first state: edits stay here, only flushed to store on discrete actions or debounced
  let local = { ...defaultHousehold };
  let hasPendingChanges = false;
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  // Sync from store ONLY when we don't have pending local changes
  const unsub = householdStore.subscribe((value) => {
    if (!hasPendingChanges) {
      local = value ?? defaultHousehold;
    }
  });
  onDestroy(() => {
    // Flush any pending changes before unmount
    if (hasPendingChanges) {
      if (debounceTimer) clearTimeout(debounceTimer);
      document.updateSection('household', local);
    }
    unsub();
  });

  function scheduleFlush() {
    hasPendingChanges = true;
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      document.updateSection('household', local);
      setTimeout(() => { hasPendingChanges = false; }, 0);
      debounceTimer = null;
    }, 300);
  }

  function flushNow(updatedLocal: typeof local) {
    local = updatedLocal;
    if (debounceTimer) { clearTimeout(debounceTimer); debounceTimer = null; }
    hasPendingChanges = false;
    document.updateSection('household', local);
  }

  // --- Maintenance Items ---
  function addMaintenance() {
    flushNow({
      ...local,
      maintenance_items: [...local.maintenance_items, { name: '', frequency: '', last_done: '', notes: '' }]
    });
  }

  function removeMaintenance(index: number) {
    flushNow({
      ...local,
      maintenance_items: local.maintenance_items.filter((_: any, i: number) => i !== index)
    });
  }

  function updateMaintenance(index: number, field: string, value: string) {
    const items = [...local.maintenance_items];
    items[index] = { ...items[index], [field]: value };
    local = { ...local, maintenance_items: items };
    scheduleFlush();
  }

  // --- Contractors ---
  function addContractor() {
    flushNow({
      ...local,
      contractors: [...local.contractors, { name: '', relationship: '', phone: '', email: '', notes: '' }]
    });
  }

  function removeContractor(index: number) {
    flushNow({
      ...local,
      contractors: local.contractors.filter((_: any, i: number) => i !== index)
    });
  }

  function updateContractor(index: number, field: string, value: string) {
    const items = [...local.contractors];
    items[index] = { ...items[index], [field]: value };
    local = { ...local, contractors: items };
    scheduleFlush();
  }

  // --- How Things Work ---
  function addHowTo() {
    flushNow({
      ...local,
      how_things_work: [...local.how_things_work, { name: '', instructions: '' }]
    });
  }

  function removeHowTo(index: number) {
    flushNow({
      ...local,
      how_things_work: local.how_things_work.filter((_: any, i: number) => i !== index)
    });
  }

  function updateHowTo(index: number, field: string, value: string) {
    const items = [...local.how_things_work];
    items[index] = { ...items[index], [field]: value };
    local = { ...local, how_things_work: items };
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
    <h3>Maintenance Tasks</h3>
    <p class="hint">Regular maintenance that needs to happen to keep the house running.</p>
    {#each local.maintenance_items as item, i}
      <ItemCard title={item.name || 'New Task'} on:delete={() => removeMaintenance(i)}>
        <FormField label="Task" value={item.name} placeholder="Change HVAC filter, service furnace, etc." on:change={(e) => updateMaintenance(i, 'name', e.detail.value)} />
        <FormField label="Frequency" value={item.frequency} placeholder="Monthly, Annually, etc." on:change={(e) => updateMaintenance(i, 'frequency', e.detail.value)} />
        <FormField label="Last Done" value={item.last_done} placeholder="January 2024" on:change={(e) => updateMaintenance(i, 'last_done', e.detail.value)} />
        <FormField label="Notes" type="textarea" value={item.notes} on:change={(e) => updateMaintenance(i, 'notes', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Maintenance Task" on:click={addMaintenance} />
  </div>

  <div class="subsection">
    <h3>Contractors & Service Providers</h3>
    {#each local.contractors as contractor, i}
      <ItemCard title={contractor.name || 'New Contractor'} on:delete={() => removeContractor(i)}>
        <FormField label="Name/Company" value={contractor.name} on:change={(e) => updateContractor(i, 'name', e.detail.value)} />
        <FormField label="Service" value={contractor.relationship} placeholder="Plumber, Electrician, Lawn care, etc." on:change={(e) => updateContractor(i, 'relationship', e.detail.value)} />
        <FormField label="Phone" value={contractor.phone} on:change={(e) => updateContractor(i, 'phone', e.detail.value)} />
        <FormField label="Email" value={contractor.email} on:change={(e) => updateContractor(i, 'email', e.detail.value)} />
        <FormField label="Notes" type="textarea" value={contractor.notes} on:change={(e) => updateContractor(i, 'notes', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Contractor" on:click={addContractor} />
  </div>

  <div class="subsection">
    <h3>How Things Work</h3>
    <p class="hint">Explain things only you know how to do around the house.</p>
    {#each local.how_things_work as howto, i}
      <ItemCard title={howto.name || 'New How-To'} on:delete={() => removeHowTo(i)}>
        <FormField label="What" value={howto.name} placeholder="Turn off water main, reset breaker, etc." on:change={(e) => updateHowTo(i, 'name', e.detail.value)} />
        <FormField label="Instructions" type="textarea" value={howto.instructions} placeholder="Step-by-step instructions..." on:change={(e) => updateHowTo(i, 'instructions', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add How-To" on:click={addHowTo} />
  </div>

  <NotesField value={local.notes} on:change={updateNotes} />

  <CustomSubsections parentId="household" />
</div>

<style>
  .section { max-width: 800px; }
  .subsection { margin-bottom: 32px; }
  h3 { margin: 0 0 8px 0; color: var(--text-primary); font-size: 1.1rem; }
  .hint { color: var(--text-secondary); font-size: 0.9rem; margin-bottom: 16px; }
</style>
