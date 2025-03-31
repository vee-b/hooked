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
    display: flex;
    flex-direction: column;
    max-width: 400px;
    margin: auto;
    padding: 20px;
    position: relative;
  }

  .title {
    color: rgb(57, 57, 57);
    font-weight: lighter;
    margin-bottom: 20px;
    letter-spacing: 8px; /* Adjust the value to control the space between letters */
  }

  .grade-dropdown {
    border: p-2 rounded;
    width: 40%;
  }

  .logout-button {
    padding: 1rem;
    border: none;
    width: 25%;
    border-radius: 10px;
    font-size: 1rem;
    cursor: pointer;
    background: #e6f4fd;
    box-shadow: 5px 5px 10px #b4d1e3, -5px -5px 10px #ffffff;
    transition: all 0.3s ease;
    position: absolute; /* Position relative to the .settings container */
    top: 10px;  /* Adjust for distance from top */
    right: 10px; /* Adjust for distance from right */
  }

  .logout-button:hover {
    box-shadow: inset 3px 3px 6px #b4d1e3, inset -3px -3px 6px #ffffff;
  }
</style>

<div class="settings">
  <h1 class="title">Settings</h1>

  <!-- Dropdown for Grade System -->
  <!-- <label for="grade-system">Choose Grade System:</label> -->
  <select 
    id="grade-system" 
    bind:value={$gradeSystem} 
    on:change={handleGradeChange} 
    class="grade-dropdown"
  >
    <option value="V-Scale">V-Scale</option>
    <option value="Font Scale">Font Scale</option>
  </select>

  <!-- <p>Grade System: {$gradeSystem}</p> -->

  <button on:click={handleLogout} class="logout-button">
    Logout
  </button>
</div>