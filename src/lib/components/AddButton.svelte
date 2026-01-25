<script lang="ts">
  export let label: string = 'Add Item';
</script>

<button class="add-btn" on:click>
  <span class="plus">+</span>
  <span>{label}</span>
</button>

<style>
  .add-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 16px;
    background: #f0f7ff;
    border: 2px dashed #1976d2;
    border-radius: 8px;
    color: #1976d2;
    font-size: 0.95rem;
    cursor: pointer;
    width: 100%;
    justify-content: center;
  }

  .add-btn:hover {
    background: #e3f2fd;
  }

  .plus {
    font-size: 1.25rem;
    font-weight: bold;
  }
</style>
