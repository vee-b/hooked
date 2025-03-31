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
    /* text-align: center; */
    padding: 1rem;
    font-family: 'Poppins', sans-serif;
    padding-bottom: 4rem;
  }

  .title {
    color: rgb(57, 57, 57);
    font-weight: lighter;
    margin-bottom: 20px;
    letter-spacing: 8px; /* Adjust the value to control the space between letters */
  }
</style>

<div class="home">
  <h1 class="title">Stats</h1>
  <SendsComponent />
</div>