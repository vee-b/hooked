<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from 'svelte';
  import SendsComponent from '../../components/SendsComponent.svelte'; // Adjust the path as necessary
  import { checkLoginStatus, logoutAccount } from '../../controllers/accountsController';

  // Handle logout on button click
  const handleLogout = () => {
    logoutAccount();
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
  .home {
    /* text-align: center; */
    padding: 1rem;
    font-family: 'Poppins', sans-serif;
    padding-bottom: 4rem;
  }

  .logout-button-wrapper {
    display: inline;
    position: absolute;
    top: 1rem;
    right: 1rem; /* Place the logout button at the top right */
  }

  .logout-button {
    display: flex;
    align-items: center;
    background: none;
    border: none;
    width: 80px;
    height: 45px;
    cursor: pointer;
    border-radius: 8px;
    transition: background 0.3s ease;
    margin-right: 1rem;
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
  
  <div class="logout-button-wrapper">
    <button class="logout-button" on:click={handleLogout}>
      Logout
    </button>
  </div>

  <SendsComponent />
</div>