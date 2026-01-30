<script lang="ts">
  import { onDestroy } from 'svelte';
  import { document, personalStore } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';
  import CustomSubsections from '../components/CustomSubsections.svelte';

  const defaultPersonal = {
    funeral_preferences: '',
    obituary_notes: '',
    messages: [] as any[],
    notes: ''
  };

  // Local-first state: edits stay here, only flushed to store on discrete actions or debounced
  let local = { ...defaultPersonal };
  let hasPendingChanges = false;
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  // Sync from store ONLY when we don't have pending local changes
  const unsub = personalStore.subscribe((value) => {
    if (!hasPendingChanges) {
      local = value ?? defaultPersonal;
    }
  });
  onDestroy(() => {
    // Flush any pending changes before unmount
    if (hasPendingChanges) {
      if (debounceTimer) clearTimeout(debounceTimer);
      document.updateSection('personal', local);
    }
    unsub();
  });

  function scheduleFlush() {
    hasPendingChanges = true;
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      document.updateSection('personal', local);
      setTimeout(() => { hasPendingChanges = false; }, 0);
      debounceTimer = null;
    }, 300);
  }

  function updateField(field: string, value: string) {
    local = { ...local, [field]: value };
    scheduleFlush();
  }

  function addMessage() {
    local = { ...local, messages: [...local.messages, { recipient: '', message: '' }] };
    scheduleFlush();
  }

  function removeMessage(index: number) {
    local = { ...local, messages: local.messages.filter((_: any, i: number) => i !== index) };
    scheduleFlush();
  }

  function updateMessage(index: number, field: string, value: string) {
    const messages = [...local.messages];
    messages[index] = { ...messages[index], [field]: value };
    local = { ...local, messages };
    scheduleFlush();
  }

  function updateNotes(e: Event) {
    local = { ...local, notes: (e.target as HTMLTextAreaElement).value };
    scheduleFlush();
  }
</script>

<div class="section">
  <p class="intro">This section is for your personal wishes and messages to loved ones. Take your time with this one.</p>

  <div class="subsection">
    <h3>Funeral Preferences</h3>
    <FormField
      label=""
      type="textarea"
      value={local.funeral_preferences}
      placeholder="Burial vs cremation, service preferences, music, readings, any specific wishes..."
      on:change={(e) => updateField('funeral_preferences', e.detail.value)}
    />
  </div>

  <div class="subsection">
    <h3>Obituary Notes</h3>
    <FormField
      label=""
      type="textarea"
      value={local.obituary_notes}
      placeholder="Key life events, achievements, family members to mention, tone you'd prefer..."
      on:change={(e) => updateField('obituary_notes', e.detail.value)}
    />
  </div>

  <div class="subsection">
    <h3>Personal Messages</h3>
    <p class="hint">Leave messages for specific people. These will be included in the document they receive.</p>
    {#each local.messages as msg, i}
      <ItemCard title={msg.recipient || 'New Message'} on:delete={() => removeMessage(i)}>
        <FormField label="To" value={msg.recipient} placeholder="Name of recipient" on:change={(e) => updateMessage(i, 'recipient', e.detail.value)} />
        <FormField label="Message" type="textarea" value={msg.message} placeholder="Your message to them..." on:change={(e) => updateMessage(i, 'message', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Personal Message" on:click={addMessage} />
  </div>

  <NotesField value={local.notes} on:change={updateNotes} />

  <CustomSubsections parentId="personal" />
</div>

<style>
  .section { max-width: 800px; }
  .intro { color: var(--text-secondary); margin-bottom: 24px; font-style: italic; }
  .subsection { margin-bottom: 32px; }
  h3 { margin: 0 0 12px 0; color: var(--text-primary); font-size: 1.1rem; }
  .hint { color: var(--text-secondary); font-size: 0.9rem; margin-bottom: 16px; }

  @media (max-width: 768px) {
    .section {
      max-width: 100%;
    }
  }
</style>
