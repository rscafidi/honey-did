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
    const updated = { ...household, maintenance_items: household.maintenance_items.filter((_: any, i: number) => i !== index) };
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
    const updated = { ...household, contractors: household.contractors.filter((_: any, i: number) => i !== index) };
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
    const updated = { ...household, how_things_work: household.how_things_work.filter((_: any, i: number) => i !== index) };
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
    {#each household.contractors as contractor, i}
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
    {#each household.how_things_work as howto, i}
      <ItemCard title={howto.name || 'New How-To'} on:delete={() => removeHowTo(i)}>
        <FormField label="What" value={howto.name} placeholder="Turn off water main, reset breaker, etc." on:change={(e) => updateHowTo(i, 'name', e.detail.value)} />
        <FormField label="Instructions" type="textarea" value={howto.instructions} placeholder="Step-by-step instructions..." on:change={(e) => updateHowTo(i, 'instructions', e.detail.value)} />
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
