<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { FormElement, FormElementField, CustomItem, FieldType } from '../stores/document';
  import { getFieldElements } from '../stores/document';

  export let formElements: FormElement[] = [];
  export let items: CustomItem[] = [];

  const dispatch = createEventDispatcher<{
    updateItems: CustomItem[];
  }>();

  let editingItemId: string | null = null;
  let editingItemValues: Record<string, string> = {};

  $: fieldElements = getFieldElements(formElements);
  $: hasFields = fieldElements.length > 0;

  function generateId(): string {
    return Math.random().toString(36).substring(2, 9);
  }

  function dispatchItems(updated: CustomItem[]) {
    dispatch('updateItems', updated);
  }

  export function addItem() {
    if (!hasFields) return;
    const newItem: CustomItem = {
      id: generateId(),
      values: {},
    };
    fieldElements.forEach((f) => {
      newItem.values[f.id] = f.field_type === 'boolean' ? 'false' : '';
    });
    editingItemId = newItem.id;
    editingItemValues = { ...newItem.values };
    dispatchItems([...items, newItem]);
  }

  function startEditItem(item: CustomItem) {
    editingItemId = item.id;
    editingItemValues = { ...item.values };
  }

  function saveItem() {
    if (!editingItemId) return;
    dispatchItems(
      items.map((item) =>
        item.id === editingItemId ? { ...item, values: editingItemValues } : item
      )
    );
    editingItemId = null;
    editingItemValues = {};
  }

  function cancelEditItem() {
    editingItemId = null;
    editingItemValues = {};
  }

  function deleteItem(itemId: string) {
    if (!confirm('Delete this item?')) return;
    dispatchItems(items.filter((item) => item.id !== itemId));
    if (editingItemId === itemId) {
      editingItemId = null;
      editingItemValues = {};
    }
  }

  function renderFieldInput(field: FormElementField): FieldType {
    return field.field_type;
  }
</script>

<div class="form-preview">
  {#if items.length === 0}
    <p class="empty-message">{hasFields ? 'No items yet. Click "+ Add Item" to add your first.' : 'Add fields in the form builder, then add items.'}</p>
  {:else}
    {#each items as item (item.id)}
      <div class="item-card">
        {#if editingItemId === item.id}
          <!-- Editing Form -->
          <div class="item-form">
            {#each formElements as el (el.id)}
              {#if el.type === 'field'}
                <div class="form-field">
                  <label for="field-{el.id}">{el.name || 'Unnamed Field'}</label>
                  {#if el.field_type === 'boolean'}
                    <label class="checkbox-label">
                      <input
                        type="checkbox"
                        checked={editingItemValues[el.id] === 'true'}
                        on:change={(e) => (editingItemValues[el.id] = e.currentTarget.checked ? 'true' : 'false')}
                      />
                      <span>Yes</span>
                    </label>
                  {:else if el.field_type === 'date'}
                    <input type="date" id="field-{el.id}" bind:value={editingItemValues[el.id]} />
                  {:else if el.field_type === 'number'}
                    <input type="number" id="field-{el.id}" bind:value={editingItemValues[el.id]} />
                  {:else}
                    <input type="text" id="field-{el.id}" bind:value={editingItemValues[el.id]} />
                  {/if}
                </div>
              {:else if el.type === 'divider'}
                <hr class="preview-divider" />
              {:else if el.type === 'header'}
                <h4 class="preview-header">{el.text || 'Untitled Header'}</h4>
              {/if}
            {/each}
            <div class="form-field">
              <label for="notes-{item.id}">Notes</label>
              <textarea id="notes-{item.id}" bind:value={editingItemValues['_notes']} rows="2"></textarea>
            </div>
            <div class="form-actions">
              <button class="btn btn-secondary" on:click={cancelEditItem}>Cancel</button>
              <button class="btn btn-danger" on:click={() => deleteItem(item.id)}>Delete</button>
              <button class="btn btn-primary" on:click={saveItem}>Save</button>
            </div>
          </div>
        {:else}
          <!-- Display Card -->
          <div class="item-display">
            <div class="item-content">
              {#each formElements as el (el.id)}
                {#if el.type === 'field'}
                  {#if item.values[el.id]}
                    <div class="item-detail">
                      <span class="detail-label">{el.name}:</span>
                      <span class="detail-value">
                        {#if el.field_type === 'boolean'}
                          {item.values[el.id] === 'true' ? 'Yes' : 'No'}
                        {:else}
                          {item.values[el.id]}
                        {/if}
                      </span>
                    </div>
                  {/if}
                {:else if el.type === 'divider'}
                  <hr class="preview-divider" />
                {:else if el.type === 'header'}
                  <h4 class="preview-header">{el.text}</h4>
                {/if}
              {/each}
              {#if item.values['_notes']}
                <div class="item-notes">{item.values['_notes']}</div>
              {/if}
              <div class="item-actions">
                <button class="btn btn-edit" on:click={() => startEditItem(item)}>Edit</button>
              </div>
            </div>
          </div>
        {/if}
      </div>
    {/each}
  {/if}
</div>

<style>
  .form-preview {
    display: flex;
    flex-direction: column;
    gap: 12px;
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
    overflow: hidden;
  }

  .item-content {
    padding: 12px 14px;
  }

  .item-actions {
    display: flex;
    justify-content: flex-end;
    margin-top: 8px;
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

  .preview-divider {
    border: none;
    border-top: 1px solid var(--border-color);
    margin: 10px 0;
  }

  .preview-header {
    margin: 10px 0 4px;
    font-size: 0.95rem;
    color: var(--text-primary);
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

  .btn {
    padding: 6px 12px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.85rem;
    font-weight: 500;
  }

  .btn-edit {
    background: var(--bg-secondary);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
  }

  .btn-edit:hover {
    background: var(--border-color);
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

  @media (max-width: 768px) {
    .form-actions {
      flex-direction: column;
    }

    .form-actions .btn-primary,
    .form-actions .btn-secondary,
    .form-actions .btn-danger {
      width: 100%;
      text-align: center;
    }
  }
</style>
