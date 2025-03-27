<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from 'svelte';
  import SendsComponent from '../../components/SendsComponent.svelte'; // Adjust the path as necessary
  import { checkLoginStatus } from '../../controllers/accountsController';

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
  .home {
    text-align: center;
    padding: 2rem;
    font-family: Arial, sans-serif;
    padding-bottom: 4rem;
  }
</style>

<div class="home">
  <h1>Stats</h1>
  <SendsComponent />
</div>