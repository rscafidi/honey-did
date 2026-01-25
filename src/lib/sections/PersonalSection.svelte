<script lang="ts">
  import { document } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';

  $: personal = $document?.personal ?? {
    funeral_preferences: '',
    obituary_notes: '',
    messages: [],
    notes: ''
  };

  function updateField(field: string, value: string) {
    document.updateSection('personal', { ...personal, [field]: value });
  }

  function addMessage() {
    const updated = { ...personal, messages: [...personal.messages, { recipient: '', message: '' }] };
    document.updateSection('personal', updated);
  }

  function removeMessage(index: number) {
    const updated = { ...personal, messages: personal.messages.filter((_: any, i: number) => i !== index) };
    document.updateSection('personal', updated);
  }

  function updateMessage(index: number, field: string, value: string) {
    const messages = [...personal.messages];
    messages[index] = { ...messages[index], [field]: value };
    document.updateSection('personal', { ...personal, messages });
  }

  function updateNotes(e: Event) {
    document.updateSection('personal', { ...personal, notes: (e.target as HTMLTextAreaElement).value });
  }
</script>

<div class="section">
  <p class="intro">This section is for your personal wishes and messages to loved ones. Take your time with this one.</p>

  <div class="subsection">
    <h3>Funeral Preferences</h3>
    <FormField
      label=""
      type="textarea"
      value={personal.funeral_preferences}
      placeholder="Burial vs cremation, service preferences, music, readings, any specific wishes..."
      on:change={(e) => updateField('funeral_preferences', e.detail.value)}
    />
  </div>

  <div class="subsection">
    <h3>Obituary Notes</h3>
    <FormField
      label=""
      type="textarea"
      value={personal.obituary_notes}
      placeholder="Key life events, achievements, family members to mention, tone you'd prefer..."
      on:change={(e) => updateField('obituary_notes', e.detail.value)}
    />
  </div>

  <div class="subsection">
    <h3>Personal Messages</h3>
    <p class="hint">Leave messages for specific people. These will be included in the document they receive.</p>
    {#each personal.messages as msg, i}
      <ItemCard title={msg.recipient || 'New Message'} on:delete={() => removeMessage(i)}>
        <FormField label="To" value={msg.recipient} placeholder="Name of recipient" on:change={(e) => updateMessage(i, 'recipient', e.detail.value)} />
        <FormField label="Message" type="textarea" value={msg.message} placeholder="Your message to them..." on:change={(e) => updateMessage(i, 'message', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Personal Message" on:click={addMessage} />
  </div>

  <NotesField value={personal.notes} on:change={updateNotes} />
</div>

<style>
  .section { max-width: 800px; }
  .intro { color: #666; margin-bottom: 24px; font-style: italic; }
  .subsection { margin-bottom: 32px; }
  h3 { margin: 0 0 12px 0; color: #333; font-size: 1.1rem; }
  .hint { color: #666; font-size: 0.9rem; margin-bottom: 16px; }
</style>
