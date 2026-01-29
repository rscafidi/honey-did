<script lang="ts">
  import { onDestroy } from 'svelte';
  import { document, contactsStore } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';
  import CustomSubsections from '../components/CustomSubsections.svelte';

  const emptyContact = { name: '', relationship: '', phone: '', email: '', notes: '' };

  const defaultContacts = {
    emergency_contacts: [] as any[],
    family: [] as any[],
    professionals: [] as any[],
    notes: ''
  };

  // Local-first state: edits stay here, only flushed to store on discrete actions or debounced
  let local = { ...defaultContacts };
  let hasPendingChanges = false;
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  // Sync from store ONLY when we don't have pending local changes
  const unsub = contactsStore.subscribe((value) => {
    if (!hasPendingChanges) {
      local = value ?? defaultContacts;
    }
  });
  onDestroy(() => {
    // Flush any pending changes before unmount
    if (hasPendingChanges) {
      if (debounceTimer) clearTimeout(debounceTimer);
      document.updateSection('contacts', local);
    }
    unsub();
  });

  function scheduleFlush() {
    hasPendingChanges = true;
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      document.updateSection('contacts', local);
      setTimeout(() => { hasPendingChanges = false; }, 0);
      debounceTimer = null;
    }, 300);
  }

  function flushNow(updatedLocal: typeof local) {
    local = updatedLocal;
    if (debounceTimer) { clearTimeout(debounceTimer); debounceTimer = null; }
    hasPendingChanges = false;
    document.updateSection('contacts', local);
  }

  function addContact(list: 'emergency_contacts' | 'family' | 'professionals') {
    flushNow({ ...local, [list]: [...local[list], { ...emptyContact }] });
  }

  function removeContact(list: 'emergency_contacts' | 'family' | 'professionals', index: number) {
    flushNow({ ...local, [list]: local[list].filter((_: any, i: number) => i !== index) });
  }

  function updateContact(list: 'emergency_contacts' | 'family' | 'professionals', index: number, field: string, value: string) {
    const items = [...local[list]];
    items[index] = { ...items[index], [field]: value };
    local = { ...local, [list]: items };
    scheduleFlush();
  }

  function updateNotes(e: Event) {
    local = { ...local, notes: (e.target as HTMLTextAreaElement).value };
    scheduleFlush();
  }
</script>

<div class="section">
  <div class="subsection emergency">
    <h3>Emergency Contacts</h3>
    <p class="hint">Who should be called first in an emergency?</p>
    {#each local.emergency_contacts as contact, i}
      <ItemCard title={contact.name || 'New Contact'} on:delete={() => removeContact('emergency_contacts', i)}>
        <FormField label="Name" value={contact.name} on:change={(e) => updateContact('emergency_contacts', i, 'name', e.detail.value)} />
        <FormField label="Relationship" value={contact.relationship} on:change={(e) => updateContact('emergency_contacts', i, 'relationship', e.detail.value)} />
        <FormField label="Phone" value={contact.phone} on:change={(e) => updateContact('emergency_contacts', i, 'phone', e.detail.value)} />
        <FormField label="Email" value={contact.email} on:change={(e) => updateContact('emergency_contacts', i, 'email', e.detail.value)} />
        <FormField label="Notes" type="textarea" value={contact.notes} placeholder="When to call, what they can help with..." on:change={(e) => updateContact('emergency_contacts', i, 'notes', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Emergency Contact" on:click={() => addContact('emergency_contacts')} />
  </div>

  <div class="subsection">
    <h3>Family Members</h3>
    {#each local.family as contact, i}
      <ItemCard title={contact.name || 'New Family Member'} on:delete={() => removeContact('family', i)}>
        <FormField label="Name" value={contact.name} on:change={(e) => updateContact('family', i, 'name', e.detail.value)} />
        <FormField label="Relationship" value={contact.relationship} on:change={(e) => updateContact('family', i, 'relationship', e.detail.value)} />
        <FormField label="Phone" value={contact.phone} on:change={(e) => updateContact('family', i, 'phone', e.detail.value)} />
        <FormField label="Email" value={contact.email} on:change={(e) => updateContact('family', i, 'email', e.detail.value)} />
        <FormField label="Notes" type="textarea" value={contact.notes} on:change={(e) => updateContact('family', i, 'notes', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Family Member" on:click={() => addContact('family')} />
  </div>

  <div class="subsection">
    <h3>Professionals</h3>
    <p class="hint">Accountant, financial advisor, doctor, etc.</p>
    {#each local.professionals as contact, i}
      <ItemCard title={contact.name || 'New Professional'} on:delete={() => removeContact('professionals', i)}>
        <FormField label="Name" value={contact.name} on:change={(e) => updateContact('professionals', i, 'name', e.detail.value)} />
        <FormField label="Role/Specialty" value={contact.relationship} on:change={(e) => updateContact('professionals', i, 'relationship', e.detail.value)} />
        <FormField label="Phone" value={contact.phone} on:change={(e) => updateContact('professionals', i, 'phone', e.detail.value)} />
        <FormField label="Email" value={contact.email} on:change={(e) => updateContact('professionals', i, 'email', e.detail.value)} />
        <FormField label="Notes" type="textarea" value={contact.notes} on:change={(e) => updateContact('professionals', i, 'notes', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Professional Contact" on:click={() => addContact('professionals')} />
  </div>

  <NotesField value={local.notes} on:change={updateNotes} />

  <CustomSubsections parentId="contacts" />
</div>

<style>
  .section { max-width: 800px; }
  .subsection { margin-bottom: 32px; }
  .subsection.emergency { background: var(--warning-bg); padding: 20px; border-radius: 8px; }
  h3 { margin: 0 0 8px 0; color: var(--text-primary); font-size: 1.1rem; }
  .hint { color: var(--text-secondary); font-size: 0.9rem; margin-bottom: 16px; }
</style>
