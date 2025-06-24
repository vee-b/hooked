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
    padding: 1rem;
    font-family: 'Poppins', sans-serif;
    padding-bottom: 4rem;
    color: black;
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
</style>

<div class="home">
  <div class="header-container">
    <h1 class="title">Stats</h1>
  </div>

  <div class="divider"></div>

  <SendsComponent />
</div>