<script lang="ts">
  import { onDestroy } from 'svelte';
  import { document, medicalStore } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';
  import CustomSubsections from '../components/CustomSubsections.svelte';

  const emptyContact = { name: '', relationship: '', phone: '', email: '', notes: '' };
  const emptyMedication = { name: '', dosage: '', frequency: '', prescriber: '', notes: '' };
  const emptyDoctor = { name: '', specialty: '', phone: '', email: '', notes: '' };

  const defaultMedical = {
    family_members: [] as any[],
    notes: ''
  };

  // Local-first state: edits stay here, only flushed to store on discrete actions or debounced
  let local = { ...defaultMedical };
  let hasPendingChanges = false;
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  // Sync from store ONLY when we don't have pending local changes
  const unsub = medicalStore.subscribe((value) => {
    if (!hasPendingChanges) {
      local = value ?? defaultMedical;
    }
  });
  onDestroy(() => {
    // Flush any pending changes before unmount
    if (hasPendingChanges) {
      if (debounceTimer) clearTimeout(debounceTimer);
      document.updateSection('medical', local);
    }
    unsub();
  });

  function scheduleFlush() {
    hasPendingChanges = true;
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      document.updateSection('medical', local);
      setTimeout(() => { hasPendingChanges = false; }, 0);
      debounceTimer = null;
    }, 300);
  }

  function addFamilyMember() {
    local = {
      ...local,
      family_members: [...local.family_members, {
        name: '',
        doctors: [],
        medications: [],
        conditions: [],
        allergies: [],
        pharmacy: { ...emptyContact },
        notes: ''
      }]
    };
    scheduleFlush();
  }

  function removeFamilyMember(index: number) {
    local = { ...local, family_members: local.family_members.filter((_: any, i: number) => i !== index) };
    scheduleFlush();
  }

  function updateFamilyMember(index: number, field: string, value: any) {
    const members = [...local.family_members];
    members[index] = { ...members[index], [field]: value };
    local = { ...local, family_members: members };
    scheduleFlush();
  }

  function updateNotes(e: Event) {
    local = { ...local, notes: (e.target as HTMLTextAreaElement).value };
    scheduleFlush();
  }

  function inputValue(e: Event): string {
    return (e.target as HTMLInputElement).value;
  }

  function removeAtIndex<T>(arr: T[], index: number): T[] {
    return arr.filter((_, k) => k !== index);
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

  {#each local.family_members as member, i}
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
        <h4>Doctors</h4>
        {#each member.doctors || [] as doctor, j}
          <div class="doctor-card">
            <div class="doctor-header">
              <span class="doctor-name">{doctor.name || 'New Doctor'}</span>
              {#if doctor.specialty}
                <span class="doctor-specialty">{doctor.specialty}</span>
              {/if}
              <button class="remove-btn" on:click={() => {
                updateFamilyMember(i, 'doctors', removeAtIndex(member.doctors || [], j));
              }}>×</button>
            </div>
            <div class="doctor-fields">
              <input placeholder="Doctor Name" value={doctor.name} on:change={(e) => {
                const docs = [...(member.doctors || [])];
                docs[j] = { ...docs[j], name: inputValue(e) };
                updateFamilyMember(i, 'doctors', docs);
              }} />
              <input placeholder="Specialty (e.g., Cardiologist, Primary Care)" value={doctor.specialty} on:change={(e) => {
                const docs = [...(member.doctors || [])];
                docs[j] = { ...docs[j], specialty: inputValue(e) };
                updateFamilyMember(i, 'doctors', docs);
              }} />
              <input placeholder="Phone" value={doctor.phone} on:change={(e) => {
                const docs = [...(member.doctors || [])];
                docs[j] = { ...docs[j], phone: inputValue(e) };
                updateFamilyMember(i, 'doctors', docs);
              }} />
              <input placeholder="Email" value={doctor.email} on:change={(e) => {
                const docs = [...(member.doctors || [])];
                docs[j] = { ...docs[j], email: inputValue(e) };
                updateFamilyMember(i, 'doctors', docs);
              }} />
              <textarea placeholder="Notes (office hours, special instructions, etc.)" value={doctor.notes} on:change={(e) => {
                const docs = [...(member.doctors || [])];
                docs[j] = { ...docs[j], notes: inputValue(e) };
                updateFamilyMember(i, 'doctors', docs);
              }}></textarea>
            </div>
          </div>
        {/each}
        <button class="add-small" on:click={() => {
          const docs = [...(member.doctors || []), { ...emptyDoctor }];
          updateFamilyMember(i, 'doctors', docs);
        }}>+ Add Doctor</button>
      </div>

      <div class="sub-section">
        <h4>Medications</h4>
        {#each member.medications || [] as med, j}
          <div class="med-row">
            <input placeholder="Medication" value={med.name} on:change={(e) => {
              const meds = [...(member.medications || [])];
              meds[j] = { ...meds[j], name: inputValue(e) };
              updateFamilyMember(i, 'medications', meds);
            }} />
            <input placeholder="Dosage" value={med.dosage} on:change={(e) => {
              const meds = [...(member.medications || [])];
              meds[j] = { ...meds[j], dosage: inputValue(e) };
              updateFamilyMember(i, 'medications', meds);
            }} />
            <input placeholder="Frequency" value={med.frequency} on:change={(e) => {
              const meds = [...(member.medications || [])];
              meds[j] = { ...meds[j], frequency: inputValue(e) };
              updateFamilyMember(i, 'medications', meds);
            }} />
            <button class="remove-btn" on:click={() => {
              updateFamilyMember(i, 'medications', removeAtIndex(member.medications || [], j));
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
  <NotesField value={local.notes} on:change={updateNotes} />

  <CustomSubsections parentId="medical" />
</div>

<style>
  .section { max-width: 800px; }
  .intro { color: var(--text-secondary); margin-bottom: 24px; }
  .sub-section { margin: 16px 0; padding: 12px; background: var(--bg-tertiary); border-radius: 6px; }
  .sub-section h4 { margin: 0 0 12px 0; font-size: 0.95rem; color: var(--text-secondary); }

  /* Doctor card styles */
  .doctor-card { background: var(--bg-secondary); border: 1px solid var(--border-color); border-radius: 6px; margin-bottom: 12px; overflow: hidden; }
  .doctor-header { display: flex; align-items: center; gap: 8px; padding: 10px 12px; background: var(--bg-primary); border-bottom: 1px solid var(--border-color); }
  .doctor-name { font-weight: 600; color: var(--text-primary); flex: 1; }
  .doctor-specialty { font-size: 0.85rem; color: var(--accent-secondary); background: var(--accent-light); padding: 2px 8px; border-radius: 4px; }
  .doctor-fields { padding: 12px; display: flex; flex-direction: column; gap: 8px; }
  .doctor-fields input, .doctor-fields textarea { width: 100%; padding: 8px 10px; border: 1px solid var(--border-color); border-radius: 4px; box-sizing: border-box; background: var(--bg-secondary); color: var(--text-primary); font-family: inherit; font-size: 0.9rem; }
  .doctor-fields input:focus, .doctor-fields textarea:focus { outline: none; border-color: var(--accent-primary); }
  .doctor-fields textarea { resize: vertical; min-height: 60px; }

  /* Medication row styles */
  .med-row { display: flex; gap: 8px; margin-bottom: 8px; }
  .med-row input { flex: 1; padding: 6px 10px; border: 1px solid var(--border-color); border-radius: 4px; box-sizing: border-box; background: var(--bg-secondary); color: var(--text-primary); }

  /* Common styles */
  .remove-btn { background: none; border: none; color: var(--text-muted); font-size: 1.2rem; cursor: pointer; padding: 0 8px; }
  .remove-btn:hover { color: var(--error-color); }
  .add-small { background: none; border: none; color: var(--accent-primary); cursor: pointer; font-size: 0.9rem; padding: 4px 0; }
  .add-small:hover { text-decoration: underline; }
</style>
