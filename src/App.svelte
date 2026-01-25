<script lang="ts">
  import { onMount } from 'svelte';
  import { document } from './lib/stores/document';
  import FinancialSection from './lib/sections/FinancialSection.svelte';

  type Section =
    | 'financial' | 'insurance' | 'bills' | 'property' | 'legal'
    | 'digital' | 'household' | 'personal' | 'contacts' | 'medical' | 'pets';

  let currentSection: Section = 'financial';
  let showExportDialog = false;

  const sections: { id: Section; label: string; icon: string }[] = [
    { id: 'financial', label: 'Financial', icon: '💰' },
    { id: 'insurance', label: 'Insurance', icon: '🛡️' },
    { id: 'bills', label: 'Bills', icon: '📄' },
    { id: 'property', label: 'Property', icon: '🏠' },
    { id: 'legal', label: 'Legal', icon: '⚖️' },
    { id: 'digital', label: 'Digital Life', icon: '💻' },
    { id: 'household', label: 'Household', icon: '🔧' },
    { id: 'personal', label: 'Personal', icon: '💝' },
    { id: 'contacts', label: 'Contacts', icon: '📞' },
    { id: 'medical', label: 'Medical', icon: '🏥' },
    { id: 'pets', label: 'Pets', icon: '🐾' },
  ];

  function getSectionStatus(sectionId: Section): 'empty' | 'partial' | 'complete' {
    // TODO: Implement actual status checking
    return 'empty';
  }

  onMount(() => {
    document.load();
  });
</script>

<main class="app">
  <aside class="sidebar">
    <div class="logo">
      <h1>honey-did</h1>
    </div>
    <nav class="nav">
      {#each sections as section}
        <button
          class="nav-item"
          class:active={currentSection === section.id}
          on:click={() => (currentSection = section.id)}
        >
          <span class="nav-icon">{section.icon}</span>
          <span class="nav-label">{section.label}</span>
          <span class="nav-status" data-status={getSectionStatus(section.id)}></span>
        </button>
      {/each}
    </nav>
    <div class="sidebar-footer">
      <button class="btn btn-secondary" on:click={() => {}}>
        Import File
      </button>
      <button class="btn btn-primary" on:click={() => (showExportDialog = true)}>
        Export
      </button>
    </div>
  </aside>
  <section class="content">
    <header class="content-header">
      <h2>{sections.find((s) => s.id === currentSection)?.label}</h2>
    </header>
    <div class="content-body">
      {#if currentSection === 'financial'}
        <FinancialSection />
      {:else}
        <p>Section content for {currentSection} coming soon...</p>
      {/if}
    </div>
  </section>
</main>

<style>
  .app {
    display: flex;
    height: 100vh;
    background: #f5f5f5;
  }

  .sidebar {
    width: 240px;
    background: white;
    border-right: 1px solid #e0e0e0;
    display: flex;
    flex-direction: column;
  }

  .logo {
    padding: 20px;
    border-bottom: 1px solid #e0e0e0;
  }

  .logo h1 {
    margin: 0;
    font-size: 1.5rem;
    color: #333;
  }

  .nav {
    flex: 1;
    overflow-y: auto;
    padding: 10px;
  }

  .nav-item {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 12px;
    border: none;
    background: none;
    border-radius: 8px;
    cursor: pointer;
    text-align: left;
    gap: 12px;
  }

  .nav-item:hover {
    background: #f0f0f0;
  }

  .nav-item.active {
    background: #e3f2fd;
    color: #1976d2;
  }

  .nav-icon {
    font-size: 1.25rem;
  }

  .nav-label {
    flex: 1;
  }

  .nav-status {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: #e0e0e0;
  }

  .nav-status[data-status='partial'] {
    background: #ffc107;
  }

  .nav-status[data-status='complete'] {
    background: #4caf50;
  }

  .sidebar-footer {
    padding: 15px;
    border-top: 1px solid #e0e0e0;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .btn {
    padding: 10px 16px;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.9rem;
  }

  .btn-primary {
    background: #1976d2;
    color: white;
  }

  .btn-primary:hover {
    background: #1565c0;
  }

  .btn-secondary {
    background: #e0e0e0;
    color: #333;
  }

  .btn-secondary:hover {
    background: #d0d0d0;
  }

  .content {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .content-header {
    padding: 20px;
    background: white;
    border-bottom: 1px solid #e0e0e0;
  }

  .content-header h2 {
    margin: 0;
  }

  .content-body {
    flex: 1;
    padding: 20px;
    overflow-y: auto;
  }
</style>
