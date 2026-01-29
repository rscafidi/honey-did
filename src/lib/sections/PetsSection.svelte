<script lang="ts">
  import { document } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';
  import CustomSubsections from '../components/CustomSubsections.svelte';

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
    const updated = { ...pets, pets: pets.pets.filter((_: any, i: number) => i !== index) };
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

  function inputValue(e: Event): string {
    return (e.target as HTMLInputElement).value;
  }

  function removeAtIndex<T>(arr: T[], index: number): T[] {
    return arr.filter((_, k) => k !== index);
  }
</script>

<div class="section">
  <p class="intro">Make sure your pets are cared for by documenting their needs and care providers.</p>

  {#each pets.pets as pet, i}
    <ItemCard title={pet.name || 'New Pet'} on:delete={() => removePet(i)}>
      <div class="row">
        <FormField label="Name" value={pet.name} on:change={(e) => updatePet(i, 'name', e.detail.value)} />
        <FormField label="Species" value={pet.species} placeholder="Dog, Cat, etc." on:change={(e) => updatePet(i, 'species', e.detail.value)} />
        <FormField label="Breed" value={pet.breed} on:change={(e) => updatePet(i, 'breed', e.detail.value)} />
      </div>

      <div class="sub-section">
        <h4>Veterinarian</h4>
        <FormField label="Clinic/Vet Name" value={pet.vet?.name || ''} on:change={(e) => updatePet(i, 'vet', { ...pet.vet, name: e.detail.value })} />
        <FormField label="Phone" value={pet.vet?.phone || ''} on:change={(e) => updatePet(i, 'vet', { ...pet.vet, phone: e.detail.value })} />
        <FormField label="Notes" value={pet.vet?.notes || ''} on:change={(e) => updatePet(i, 'vet', { ...pet.vet, notes: e.detail.value })} />
      </div>

      <div class="sub-section">
        <h4>Medications</h4>
        {#each pet.medications || [] as med, j}
          <div class="med-row">
            <input placeholder="Medication" value={med.name} on:change={(e) => {
              const meds = [...(pet.medications || [])];
              meds[j] = { ...meds[j], name: inputValue(e) };
              updatePet(i, 'medications', meds);
            }} />
            <input placeholder="Dosage" value={med.dosage} on:change={(e) => {
              const meds = [...(pet.medications || [])];
              meds[j] = { ...meds[j], dosage: inputValue(e) };
              updatePet(i, 'medications', meds);
            }} />
            <input placeholder="Frequency" value={med.frequency} on:change={(e) => {
              const meds = [...(pet.medications || [])];
              meds[j] = { ...meds[j], frequency: inputValue(e) };
              updatePet(i, 'medications', meds);
            }} />
            <button class="remove-btn" on:click={() => {
              updatePet(i, 'medications', removeAtIndex(pet.medications || [], j));
            }}>×</button>
          </div>
        {/each}
        <button class="add-small" on:click={() => {
          const meds = [...(pet.medications || []), { ...emptyMedication }];
          updatePet(i, 'medications', meds);
        }}>+ Add Medication</button>
      </div>

      <FormField label="Feeding Instructions" type="textarea" value={pet.feeding} placeholder="What, when, and how much to feed..." on:change={(e) => updatePet(i, 'feeding', e.detail.value)} />
      <FormField label="Care Notes" type="textarea" value={pet.care_notes} placeholder="Walks, grooming, behavior notes, favorite things..." on:change={(e) => updatePet(i, 'care_notes', e.detail.value)} />
    </ItemCard>
  {/each}

  <AddButton label="Add Pet" on:click={addPet} />
  <NotesField value={pets.notes} on:change={updateNotes} />

  <CustomSubsections parentId="pets" />
</div>

<style>
  .section { max-width: 800px; }
  .intro { color: var(--text-secondary); margin-bottom: 24px; }
  .row { display: grid; grid-template-columns: repeat(3, 1fr); gap: 12px; }
  .sub-section { margin: 16px 0; padding: 12px; background: var(--bg-tertiary); border-radius: 6px; }
  .sub-section h4 { margin: 0 0 12px 0; font-size: 0.95rem; color: var(--text-secondary); }
  .med-row { display: flex; gap: 8px; margin-bottom: 8px; }
  .med-row input { flex: 1; padding: 6px 10px; border: 1px solid var(--border-color); border-radius: 4px; box-sizing: border-box; background: var(--bg-secondary); color: var(--text-primary); }
  .remove-btn { background: none; border: none; color: var(--text-muted); font-size: 1.2rem; cursor: pointer; padding: 0 8px; }
  .remove-btn:hover { color: var(--error-color); }
  .add-small { background: none; border: none; color: var(--accent-primary); cursor: pointer; font-size: 0.9rem; padding: 4px 0; }
  .add-small:hover { text-decoration: underline; }
</style>
