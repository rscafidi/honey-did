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
    background: #F0EFEB;
    border: 2px dashed #283618;
    border-radius: 8px;
    color: #283618;
    font-size: 0.95rem;
    cursor: pointer;
    width: 100%;
    justify-content: center;
    font-weight: 500;
    transition: all 0.15s ease;
  }

  .add-btn:hover {
    background: #DDE5B6;
  }

  .plus {
    font-size: 1.25rem;
    font-weight: bold;
  }
</style>
