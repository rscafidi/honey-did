<script lang="ts">
  import { document } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';
  import CustomSubsections from '../components/CustomSubsections.svelte';

  $: property = $document?.property ?? { properties: [], vehicles: [], valuables: [], notes: '' };

  function addProperty() {
    const updated = { ...property, properties: [...property.properties, { name: '', address: '', notes: '' }] };
    document.updateSection('property', updated);
  }

  function removeProperty(index: number) {
    const updated = { ...property, properties: property.properties.filter((_: any, i: number) => i !== index) };
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
    const updated = { ...property, vehicles: property.vehicles.filter((_: any, i: number) => i !== index) };
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
    const updated = { ...property, valuables: property.valuables.filter((_: any, i: number) => i !== index) };
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
        <FormField label="Property Name" value={prop.name} placeholder="Primary home, Rental, etc." on:change={(e) => updateProperty(i, 'name', e.detail.value)} />
        <FormField label="Address" value={prop.address} on:change={(e) => updateProperty(i, 'address', e.detail.value)} />
        <FormField label="Notes" type="textarea" value={prop.notes} placeholder="Mortgage info, deed location, etc." on:change={(e) => updateProperty(i, 'notes', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Property" on:click={addProperty} />
  </div>

  <div class="subsection">
    <h3>Vehicles</h3>
    {#each property.vehicles as vehicle, i}
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
    {#each property.valuables as valuable, i}
      <ItemCard title={valuable.name || 'New Item'} on:delete={() => removeValuable(i)}>
        <FormField label="Item" value={valuable.name} placeholder="Jewelry, safe deposit box, etc." on:change={(e) => updateValuable(i, 'name', e.detail.value)} />
        <FormField label="Location" value={valuable.location} on:change={(e) => updateValuable(i, 'location', e.detail.value)} />
        <FormField label="Notes" type="textarea" value={valuable.notes} on:change={(e) => updateValuable(i, 'notes', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Valuable/Storage" on:click={addValuable} />
  </div>

  <NotesField value={property.notes} on:change={updateNotes} />

  <CustomSubsections parentId="property" />
</div>

<style>
  .section { max-width: 800px; }
  .subsection { margin-bottom: 32px; }
  h3 { margin: 0 0 16px 0; color: var(--text-primary); font-size: 1.1rem; }
</style>
