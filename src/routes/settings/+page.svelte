<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from 'svelte';
  import { checkLoginStatus, logoutAccount } from '../../controllers/accountsController';

  // Handle logout on button click
  const handleLogout = () => {
    logoutAccount();
  };

  // Function to navigate to the home page
  const navigateToHome = () => {
      goto('/'); // Change the path to your actual route for the new project page
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

<div class="settings">
    <h1>Settings</h1>
  </div>

<button class="button" on:click={navigateToHome}>Home</button>
<button on:click={handleLogout} class="bg-red-500 px-4 py-2 rounded hover:bg-red-700">
  Logout
</button>