<script lang="ts">
  import { document, type CustomSection, type CustomSubsection } from '../stores/document';
  import CustomSectionEditor from './CustomSectionEditor.svelte';

  export let parentId: string;

  // Get custom subsections for this parent section
  $: customSection = ($document?.custom_sections || []).find(s => s.parent === parentId);
  $: customSubsections = customSection?.subsections || [];

  let showAddCustomForm = false;
  let newCustomName = '';

  function generateId(): string {
    return Math.random().toString(36).substring(2, 9);
  }

  function addCustomSubsection() {
    if (!newCustomName.trim() || !$document) return;

    const newSubsection: CustomSubsection = {
      id: generateId(),
      name: newCustomName.trim(),
      field_definitions: [],
      items: [],
    };

    if (customSection) {
      document.updateSection('custom_sections',
        ($document.custom_sections || []).map(s =>
          s.id === customSection.id
            ? { ...s, subsections: [...s.subsections, newSubsection] }
            : s
        )
      );
    } else {
      const newSection: CustomSection = {
        id: generateId(),
        name: parentId,
        parent: parentId,
        subsections: [newSubsection],
      };
      document.updateSection('custom_sections', [...($document.custom_sections || []), newSection]);
    }

    newCustomName = '';
    showAddCustomForm = false;
  }

  function updateCustomSubsection(subsectionId: string, updated: CustomSubsection) {
    if (!$document || !customSection) return;
    document.updateSection('custom_sections',
      ($document.custom_sections || []).map(s =>
        s.id === customSection.id
          ? { ...s, subsections: s.subsections.map(sub => sub.id === subsectionId ? updated : sub) }
          : s
      )
    );
  }

  function deleteCustomSubsection(subsectionId: string) {
    if (!$document || !customSection) return;
    document.updateSection('custom_sections',
      ($document.custom_sections || []).map(s =>
        s.id === customSection.id
          ? { ...s, subsections: s.subsections.filter(sub => sub.id !== subsectionId) }
          : s
      )
    );
  }

  function renameCustomSubsection(subsectionId: string, newName: string) {
    if (!$document || !customSection) return;
    document.updateSection('custom_sections',
      ($document.custom_sections || []).map(s =>
        s.id === customSection.id
          ? { ...s, subsections: s.subsections.map(sub => sub.id === subsectionId ? { ...sub, name: newName } : sub) }
          : s
      )
    );
  }
</script>

<!-- Custom Subsections -->
{#each customSubsections as subsection (subsection.id)}
  <CustomSectionEditor
    {subsection}
    sectionName={subsection.name}
    on:update={(e) => updateCustomSubsection(subsection.id, e.detail)}
    on:delete={() => deleteCustomSubsection(subsection.id)}
    on:rename={(e) => renameCustomSubsection(subsection.id, e.detail)}
  />
{/each}

<!-- Add Custom Subsection -->
{#if showAddCustomForm}
  <div class="add-custom-form">
    <input
      type="text"
      class="add-custom-input"
      placeholder="Subsection name"
      bind:value={newCustomName}
      on:keydown={(e) => e.key === 'Enter' && addCustomSubsection()}
    />
    <div class="add-custom-actions">
      <button class="btn-cancel" on:click={() => { showAddCustomForm = false; newCustomName = ''; }}>Cancel</button>
      <button class="btn-create" on:click={addCustomSubsection}>Create</button>
    </div>
  </div>
{:else}
  <button class="add-custom-btn" on:click={() => (showAddCustomForm = true)}>
    + Add Custom Subsection
  </button>
{/if}

<style>
  .add-custom-btn {
    display: block;
    width: 100%;
    padding: 16px;
    margin-top: 24px;
    background: white;
    border: 1px dashed #D4D4D4;
    border-radius: 8px;
    color: #606060;
    cursor: pointer;
    font-size: 0.95rem;
    text-align: center;
    transition: all 0.15s ease;
  }

  .add-custom-btn:hover {
    border-color: #283618;
    color: #283618;
    background: #F0EFEB;
  }

  .add-custom-form {
    background: white;
    border: 1px solid #D4D4D4;
    border-radius: 8px;
    padding: 16px;
    margin-top: 24px;
  }

  .add-custom-input {
    width: 100%;
    padding: 10px 12px;
    border: 1px solid #D4D4D4;
    border-radius: 6px;
    font-size: 0.95rem;
    box-sizing: border-box;
    margin-bottom: 12px;
  }

  .add-custom-input:focus {
    outline: none;
    border-color: #283618;
  }

  .add-custom-actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }

  .btn-cancel, .btn-create {
    padding: 8px 16px;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.9rem;
    font-weight: 500;
  }

  .btn-cancel {
    background: #D4D4D4;
    color: #283618;
  }

  .btn-cancel:hover {
    background: #B7B7A4;
  }

  .btn-create {
    background: #283618;
    color: #F0EFEB;
  }

  .btn-create:hover {
    background: #1f2a12;
  }
</style>
