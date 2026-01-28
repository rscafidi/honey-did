<script lang="ts">
  import { document } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';
  import CustomSubsections from '../components/CustomSubsections.svelte';

  const emptyContact = { name: '', relationship: '', phone: '', email: '', notes: '' };

  $: legal = $document?.legal ?? {
    will_location: '',
    attorney: { ...emptyContact },
    power_of_attorney: '',
    trusts: [],
    notes: ''
  };

  function updateField(field: string, value: string) {
    document.updateSection('legal', { ...legal, [field]: value });
  }

  function updateAttorney(field: string, value: string) {
    document.updateSection('legal', { ...legal, attorney: { ...legal.attorney, [field]: value } });
  }

  function addTrust() {
    const updated = { ...legal, trusts: [...legal.trusts, { name: '', trustee: '', notes: '' }] };
    document.updateSection('legal', updated);
  }

  function removeTrust(index: number) {
    const updated = { ...legal, trusts: legal.trusts.filter((_: any, i: number) => i !== index) };
    document.updateSection('legal', updated);
  }

  function updateTrust(index: number, field: string, value: string) {
    const trusts = [...legal.trusts];
    trusts[index] = { ...trusts[index], [field]: value };
    document.updateSection('legal', { ...legal, trusts });
  }

  function updateNotes(e: Event) {
    document.updateSection('legal', { ...legal, notes: (e.target as HTMLTextAreaElement).value });
  }
</script>

<div class="section">
  <div class="subsection">
    <h3>Will & Estate Documents</h3>
    <FormField label="Will Location" value={legal.will_location} placeholder="Where is your will stored?" on:change={(e) => updateField('will_location', e.detail.value)} />
    <FormField label="Power of Attorney" value={legal.power_of_attorney} placeholder="Who has power of attorney?" on:change={(e) => updateField('power_of_attorney', e.detail.value)} />
  </div>

  <div class="subsection">
    <h3>Attorney</h3>
    <div class="attorney-card">
      <FormField label="Name" value={legal.attorney?.name || ''} on:change={(e) => updateAttorney('name', e.detail.value)} />
      <FormField label="Firm/Relationship" value={legal.attorney?.relationship || ''} on:change={(e) => updateAttorney('relationship', e.detail.value)} />
      <FormField label="Phone" value={legal.attorney?.phone || ''} on:change={(e) => updateAttorney('phone', e.detail.value)} />
      <FormField label="Email" value={legal.attorney?.email || ''} on:change={(e) => updateAttorney('email', e.detail.value)} />
      <FormField label="Notes" type="textarea" value={legal.attorney?.notes || ''} on:change={(e) => updateAttorney('notes', e.detail.value)} />
    </div>
  </div>

  <div class="subsection">
    <h3>Trusts</h3>
    {#each legal.trusts as trust, i}
      <ItemCard title={trust.name || 'New Trust'} on:delete={() => removeTrust(i)}>
        <FormField label="Trust Name" value={trust.name} on:change={(e) => updateTrust(i, 'name', e.detail.value)} />
        <FormField label="Trustee" value={trust.trustee} on:change={(e) => updateTrust(i, 'trustee', e.detail.value)} />
        <FormField label="Notes" type="textarea" value={trust.notes} on:change={(e) => updateTrust(i, 'notes', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Trust" on:click={addTrust} />
  </div>

  <NotesField value={legal.notes} on:change={updateNotes} />

  <CustomSubsections parentId="legal" />
</div>

<style>
  .section { max-width: 800px; }
  .subsection { margin-bottom: 32px; }
  h3 { margin: 0 0 16px 0; color: var(--text-primary); font-size: 1.1rem; }
  .attorney-card { background: var(--bg-tertiary); padding: 16px; border-radius: 8px; }
</style>
