<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { dndzone } from 'svelte-dnd-action';
  import type { FormElement, FormElementField, FieldType } from '../stores/document';

  export let elements: FormElement[] = [];

  const dispatch = createEventDispatcher<{
    update: FormElement[];
  }>();

  function generateId(): string {
    return Math.random().toString(36).substring(2, 9);
  }

  function dispatchElements(updated: FormElement[]) {
    elements = updated;
    dispatch('update', updated);
  }

  function addField() {
    const el: FormElementField = {
      type: 'field',
      id: generateId(),
      name: '',
      field_type: 'text',
    };
    dispatchElements([...elements, el]);
  }

  function addDivider() {
    dispatchElements([...elements, { type: 'divider', id: generateId() }]);
  }

  function addHeader() {
    dispatchElements([...elements, { type: 'header', id: generateId(), text: '' }]);
  }

  function updateElement(id: string, updates: Record<string, unknown>) {
    dispatchElements(
      elements.map((el) => (el.id === id ? { ...el, ...updates } : el))
    );
  }

  function updateFieldName(id: string, value: string) {
    updateElement(id, { name: value });
  }

  function updateHeaderText(id: string, value: string) {
    updateElement(id, { text: value });
  }

  function deleteElement(id: string) {
    dispatchElements(elements.filter((el) => el.id !== id));
  }

  function handleFieldTypeChange(id: string, event: Event) {
    const target = event.target as HTMLSelectElement;
    updateElement(id, { field_type: target.value });
  }

  function handleDndConsider(e: CustomEvent) {
    elements = e.detail.items;
  }

  function handleDndFinalize(e: CustomEvent) {
    dispatchElements(e.detail.items);
  }

  const fieldTypes: { value: FieldType; label: string }[] = [
    { value: 'text', label: 'Text' },
    { value: 'number', label: 'Number' },
    { value: 'date', label: 'Date' },
    { value: 'boolean', label: 'Yes/No' },
  ];
</script>

<div class="form-builder">
  <div class="toolbar">
    <button class="btn btn-add-el" on:click={addField}>+ Field</button>
    <button class="btn btn-add-el" on:click={addDivider}>+ Divider</button>
    <button class="btn btn-add-el" on:click={addHeader}>+ Header</button>
  </div>

  {#if elements.length === 0}
    <p class="empty-hint">Add fields, dividers, or headers to build your form.</p>
  {:else}
    <div
      class="element-list"
      use:dndzone={{ items: elements, flipDurationMs: 200 }}
      on:consider={handleDndConsider}
      on:finalize={handleDndFinalize}
    >
      {#each elements as el (el.id)}
        <div class="element-row element-{el.type}">
          <span class="drag-handle" aria-label="Drag to reorder">⠿</span>

          {#if el.type === 'field'}
            <input
              type="text"
              class="el-name"
              value={el.name}
              placeholder="Field name"
              on:input={(e) => updateFieldName(el.id, e.currentTarget.value)}
            />
            <select
              class="el-field-type"
              value={el.field_type}
              on:change={(e) => handleFieldTypeChange(el.id, e)}
            >
              {#each fieldTypes as ft}
                <option value={ft.value}>{ft.label}</option>
              {/each}
            </select>
          {:else if el.type === 'divider'}
            <span class="divider-indicator"><hr /></span>
          {:else if el.type === 'header'}
            <input
              type="text"
              class="el-header-text"
              value={el.text}
              placeholder="Header text"
              on:input={(e) => updateHeaderText(el.id, e.currentTarget.value)}
            />
          {/if}

          <button class="btn-icon delete" on:click={() => deleteElement(el.id)} title="Delete element">&times;</button>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .form-builder {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .toolbar {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  .btn {
    padding: 6px 12px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.85rem;
    font-weight: 500;
  }

  .btn-add-el {
    background: var(--accent-light);
    color: var(--accent-primary);
  }

  .btn-add-el:hover {
    background: var(--accent-hover);
  }

  .empty-hint {
    color: var(--text-secondary);
    font-style: italic;
    font-size: 0.9rem;
    margin: 0;
  }

  .element-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .element-row {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 10px;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: 6px;
  }

  .drag-handle {
    cursor: grab;
    color: var(--text-secondary);
    font-size: 1.1rem;
    user-select: none;
    flex-shrink: 0;
  }

  .el-name {
    flex: 1;
    padding: 6px 10px;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    font-size: 0.9rem;
    background: var(--bg-secondary);
    color: var(--text-primary);
  }

  .el-name:focus {
    outline: none;
    border-color: var(--accent-primary);
  }

  .el-field-type {
    padding: 6px 10px;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    font-size: 0.9rem;
    background: var(--bg-secondary);
    color: var(--text-primary);
  }

  .divider-indicator {
    flex: 1;
    display: flex;
    align-items: center;
  }

  .divider-indicator hr {
    flex: 1;
    border: none;
    border-top: 2px dashed var(--border-color);
    margin: 0;
  }

  .el-header-text {
    flex: 1;
    padding: 6px 10px;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    font-size: 0.9rem;
    font-weight: 600;
    background: var(--bg-secondary);
    color: var(--text-primary);
  }

  .el-header-text:focus {
    outline: none;
    border-color: var(--accent-primary);
  }

  .btn-icon {
    background: none;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    width: 30px;
    height: 30px;
    cursor: pointer;
    font-size: 1.2rem;
    color: var(--text-secondary);
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .btn-icon.delete:hover {
    background: rgba(155, 44, 44, 0.1);
    border-color: var(--error-color);
    color: var(--error-color);
  }
</style>
