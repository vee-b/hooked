<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from 'svelte';
  import { checkLoginStatus, logoutAccount } from '../../controllers/accountsController';
  import { gradeSystem, setGradeSystem } from '../../stores/settingsStore';

  // Handle logout on button click
  const handleLogout = () => {
    logoutAccount();
  };

  // Handle dropdown change event
  const handleGradeChange = (event: Event) => {
    const target = event.target as HTMLSelectElement;
    setGradeSystem(target.value);
  };

  // Fetch project details on mount
  onMount(async () => {
    // Check if user if logged in
    const isLoggedIn = checkLoginStatus();
    if (!isLoggedIn) {
      goto('/login'); // Redirect if not logged in
    }
  })
</script>

<style>
  .settings {
    max-width: 400px;
    margin: auto;
    padding: 20px;
  }
</style>

<div class="settings">
  <h1>Settings</h1>

  <!-- Dropdown for Grade System -->
  <label for="grade-system">Choose Grade System:</label>
  <select 
    id="grade-system" 
    bind:value={$gradeSystem} 
    on:change={handleGradeChange} 
    class="border p-2 rounded"
  >
    <option value="V-Scale">V-Scale</option>
    <option value="Font Scale">Font Scale</option>
  </select>

  <p>Current Grade System: {$gradeSystem}</p>
</div>

<button on:click={handleLogout} class="bg-red-500 px-4 py-2 rounded hover:bg-red-700">
  Logout
</button>