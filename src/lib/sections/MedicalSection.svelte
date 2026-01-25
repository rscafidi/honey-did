<script lang="ts">
  import { document } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';

  const emptyContact = { name: '', relationship: '', phone: '', email: '', notes: '' };
  const emptyMedication = { name: '', dosage: '', frequency: '', prescriber: '', notes: '' };

  $: medical = $document?.medical ?? { family_members: [], notes: '' };

  function addFamilyMember() {
    const updated = {
      ...medical,
      family_members: [...medical.family_members, {
        name: '',
        doctors: [],
        medications: [],
        conditions: [],
        allergies: [],
        pharmacy: { ...emptyContact },
        notes: ''
      }]
    };
    document.updateSection('medical', updated);
  }

  function removeFamilyMember(index: number) {
    const updated = { ...medical, family_members: medical.family_members.filter((_: any, i: number) => i !== index) };
    document.updateSection('medical', updated);
  }

  function updateFamilyMember(index: number, field: string, value: any) {
    const members = [...medical.family_members];
    members[index] = { ...members[index], [field]: value };
    document.updateSection('medical', { ...medical, family_members: members });
  }

  function updateNotes(e: Event) {
    document.updateSection('medical', { ...medical, notes: (e.target as HTMLTextAreaElement).value });
  }

  function parseList(value: string): string[] {
    return value.split(',').map(s => s.trim()).filter(s => s);
  }

  function formatList(arr: string[]): string {
    return (arr || []).join(', ');
  }
</script>

<div class="section">
  <p class="intro">Document medical information for each family member. This can be critical in emergencies.</p>

  {#each medical.family_members as member, i}
    <ItemCard title={member.name || 'New Family Member'} on:delete={() => removeFamilyMember(i)}>
      <FormField label="Name" value={member.name} on:change={(e) => updateFamilyMember(i, 'name', e.detail.value)} />

      <FormField
        label="Conditions (comma-separated)"
        value={formatList(member.conditions)}
        placeholder="Diabetes, High blood pressure, etc."
        on:change={(e) => updateFamilyMember(i, 'conditions', parseList(e.detail.value))}
      />

      <FormField
        label="Allergies (comma-separated)"
        value={formatList(member.allergies)}
        placeholder="Penicillin, Peanuts, etc."
        on:change={(e) => updateFamilyMember(i, 'allergies', parseList(e.detail.value))}
      />

      <div class="sub-section">
        <h4>Medications</h4>
        {#each member.medications || [] as med, j}
          <div class="med-row">
            <input placeholder="Medication" value={med.name} on:change={(e) => {
              const target = e.target; if (!target) return;
              const meds = [...(member.medications || [])];
              meds[j] = { ...meds[j], name: target.value };
              updateFamilyMember(i, 'medications', meds);
            }} />
            <input placeholder="Dosage" value={med.dosage} on:change={(e) => {
              const target = e.target; if (!target) return;
              const meds = [...(member.medications || [])];
              meds[j] = { ...meds[j], dosage: target.value };
              updateFamilyMember(i, 'medications', meds);
            }} />
            <input placeholder="Frequency" value={med.frequency} on:change={(e) => {
              const target = e.target; if (!target) return;
              const meds = [...(member.medications || [])];
              meds[j] = { ...meds[j], frequency: target.value };
              updateFamilyMember(i, 'medications', meds);
            }} />
            <button class="remove-btn" on:click={() => {
              const meds = (member.medications || []).filter((_, k) => k !== j);
              updateFamilyMember(i, 'medications', meds);
            }}>×</button>
          </div>
        {/each}
        <button class="add-small" on:click={() => {
          const meds = [...(member.medications || []), { ...emptyMedication }];
          updateFamilyMember(i, 'medications', meds);
        }}>+ Add Medication</button>
      </div>

      <div class="sub-section">
        <h4>Pharmacy</h4>
        <FormField label="Name" value={member.pharmacy?.name || ''} on:change={(e) => updateFamilyMember(i, 'pharmacy', { ...member.pharmacy, name: e.detail.value })} />
        <FormField label="Phone" value={member.pharmacy?.phone || ''} on:change={(e) => updateFamilyMember(i, 'pharmacy', { ...member.pharmacy, phone: e.detail.value })} />
      </div>

      <FormField label="Notes" type="textarea" value={member.notes} on:change={(e) => updateFamilyMember(i, 'notes', e.detail.value)} />
    </ItemCard>
  {/each}

  <AddButton label="Add Family Member" on:click={addFamilyMember} />
  <NotesField value={medical.notes} on:change={updateNotes} />
</div>

<style>
  .section { max-width: 800px; }
  .intro { color: #666; margin-bottom: 24px; }
  .sub-section { margin: 16px 0; padding: 12px; background: #f8f9fa; border-radius: 6px; }
  .sub-section h4 { margin: 0 0 12px 0; font-size: 0.95rem; color: #555; }
  .med-row { display: flex; gap: 8px; margin-bottom: 8px; }
  .med-row input { flex: 1; padding: 6px 10px; border: 1px solid #ddd; border-radius: 4px; box-sizing: border-box; }
  .remove-btn { background: none; border: none; color: #999; font-size: 1.2rem; cursor: pointer; padding: 0 8px; }
  .remove-btn:hover { color: #dc3545; }
  .add-small { background: none; border: none; color: #1976d2; cursor: pointer; font-size: 0.9rem; padding: 4px 0; }
  .add-small:hover { text-decoration: underline; }
</style>
