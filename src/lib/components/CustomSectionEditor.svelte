<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { CustomSubsection, FieldDefinition, CustomItem, FieldType } from '../stores/document';

  export let subsection: CustomSubsection;
  export let sectionName: string = '';

  const dispatch = createEventDispatcher<{
    update: CustomSubsection;
    delete: void;
    rename: string;
  }>();

  let showFieldEditor = subsection.items.length === 0;
  let editingItemId: string | null = null;
  let editingItemValues: Record<string, string> = {};
  let isRenamingSection = false;
  let newSectionName = sectionName;

  function generateId(): string {
    return Math.random().toString(36).substring(2, 9);
  }

  function addField() {
    const newField: FieldDefinition = {
      id: generateId(),
      name: '',
      field_type: 'text',
    };
    dispatch('update', {
      ...subsection,
      field_definitions: [...subsection.field_definitions, newField],
    });
  }

  function updateField(fieldId: string, updates: Partial<FieldDefinition>) {
    dispatch('update', {
      ...subsection,
      field_definitions: subsection.field_definitions.map((f) =>
        f.id === fieldId ? { ...f, ...updates } : f
      ),
    });
  }

  function handleFieldTypeChange(fieldId: string, event: Event) {
    const target = event.target as HTMLSelectElement;
    updateField(fieldId, { field_type: target.value as FieldType });
  }

  function deleteField(fieldId: string) {
    if (subsection.items.length > 0) {
      if (!confirm('This field has data in existing items. Delete anyway?')) {
        return;
      }
    }
    dispatch('update', {
      ...subsection,
      field_definitions: subsection.field_definitions.filter((f) => f.id !== fieldId),
      items: subsection.items.map((item) => {
        const newValues = { ...item.values };
        delete newValues[fieldId];
        return { ...item, values: newValues };
      }),
    });
  }

  function addItem() {
    const newItem: CustomItem = {
      id: generateId(),
      values: {},
    };
    // Initialize with empty values for all fields
    subsection.field_definitions.forEach((f) => {
      newItem.values[f.id] = f.field_type === 'boolean' ? 'false' : '';
    });
    editingItemId = newItem.id;
    editingItemValues = { ...newItem.values };
    dispatch('update', {
      ...subsection,
      items: [...subsection.items, newItem],
    });
  }

  function startEditItem(item: CustomItem) {
    editingItemId = item.id;
    editingItemValues = { ...item.values };
  }

  function saveItem() {
    if (!editingItemId) return;
    dispatch('update', {
      ...subsection,
      items: subsection.items.map((item) =>
        item.id === editingItemId ? { ...item, values: editingItemValues } : item
      ),
    });
    editingItemId = null;
    editingItemValues = {};
  }

  function cancelEditItem() {
    editingItemId = null;
    editingItemValues = {};
  }

  function deleteItem(itemId: string) {
    if (!confirm('Delete this item?')) return;
    dispatch('update', {
      ...subsection,
      items: subsection.items.filter((item) => item.id !== itemId),
    });
    if (editingItemId === itemId) {
      editingItemId = null;
      editingItemValues = {};
    }
  }

  function getItemTitle(item: CustomItem): string {
    if (subsection.field_definitions.length === 0) return 'Untitled';
    const firstField = subsection.field_definitions[0];
    return item.values[firstField.id] || 'Untitled';
  }

  function handleRename() {
    if (newSectionName.trim()) {
      dispatch('rename', newSectionName.trim());
    }
    isRenamingSection = false;
  }

  function handleDeleteSection() {
    const itemCount = subsection.items.length;
    const message = itemCount > 0
      ? `Delete "${sectionName}" and all ${itemCount} item${itemCount === 1 ? '' : 's'} within it?`
      : `Delete "${sectionName}"?`;
    if (confirm(message)) {
      dispatch('delete');
    }
  }

  const fieldTypes: { value: FieldType; label: string }[] = [
    { value: 'text', label: 'Text' },
    { value: 'number', label: 'Number' },
    { value: 'date', label: 'Date' },
    { value: 'boolean', label: 'Yes/No' },
  ];
</script>

<div class="custom-section">
  <div class="section-header">
    {#if isRenamingSection}
      <input
        type="text"
        class="rename-input"
        bind:value={newSectionName}
        on:blur={handleRename}
        on:keydown={(e) => e.key === 'Enter' && handleRename()}
      />
    {:else}
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <h3 class="section-title" on:click={() => { isRenamingSection = true; newSectionName = sectionName; }}>
        {sectionName}
      </h3>
    {/if}
    <div class="section-actions">
      <button class="btn btn-add" on:click={addItem}>+ Add Item</button>
      <button class="btn btn-delete" on:click={handleDeleteSection}>Delete</button>
    </div>
  </div>

  <!-- Field Definitions -->
  <div class="field-definitions">
    <button class="toggle-fields" on:click={() => (showFieldEditor = !showFieldEditor)}>
      <span class="toggle-icon">{showFieldEditor ? '▼' : '▶'}</span>
      Define Fields ({subsection.field_definitions.length} field{subsection.field_definitions.length === 1 ? '' : 's'})
    </button>

    {#if showFieldEditor}
      <div class="fields-panel">
        {#each subsection.field_definitions as field (field.id)}
          <div class="field-row">
            <input
              type="text"
              class="field-name"
              value={field.name}
              placeholder="Field name"
              on:input={(e) => updateField(field.id, { name: e.currentTarget.value })}
            />
            <select
              class="field-type"
              value={field.field_type}
              on:change={(e) => handleFieldTypeChange(field.id, e)}
            >
              {#each fieldTypes as ft}
                <option value={ft.value}>{ft.label}</option>
              {/each}
            </select>
            <button class="btn-icon delete" on:click={() => deleteField(field.id)} title="Delete field">×</button>
          </div>
        {/each}
        <button class="btn btn-add-field" on:click={addField}>+ Add Field</button>
      </div>
    {/if}
  </div>

  <!-- Items List -->
  <div class="items-list">
    {#if subsection.items.length === 0}
      <p class="empty-message">No items yet. Click "+ Add Item" to add your first.</p>
    {:else}
      {#each subsection.items as item (item.id)}
        <div class="item-card">
          {#if editingItemId === item.id}
            <!-- Editing Form -->
            <div class="item-form">
              {#each subsection.field_definitions as field (field.id)}
                <div class="form-field">
                  <label for="field-{field.id}">{field.name || 'Unnamed Field'}</label>
                  {#if field.field_type === 'boolean'}
                    <label class="checkbox-label">
                      <input
                        type="checkbox"
                        checked={editingItemValues[field.id] === 'true'}
                        on:change={(e) => (editingItemValues[field.id] = e.currentTarget.checked ? 'true' : 'false')}
                      />
                      <span>Yes</span>
                    </label>
                  {:else if field.field_type === 'date'}
                    <input
                      type="date"
                      id="field-{field.id}"
                      bind:value={editingItemValues[field.id]}
                    />
                  {:else if field.field_type === 'number'}
                    <input
                      type="number"
                      id="field-{field.id}"
                      bind:value={editingItemValues[field.id]}
                    />
                  {:else}
                    <input
                      type="text"
                      id="field-{field.id}"
                      bind:value={editingItemValues[field.id]}
                    />
                  {/if}
                </div>
              {/each}
              <div class="form-field">
                <label for="notes-{item.id}">Notes</label>
                <textarea
                  id="notes-{item.id}"
                  bind:value={editingItemValues['_notes']}
                  rows="2"
                ></textarea>
              </div>
              <div class="form-actions">
                <button class="btn btn-secondary" on:click={cancelEditItem}>Cancel</button>
                <button class="btn btn-danger" on:click={() => deleteItem(item.id)}>Delete</button>
                <button class="btn btn-primary" on:click={saveItem}>Save</button>
              </div>
            </div>
          {:else}
            <!-- Display Card -->
            <div class="item-header">
              <span class="item-title">{getItemTitle(item)}</span>
              <button class="btn btn-edit" on:click={() => startEditItem(item)}>Edit</button>
            </div>
            <div class="item-content">
              {#each subsection.field_definitions.slice(1) as field (field.id)}
                {#if item.values[field.id]}
                  <div class="item-detail">
                    <span class="detail-label">{field.name}:</span>
                    <span class="detail-value">
                      {#if field.field_type === 'boolean'}
                        {item.values[field.id] === 'true' ? 'Yes' : 'No'}
                      {:else}
                        {item.values[field.id]}
                      {/if}
                    </span>
                  </div>
                {/if}
              {/each}
              {#if item.values['_notes']}
                <div class="item-notes">{item.values['_notes']}</div>
              {/if}
            </div>
          {/if}
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .custom-section {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    margin-bottom: 24px;
    border-left: 4px solid var(--accent-secondary);
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px;
    border-bottom: 1px solid var(--border-color);
    background: var(--bg-tertiary);
    border-radius: 4px 8px 0 0;
  }

  .section-title {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary);
    cursor: pointer;
  }

  .section-title:hover {
    text-decoration: underline;
  }

  .rename-input {
    font-size: 1rem;
    font-weight: 600;
    padding: 4px 8px;
    border: 1px solid var(--accent-primary);
    border-radius: 4px;
    background: var(--bg-secondary);
    color: var(--text-primary);
  }

  .section-actions {
    display: flex;
    gap: 8px;
  }

  .btn {
    padding: 6px 12px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.85rem;
    font-weight: 500;
  }

  .btn-add {
    background: var(--accent-light);
    color: var(--accent-primary);
  }

  .btn-add:hover {
    background: var(--accent-hover);
  }

  .btn-delete {
    background: none;
    color: var(--error-color);
    border: 1px solid var(--error-color);
  }

  .btn-delete:hover {
    background: rgba(155, 44, 44, 0.1);
  }

  .field-definitions {
    border-bottom: 1px solid var(--border-color);
  }

  .toggle-fields {
    width: 100%;
    padding: 12px 16px;
    background: none;
    border: none;
    text-align: left;
    cursor: pointer;
    font-size: 0.9rem;
    color: var(--text-secondary);
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .toggle-fields:hover {
    background: var(--bg-tertiary);
  }

  .toggle-icon {
    font-size: 0.7rem;
  }

  .fields-panel {
    padding: 0 16px 16px;
  }

  .field-row {
    display: flex;
    gap: 8px;
    margin-bottom: 8px;
    align-items: center;
  }

  .field-name {
    flex: 1;
    padding: 8px 12px;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    font-size: 0.9rem;
    background: var(--bg-secondary);
    color: var(--text-primary);
  }

  .field-name:focus {
    outline: none;
    border-color: var(--accent-primary);
  }

  .field-type {
    padding: 8px 12px;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    font-size: 0.9rem;
    background: var(--bg-secondary);
    color: var(--text-primary);
  }

  .btn-icon {
    background: none;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    width: 32px;
    height: 32px;
    cursor: pointer;
    font-size: 1.2rem;
    color: var(--text-secondary);
  }

  .btn-icon.delete:hover {
    background: rgba(155, 44, 44, 0.1);
    border-color: var(--error-color);
    color: var(--error-color);
  }

  .btn-add-field {
    background: none;
    border: 1px dashed var(--border-color);
    padding: 8px 16px;
    color: var(--text-secondary);
    cursor: pointer;
    border-radius: 4px;
    font-size: 0.85rem;
  }

  .btn-add-field:hover {
    border-color: var(--accent-primary);
    color: var(--accent-primary);
  }

  .items-list {
    padding: 16px;
  }

  .empty-message {
    color: var(--text-secondary);
    font-style: italic;
    margin: 0;
  }

  .item-card {
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    margin-bottom: 12px;
    overflow: hidden;
  }

  .item-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
  }

  .item-title {
    font-weight: 600;
    color: var(--text-primary);
  }

  .btn-edit {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .btn-edit:hover {
    background: var(--border-color);
  }

  .item-content {
    padding: 12px 16px;
  }

  .item-detail {
    margin-bottom: 4px;
    font-size: 0.9rem;
  }

  .detail-label {
    color: var(--text-secondary);
  }

  .detail-value {
    color: var(--text-primary);
    margin-left: 4px;
  }

  .item-notes {
    margin-top: 8px;
    padding: 8px 12px;
    background: var(--bg-secondary);
    border-radius: 4px;
    font-style: italic;
    color: var(--text-secondary);
    font-size: 0.9rem;
  }

  .item-form {
    padding: 16px;
  }

  .form-field {
    margin-bottom: 12px;
  }

  .form-field label {
    display: block;
    font-size: 0.85rem;
    color: var(--text-secondary);
    margin-bottom: 4px;
  }

  .form-field input[type="text"],
  .form-field input[type="number"],
  .form-field input[type="date"],
  .form-field textarea {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    font-size: 0.95rem;
    box-sizing: border-box;
    background: var(--bg-secondary);
    color: var(--text-primary);
  }

  .form-field input:focus,
  .form-field textarea:focus {
    outline: none;
    border-color: var(--accent-primary);
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    color: var(--text-primary);
  }

  .checkbox-label input {
    width: 18px;
    height: 18px;
  }

  .form-actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
    margin-top: 16px;
  }

  .btn-secondary {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .btn-secondary:hover {
    background: var(--border-color);
  }

  .btn-danger {
    background: rgba(155, 44, 44, 0.1);
    color: var(--error-color);
  }

  .btn-danger:hover {
    opacity: 0.9;
  }

  .btn-primary {
    background: var(--accent-primary);
    color: var(--bg-secondary);
  }

  .btn-primary:hover {
    opacity: 0.9;
  }
</style>
