<script lang="ts">
  export let message = "Are you sure?";
  export let onConfirm: () => void;
  export let onCancel: () => void;
</script>

<style>
  .box-overlay {
    position: fixed;
    top: 0; left: 0; right: 0; bottom: 0;
    background: rgba(0,0,0,0.3);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 9999;
  }

  .box-card {
    background: #ffffff;
    border-radius: 1rem;
    padding: 2rem;
    margin: 1rem;
    width: 90%;
    max-width: 400px;
    text-align: center;
    font-family: 'Inter', sans-serif;
  }

  .box-message {
    font-size: 1rem;
    color: #374151;
    margin-bottom: 1.5rem;
  }

  .box-buttons {
    display: flex;
    gap: 1rem;
    justify-content: center;
  }

  .box-btn {
    flex: 1;
    padding: 0.75rem;
    font-size: 1rem;
    border: none;
    border-radius: 1rem;
    background: #ffffff;
    box-shadow: 5px 5px 10px rgba(0,0,0,0.1), -5px -5px 10px #ffffff;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .box-btn:hover {
    box-shadow: inset 3px 3px 6px rgba(0,0,0,0.1), inset -3px -3px 6px #ffffff;
  }
</style>

<div class="box-overlay">
  <div class="box-card">
    <div class="box-message">{message}</div>
    <div class="box-buttons">
      <button class="box-btn" on:click={onConfirm}>Yes</button>
      <button class="box-btn" on:click={onCancel}>No</button>
    </div>
  </div>
</div>
