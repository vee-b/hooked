<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from 'svelte';
  import { checkLoginStatus } from '../../controllers/accountsController';

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