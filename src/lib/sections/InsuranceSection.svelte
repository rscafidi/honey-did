<script lang="ts">
  import { document } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';
  import CustomSubsections from '../components/CustomSubsections.svelte';

  $: insurance = $document?.insurance ?? { policies: [], notes: '' };

  function addPolicy() {
    const updated = {
      ...insurance,
      policies: [...insurance.policies, {
        policy_type: '',
        provider: '',
        policy_number: '',
        contact: '',
        notes: ''
      }]
    };
    document.updateSection('insurance', updated);
  }

  function removePolicy(index: number) {
    const updated = {
      ...insurance,
      policies: insurance.policies.filter((_: any, i: number) => i !== index)
    };
    document.updateSection('insurance', updated);
  }

  function updatePolicy(index: number, field: string, value: string) {
    const policies = [...insurance.policies];
    policies[index] = { ...policies[index], [field]: value };
    document.updateSection('insurance', { ...insurance, policies });
  }

  function updateNotes(e: Event) {
    const target = e.target as HTMLTextAreaElement;
    document.updateSection('insurance', { ...insurance, notes: target.value });
  }
</script>

<div class="section">
  <p class="intro">Document all insurance policies so your family knows what coverage exists and how to file claims.</p>

  {#each insurance.policies as policy, i}
    <ItemCard title={policy.policy_type || 'New Policy'} on:delete={() => removePolicy(i)}>
      <FormField label="Policy Type" value={policy.policy_type} placeholder="Life, Health, Auto, Home, etc." on:change={(e) => updatePolicy(i, 'policy_type', e.detail.value)} />
      <FormField label="Provider/Company" value={policy.provider} on:change={(e) => updatePolicy(i, 'provider', e.detail.value)} />
      <FormField label="Policy Number" value={policy.policy_number} on:change={(e) => updatePolicy(i, 'policy_number', e.detail.value)} />
      <FormField label="Contact (Phone/Agent)" value={policy.contact} on:change={(e) => updatePolicy(i, 'contact', e.detail.value)} />
      <FormField label="Notes" type="textarea" value={policy.notes} placeholder="Coverage details, beneficiaries, etc." on:change={(e) => updatePolicy(i, 'notes', e.detail.value)} />
    </ItemCard>
  {/each}

  <AddButton label="Add Insurance Policy" on:click={addPolicy} />
  <NotesField value={insurance.notes} on:change={updateNotes} />

  <CustomSubsections parentId="insurance" />
</div>

<style>
  .section { max-width: 800px; }
  .intro { color: #666; margin-bottom: 20px; }
</style>
