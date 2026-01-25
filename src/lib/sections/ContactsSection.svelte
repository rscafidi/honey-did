<script lang="ts">
  import { document } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';

  const emptyContact = { name: '', relationship: '', phone: '', email: '', notes: '' };

  $: contacts = $document?.contacts ?? {
    emergency_contacts: [],
    family: [],
    professionals: [],
    notes: ''
  };

  function addContact(list: 'emergency_contacts' | 'family' | 'professionals') {
    const updated = { ...contacts, [list]: [...contacts[list], { ...emptyContact }] };
    document.updateSection('contacts', updated);
  }

  function removeContact(list: 'emergency_contacts' | 'family' | 'professionals', index: number) {
    const updated = { ...contacts, [list]: contacts[list].filter((_: any, i: number) => i !== index) };
    document.updateSection('contacts', updated);
  }

  function updateContact(list: 'emergency_contacts' | 'family' | 'professionals', index: number, field: string, value: string) {
    const items = [...contacts[list]];
    items[index] = { ...items[index], [field]: value };
    document.updateSection('contacts', { ...contacts, [list]: items });
  }

  function updateNotes(e: Event) {
    document.updateSection('contacts', { ...contacts, notes: (e.target as HTMLTextAreaElement).value });
  }
</script>

<div class="section">
  <div class="subsection emergency">
    <h3>Emergency Contacts</h3>
    <p class="hint">Who should be called first in an emergency?</p>
    {#each contacts.emergency_contacts as contact, i}
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
    {#each contacts.family as contact, i}
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
    {#each contacts.professionals as contact, i}
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

  <NotesField value={contacts.notes} on:change={updateNotes} />
</div>

<style>
  .section { max-width: 800px; }
  .subsection { margin-bottom: 32px; }
  .subsection.emergency { background: #fff3e0; padding: 20px; border-radius: 8px; }
  h3 { margin: 0 0 8px 0; color: #333; font-size: 1.1rem; }
  .hint { color: #666; font-size: 0.9rem; margin-bottom: 16px; }
</style>
