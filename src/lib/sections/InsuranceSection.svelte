<script lang="ts">
  import { onDestroy } from 'svelte';
  import { document, insuranceStore } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';
  import CustomSubsections from '../components/CustomSubsections.svelte';

  const defaultInsurance = {
    policies: [] as any[],
    notes: ''
  };

  // Local-first state: edits stay here, only flushed to store on discrete actions or debounced
  let local = { ...defaultInsurance };
  let hasPendingChanges = false;
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  // Sync from store ONLY when we don't have pending local changes
  const unsub = insuranceStore.subscribe((value) => {
    if (!hasPendingChanges) {
      local = value ?? defaultInsurance;
    }
  });
  onDestroy(() => {
    // Flush any pending changes before unmount
    if (hasPendingChanges) {
      if (debounceTimer) clearTimeout(debounceTimer);
      document.updateSection('insurance', local);
    }
    unsub();
  });

  function scheduleFlush() {
    hasPendingChanges = true;
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      document.updateSection('insurance', local);
      setTimeout(() => { hasPendingChanges = false; }, 0);
      debounceTimer = null;
    }, 300);
  }

  function addPolicy() {
    local = { ...local, policies: [...local.policies, { policy_type: '', provider: '', policy_number: '', contact: '', notes: '' }] };
    scheduleFlush();
  }

  function removePolicy(index: number) {
    local = { ...local, policies: local.policies.filter((_: any, i: number) => i !== index) };
    scheduleFlush();
  }

  function updatePolicy(index: number, field: string, value: string) {
    const policies = [...local.policies];
    policies[index] = { ...policies[index], [field]: value };
    local = { ...local, policies };
    scheduleFlush();
  }

  function updateNotes(e: Event) {
    const target = e.target as HTMLTextAreaElement;
    local = { ...local, notes: target.value };
    scheduleFlush();
  }
</script>

<div class="section">
  <p class="intro">Document all insurance policies so your family knows what coverage exists and how to file claims.</p>

  {#each local.policies as policy, i}
    <ItemCard title={policy.policy_type || 'New Policy'} on:delete={() => removePolicy(i)}>
      <FormField label="Policy Type" value={policy.policy_type} placeholder="Life, Health, Auto, Home, etc." on:change={(e) => updatePolicy(i, 'policy_type', e.detail.value)} />
      <FormField label="Provider/Company" value={policy.provider} on:change={(e) => updatePolicy(i, 'provider', e.detail.value)} />
      <FormField label="Policy Number" value={policy.policy_number} on:change={(e) => updatePolicy(i, 'policy_number', e.detail.value)} />
      <FormField label="Contact (Phone/Agent)" value={policy.contact} on:change={(e) => updatePolicy(i, 'contact', e.detail.value)} />
      <FormField label="Notes" type="textarea" value={policy.notes} placeholder="Coverage details, beneficiaries, etc." on:change={(e) => updatePolicy(i, 'notes', e.detail.value)} />
    </ItemCard>
  {/each}

  <AddButton label="Add Insurance Policy" on:click={addPolicy} />
  <NotesField value={local.notes} on:change={updateNotes} />

  <CustomSubsections parentId="insurance" />
</div>

<style>
  .section { max-width: 800px; }
  .intro { color: var(--text-secondary); margin-bottom: 20px; }

  @media (max-width: 768px) {
    .section {
      max-width: 100%;
    }
  }
</style>
