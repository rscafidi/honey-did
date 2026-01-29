<script lang="ts">
  import { onDestroy } from 'svelte';
  import { document, propertyStore } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';
  import CustomSubsections from '../components/CustomSubsections.svelte';

  const defaultProperty = {
    properties: [] as any[],
    vehicles: [] as any[],
    valuables: [] as any[],
    notes: ''
  };

  // Local-first state: edits stay here, only flushed to store on discrete actions or debounced
  let local = { ...defaultProperty };
  let hasPendingChanges = false;
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  // Sync from store ONLY when we don't have pending local changes
  const unsub = propertyStore.subscribe((value) => {
    if (!hasPendingChanges) {
      local = value ?? defaultProperty;
    }
  });
  onDestroy(() => {
    // Flush any pending changes before unmount
    if (hasPendingChanges) {
      if (debounceTimer) clearTimeout(debounceTimer);
      document.updateSection('property', local);
    }
    unsub();
  });

  function scheduleFlush() {
    hasPendingChanges = true;
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      document.updateSection('property', local);
      setTimeout(() => { hasPendingChanges = false; }, 0);
      debounceTimer = null;
    }, 300);
  }

  // --- Properties ---
  function addProperty() {
    local = { ...local, properties: [...local.properties, { name: '', address: '', notes: '' }] };
    scheduleFlush();
  }

  function removeProperty(index: number) {
    local = { ...local, properties: local.properties.filter((_: any, i: number) => i !== index) };
    scheduleFlush();
  }

  function updateProperty(index: number, field: string, value: string) {
    const items = [...local.properties];
    items[index] = { ...items[index], [field]: value };
    local = { ...local, properties: items };
    scheduleFlush();
  }

  // --- Vehicles ---
  function addVehicle() {
    local = { ...local, vehicles: [...local.vehicles, { name: '', details: '', notes: '' }] };
    scheduleFlush();
  }

  function removeVehicle(index: number) {
    local = { ...local, vehicles: local.vehicles.filter((_: any, i: number) => i !== index) };
    scheduleFlush();
  }

  function updateVehicle(index: number, field: string, value: string) {
    const items = [...local.vehicles];
    items[index] = { ...items[index], [field]: value };
    local = { ...local, vehicles: items };
    scheduleFlush();
  }

  // --- Valuables ---
  function addValuable() {
    local = { ...local, valuables: [...local.valuables, { name: '', location: '', notes: '' }] };
    scheduleFlush();
  }

  function removeValuable(index: number) {
    local = { ...local, valuables: local.valuables.filter((_: any, i: number) => i !== index) };
    scheduleFlush();
  }

  function updateValuable(index: number, field: string, value: string) {
    const items = [...local.valuables];
    items[index] = { ...items[index], [field]: value };
    local = { ...local, valuables: items };
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
    <h3>Real Estate</h3>
    {#each local.properties as prop, i}
      <ItemCard title={prop.name || 'New Property'} on:delete={() => removeProperty(i)}>
        <FormField label="Property Name" value={prop.name} placeholder="Primary home, Rental, etc." on:change={(e) => updateProperty(i, 'name', e.detail.value)} />
        <FormField label="Address" value={prop.address} on:change={(e) => updateProperty(i, 'address', e.detail.value)} />
        <FormField label="Notes" type="textarea" value={prop.notes} placeholder="Mortgage info, deed location, etc." on:change={(e) => updateProperty(i, 'notes', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Property" on:click={addProperty} />
  </div>

  <div class="subsection">
    <h3>Vehicles</h3>
    {#each local.vehicles as vehicle, i}
      <ItemCard title={vehicle.name || 'New Vehicle'} on:delete={() => removeVehicle(i)}>
        <FormField label="Vehicle" value={vehicle.name} placeholder="2020 Honda Accord" on:change={(e) => updateVehicle(i, 'name', e.detail.value)} />
        <FormField label="Details" value={vehicle.details} placeholder="VIN, license plate, loan info" on:change={(e) => updateVehicle(i, 'details', e.detail.value)} />
        <FormField label="Notes" type="textarea" value={vehicle.notes} on:change={(e) => updateVehicle(i, 'notes', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Vehicle" on:click={addVehicle} />
  </div>

  <div class="subsection">
    <h3>Valuables & Storage</h3>
    {#each local.valuables as valuable, i}
      <ItemCard title={valuable.name || 'New Item'} on:delete={() => removeValuable(i)}>
        <FormField label="Item" value={valuable.name} placeholder="Jewelry, safe deposit box, etc." on:change={(e) => updateValuable(i, 'name', e.detail.value)} />
        <FormField label="Location" value={valuable.location} on:change={(e) => updateValuable(i, 'location', e.detail.value)} />
        <FormField label="Notes" type="textarea" value={valuable.notes} on:change={(e) => updateValuable(i, 'notes', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Valuable/Storage" on:click={addValuable} />
  </div>

  <NotesField value={local.notes} on:change={updateNotes} />

  <CustomSubsections parentId="property" />
</div>

<style>
  .section { max-width: 800px; }
  .subsection { margin-bottom: 32px; }
  h3 { margin: 0 0 16px 0; color: var(--text-primary); font-size: 1.1rem; }
</style>
