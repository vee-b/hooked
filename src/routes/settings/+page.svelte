<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from 'svelte';
  import { checkLoginStatus, logoutAccount } from '../../controllers/accountsController';
  import { gradeSystem, setGradeSystem } from '../../stores/settingsStore';

  // EVENT HANDLER FOR DROPDOWN
  // Whenever user changes dropdown, update the global grade system
  const handleGradeChange = (event: Event) => {
    const target = event.target as HTMLSelectElement;
    setGradeSystem(target.value);
  };

  // ON MOUNT: CHECK LOGIN
  onMount(async () => {
    const isLoggedIn = checkLoginStatus();
    if (!isLoggedIn) {
      goto('/login'); // Redirect if not logged in
    }
  })
</script>

<style>
  .home {
    padding: 1rem;
    font-family: 'Poppins', sans-serif;
    padding-bottom: 4rem;
    color: black;
  }
  
  .settings {
    padding-left: 2rem;
    padding-right: 2rem;
    font-family: 'Poppins', sans-serif;

    max-width: 600px;
    width: 80%;
    margin: 1rem auto;
  }

  .header-container {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem 2rem;
    flex-wrap: wrap;
    gap: 1rem;
  }

  .title {
    color: rgb(57, 57, 57);
    font-size: 2rem;
    letter-spacing: 8px;
    margin: 0;
  }

  .divider {
    height: 10px;
    margin: 20px 0;
    border-top: 1px solid #ccc;
  }

  .grade-dropdown {
    padding: 0.5rem;
    font-size: 0.875rem;
    border-radius: 10px;
    border: 1px solid #ccc;
    background: #ffffff;
    box-shadow: 5px 5px 10px rgba(0, 0, 0, 0.05), -5px -5px 10px #ffffff;
    cursor: pointer;
    transition: box-shadow 0.2s ease;
    width: 100%;
    margin-top: 1rem;
  }

  .grade-dropdown:hover {
    box-shadow: inset 3px 3px 6px rgba(0, 0, 0, 0.1), inset -3px -3px 6px #ffffff;
  }
</style>

<div class="home">
  <div class="header-container">
    <h1 class="title">Settings</h1>
  </div>

  <div class="divider"></div>

  <!-- Dropdown for Grade System -->
  <div class="settings">
    <!-- 'bind:value={$gradeSystem}' Binds directly to global writable store -->
    <!-- 'on:change={handleGradeChange}' Still fires handler to ensure explicit update  -->
    <select 
      id="grade-system" 
      bind:value={$gradeSystem} 
      on:change={handleGradeChange} 
      class="grade-dropdown"
    >
      <option value="V-Scale">V-Scale</option>
      <option value="Font Scale">Font Scale</option>
    </select>
  </div>
</div>