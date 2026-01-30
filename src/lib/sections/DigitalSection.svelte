<script lang="ts">
  import { onDestroy } from 'svelte';
  import { document, digitalStore } from '../stores/document';
  import ItemCard from '../components/ItemCard.svelte';
  import AddButton from '../components/AddButton.svelte';
  import FormField from '../components/FormField.svelte';
  import NotesField from '../components/NotesField.svelte';
  import CustomSubsections from '../components/CustomSubsections.svelte';

  const emptyAccount = { name: '', username: '', recovery_hint: '', notes: '' };

  const defaultDigital = {
    email_accounts: [] as any[],
    social_media: [] as any[],
    password_manager: { name: '', master_password_hint: '', recovery_method: '', notes: '' },
    notes: ''
  };

  // Local-first state: edits stay here, only flushed to store on discrete actions or debounced
  let local = { ...defaultDigital };
  let hasPendingChanges = false;
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  // Sync from store ONLY when we don't have pending local changes
  const unsub = digitalStore.subscribe((value) => {
    if (!hasPendingChanges) {
      local = value ?? defaultDigital;
    }
  });
  onDestroy(() => {
    // Flush any pending changes before unmount
    if (hasPendingChanges) {
      if (debounceTimer) clearTimeout(debounceTimer);
      document.updateSection('digital', local);
    }
    unsub();
  });

  function scheduleFlush() {
    hasPendingChanges = true;
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      document.updateSection('digital', local);
      setTimeout(() => { hasPendingChanges = false; }, 0);
      debounceTimer = null;
    }, 300);
  }

  function addEmail() {
    local = {
      ...local,
      email_accounts: [...local.email_accounts, { ...emptyAccount }]
    };
    scheduleFlush();
  }

  function removeEmail(index: number) {
    local = {
      ...local,
      email_accounts: local.email_accounts.filter((_: any, i: number) => i !== index)
    };
    scheduleFlush();
  }

  function updateEmail(index: number, field: string, value: string) {
    const accounts = [...local.email_accounts];
    accounts[index] = { ...accounts[index], [field]: value };
    local = { ...local, email_accounts: accounts };
    scheduleFlush();
  }

  function addSocial() {
    local = {
      ...local,
      social_media: [...local.social_media, { ...emptyAccount }]
    };
    scheduleFlush();
  }

  function removeSocial(index: number) {
    local = {
      ...local,
      social_media: local.social_media.filter((_: any, i: number) => i !== index)
    };
    scheduleFlush();
  }

  function updateSocial(index: number, field: string, value: string) {
    const accounts = [...local.social_media];
    accounts[index] = { ...accounts[index], [field]: value };
    local = { ...local, social_media: accounts };
    scheduleFlush();
  }

  function updatePasswordManager(field: string, value: string) {
    local = { ...local, password_manager: { ...local.password_manager, [field]: value } };
    scheduleFlush();
  }

  function updateNotes(e: Event) {
    const target = e.target as HTMLTextAreaElement;
    local = { ...local, notes: target.value };
    scheduleFlush();
  }
</script>

<div class="section">
  <div class="subsection highlight">
    <h3>Password Manager (Important!)</h3>
    <p class="hint">If you use a password manager, this is the most important section. Access to this unlocks everything else.</p>
    <div class="pw-card">
      <FormField label="Password Manager" value={local.password_manager?.name || ''} placeholder="1Password, LastPass, Bitwarden, etc." on:change={(e) => updatePasswordManager('name', e.detail.value)} />
      <FormField label="Master Password Hint" value={local.password_manager?.master_password_hint || ''} placeholder="A hint only your family would understand" on:change={(e) => updatePasswordManager('master_password_hint', e.detail.value)} />
      <FormField label="Recovery Method" value={local.password_manager?.recovery_method || ''} placeholder="Emergency kit location, recovery key, etc." on:change={(e) => updatePasswordManager('recovery_method', e.detail.value)} />
      <FormField label="Notes" type="textarea" value={local.password_manager?.notes || ''} on:change={(e) => updatePasswordManager('notes', e.detail.value)} />
    </div>
  </div>

  <div class="subsection">
    <h3>Email Accounts</h3>
    {#each local.email_accounts as account, i}
      <ItemCard title={account.name || 'New Email'} on:delete={() => removeEmail(i)}>
        <FormField label="Service" value={account.name} placeholder="Gmail, Outlook, etc." on:change={(e) => updateEmail(i, 'name', e.detail.value)} />
        <FormField label="Email/Username" value={account.username} on:change={(e) => updateEmail(i, 'username', e.detail.value)} />
        <FormField label="Recovery Hint" value={account.recovery_hint} placeholder="Recovery phone, backup email, etc." on:change={(e) => updateEmail(i, 'recovery_hint', e.detail.value)} />
        <FormField label="Notes" type="textarea" value={account.notes} on:change={(e) => updateEmail(i, 'notes', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Email Account" on:click={addEmail} />
  </div>

  <div class="subsection">
    <h3>Social Media</h3>
    {#each local.social_media as account, i}
      <ItemCard title={account.name || 'New Account'} on:delete={() => removeSocial(i)}>
        <FormField label="Service" value={account.name} placeholder="Facebook, Twitter, LinkedIn, etc." on:change={(e) => updateSocial(i, 'name', e.detail.value)} />
        <FormField label="Username" value={account.username} on:change={(e) => updateSocial(i, 'username', e.detail.value)} />
        <FormField label="Recovery Hint" value={account.recovery_hint} on:change={(e) => updateSocial(i, 'recovery_hint', e.detail.value)} />
        <FormField label="Notes" type="textarea" value={account.notes} placeholder="Memorial settings, legacy contact, etc." on:change={(e) => updateSocial(i, 'notes', e.detail.value)} />
      </ItemCard>
    {/each}
    <AddButton label="Add Social Media Account" on:click={addSocial} />
  </div>

  <NotesField value={local.notes} on:change={updateNotes} />

  <CustomSubsections parentId="digital" />
</div>

<style>
  .section { max-width: 800px; }
  .subsection { margin-bottom: 32px; }
  .subsection.highlight { background: var(--accent-light); padding: 20px; border-radius: 8px; margin-bottom: 32px; }
  h3 { margin: 0 0 16px 0; color: var(--text-primary); font-size: 1.1rem; }
  .hint { color: var(--accent-primary); font-size: 0.9rem; margin-bottom: 16px; }
  .pw-card { background: var(--bg-secondary); padding: 16px; border-radius: 8px; }

  @media (max-width: 768px) {
    .section {
      max-width: 100%;
    }
  }
</style>
