<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { CustomSection, CustomSubsection } from '../stores/document';
  import CustomSectionEditor from '../components/CustomSectionEditor.svelte';

  export let section: CustomSection;

  const dispatch = createEventDispatcher<{
    update: CustomSection;
    rename: string;
  }>();

  let showAddSubsectionForm = false;
  let newSubsectionName = '';

  function generateId(): string {
    return Math.random().toString(36).substring(2, 9);
  }

  function addSubsection() {
    if (!newSubsectionName.trim()) return;

    const newSubsection: CustomSubsection = {
      id: generateId(),
      name: newSubsectionName.trim(),
      field_definitions: [],
      items: [],
    };

    dispatch('update', {
      ...section,
      subsections: [...section.subsections, newSubsection],
    });

    newSubsectionName = '';
    showAddSubsectionForm = false;
  }

  function updateSubsection(subsectionId: string, updatedSubsection: CustomSubsection) {
    dispatch('update', {
      ...section,
      subsections: section.subsections.map((s) =>
        s.id === subsectionId ? updatedSubsection : s
      ),
    });
  }

  function deleteSubsection(subsectionId: string) {
    dispatch('update', {
      ...section,
      subsections: section.subsections.filter((s) => s.id !== subsectionId),
    });
  }

  function renameSubsection(subsectionId: string, newName: string) {
    dispatch('update', {
      ...section,
      subsections: section.subsections.map((s) =>
        s.id === subsectionId ? { ...s, name: newName } : s
      ),
    });
  }
</script>

<div class="custom-section-page">
  {#if section.subsections.length === 0 && !showAddSubsectionForm}
    <div class="empty-state">
      <p>This section is empty. Add a subsection to get started.</p>
      <button class="btn btn-primary" on:click={() => (showAddSubsectionForm = true)}>
        + Add Subsection
      </button>
    </div>
  {:else}
    {#each section.subsections as subsection (subsection.id)}
      <CustomSectionEditor
        {subsection}
        sectionName={subsection.name}
        on:update={(e) => updateSubsection(subsection.id, e.detail)}
        on:delete={() => deleteSubsection(subsection.id)}
        on:rename={(e) => renameSubsection(subsection.id, e.detail)}
      />
    {/each}

    {#if showAddSubsectionForm}
      <div class="add-subsection-form">
        <input
          type="text"
          class="add-subsection-input"
          placeholder="Subsection name"
          bind:value={newSubsectionName}
          on:keydown={(e) => e.key === 'Enter' && addSubsection()}
        />
        <div class="add-subsection-actions">
          <button class="btn btn-secondary" on:click={() => { showAddSubsectionForm = false; newSubsectionName = ''; }}>Cancel</button>
          <button class="btn btn-primary" on:click={addSubsection}>Create</button>
        </div>
      </div>
    {:else}
      <button class="add-subsection-btn" on:click={() => (showAddSubsectionForm = true)}>
        + Add Subsection
      </button>
    {/if}
  {/if}
</div>

<style>
  .custom-section-page {
    max-width: 800px;
  }

  .empty-state {
    text-align: center;
    padding: 60px 20px;
    background: white;
    border: 1px solid #D4D4D4;
    border-radius: 8px;
  }

  .empty-state p {
    color: #606060;
    margin-bottom: 20px;
  }

  .btn {
    padding: 10px 20px;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.9rem;
    font-weight: 500;
    transition: all 0.15s ease;
  }

  .btn-primary {
    background: #283618;
    color: #F0EFEB;
  }

  .btn-primary:hover {
    background: #1f2a12;
  }

  .btn-secondary {
    background: #D4D4D4;
    color: #283618;
  }

  .btn-secondary:hover {
    background: #B7B7A4;
  }

  .add-subsection-btn {
    display: block;
    width: 100%;
    padding: 16px;
    background: white;
    border: 1px dashed #D4D4D4;
    border-radius: 8px;
    color: #606060;
    cursor: pointer;
    font-size: 0.95rem;
    text-align: center;
    transition: all 0.15s ease;
  }

  .add-subsection-btn:hover {
    border-color: #283618;
    color: #283618;
    background: #F0EFEB;
  }

  .add-subsection-form {
    background: white;
    border: 1px solid #D4D4D4;
    border-radius: 8px;
    padding: 16px;
  }

  .add-subsection-input {
    width: 100%;
    padding: 10px 12px;
    border: 1px solid #D4D4D4;
    border-radius: 6px;
    font-size: 0.95rem;
    box-sizing: border-box;
    margin-bottom: 12px;
  }

  .add-subsection-input:focus {
    outline: none;
    border-color: #283618;
  }

  .add-subsection-actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }
</style>
