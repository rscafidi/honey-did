<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { CustomSubsection, FormElement, CustomItem } from '../stores/document';
  import { getFieldElements } from '../stores/document';
  import FormBuilder from './FormBuilder.svelte';
  import FormPreview from './FormPreview.svelte';

  export let subsection: CustomSubsection;
  export let sectionName: string = '';

  const dispatch = createEventDispatcher<{
    update: CustomSubsection;
    delete: void;
    rename: string;
  }>();

  let local: CustomSubsection = subsection;
  $: local = subsection;

  $: fieldElements = getFieldElements(local.form_elements);
  $: hasFields = fieldElements.length > 0;

  let isRenamingSection = false;
  let newSectionName = sectionName;
  let showFieldEditor = false;

  function dispatchUpdate(updated: CustomSubsection) {
    local = updated;
    dispatch('update', updated);
  }

  function handleFormElementsUpdate(e: CustomEvent<FormElement[]>) {
    dispatchUpdate({
      ...local,
      form_elements: e.detail,
    });
  }

  function handleItemsUpdate(e: CustomEvent<CustomItem[]>) {
    dispatchUpdate({
      ...local,
      items: e.detail,
    });
  }

  function generateId(): string {
    return Math.random().toString(36).substring(2, 9);
  }

  function addItem() {
    if (!hasFields) return;
    const newItem: CustomItem = {
      id: generateId(),
      values: {},
    };
    fieldElements.forEach((f) => {
      newItem.values[f.id] = f.field_type === 'boolean' ? 'false' : '';
    });
    dispatchUpdate({
      ...local,
      items: [...local.items, newItem],
    });
  }

  function handleRename() {
    if (newSectionName.trim()) {
      dispatch('rename', newSectionName.trim());
    }
    isRenamingSection = false;
  }

  function handleDeleteSection() {
    const itemCount = local.items.length;
    const message = itemCount > 0
      ? `Delete "${sectionName}" and all ${itemCount} item${itemCount === 1 ? '' : 's'} within it?`
      : `Delete "${sectionName}"?`;
    if (confirm(message)) {
      dispatch('delete');
    }
  }
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
      <button class="btn btn-add" on:click={addItem} disabled={!hasFields} title={hasFields ? 'Add a new item' : 'Add fields first'}>+ Add Item</button>
      <button class="btn btn-gear" class:active={showFieldEditor} on:click={() => (showFieldEditor = !showFieldEditor)} title="Configure fields">&#9881;</button>
      <button class="btn btn-delete" on:click={handleDeleteSection}>Delete</button>
    </div>
  </div>

  {#if showFieldEditor}
    <div class="field-editor-panel">
      <div class="panel-header">
        <span class="panel-label">Configure Fields</span>
        <button class="btn btn-done" on:click={() => (showFieldEditor = false)}>Done</button>
      </div>
      <FormBuilder elements={local.form_elements} on:update={handleFormElementsUpdate} />
    </div>
  {/if}

  <div class="items-area">
    {#if !hasFields && !showFieldEditor}
      <div class="setup-prompt">
        <p>Define the fields you want to track for each item.</p>
        <button class="btn btn-setup" on:click={() => (showFieldEditor = true)}>Set Up Fields</button>
      </div>
    {:else}
      <FormPreview
        formElements={local.form_elements}
        items={local.items}
        on:updateItems={handleItemsUpdate}
      />
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

  .btn-add:hover:not(:disabled) {
    background: var(--accent-hover);
  }

  .btn-add:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .btn-gear {
    background: none;
    color: var(--text-secondary);
    border: 1px solid var(--border-color);
    font-size: 1rem;
    padding: 4px 8px;
    line-height: 1;
  }

  .btn-gear:hover {
    background: var(--bg-secondary);
    color: var(--text-primary);
  }

  .btn-gear.active {
    background: var(--accent-light);
    color: var(--accent-primary);
    border-color: var(--accent-primary);
  }

  .btn-delete {
    background: none;
    color: var(--error-color);
    border: 1px solid var(--error-color);
  }

  .btn-delete:hover {
    background: rgba(155, 44, 44, 0.1);
  }

  /* Field editor panel */
  .field-editor-panel {
    padding: 16px;
    background: var(--bg-tertiary);
    border-bottom: 1px solid var(--border-color);
  }

  .panel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }

  .panel-label {
    font-size: 0.8rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-secondary);
  }

  .btn-done {
    background: var(--accent-primary);
    color: var(--bg-secondary);
    padding: 4px 12px;
    font-size: 0.8rem;
  }

  .btn-done:hover {
    opacity: 0.9;
  }

  /* Items area */
  .items-area {
    padding: 16px;
  }

  .setup-prompt {
    text-align: center;
    padding: 24px 16px;
  }

  .setup-prompt p {
    color: var(--text-secondary);
    font-style: italic;
    margin: 0 0 12px;
    font-size: 0.9rem;
  }

  .btn-setup {
    background: var(--accent-light);
    color: var(--accent-primary);
    padding: 8px 16px;
    font-size: 0.9rem;
  }

  .btn-setup:hover {
    background: var(--accent-hover);
  }
</style>
