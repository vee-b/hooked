<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from 'svelte';
  import SendsComponent from '../../components/SendsComponent.svelte'; // Adjust the path as necessary
  import { checkLoginStatus, logoutAccount } from '../../controllers/accountsController';

  // Handle logout on button click
  // const handleLogout = () => {
  //   logoutAccount();
  // };

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

  /* .logout-button {
    padding: 0.5rem 1rem;
    font-weight: 500;
    background: #ffffff;
    box-shadow: 5px 5px 10px rgba(0, 0, 0, 0.1), -5px -5px 10px #ffffff;
    border-radius: 10px;
    transition: box-shadow 0.2s ease;
    border: none;
    cursor: pointer;
  }

  .logout-button:hover {
    box-shadow: inset 3px 3px 6px rgba(0, 0, 0, 0.1), inset -3px -3px 6px #ffffff;
  } */

  .divider {
    height: 10px;
    margin: 20px 0;
    border-top: 1px solid #ccc;
  }
</style>

<div class="home">
  <div class="header-container">
    <h1 class="title">Stats</h1>
    <!-- <button class="logout-button" on:click={handleLogout}>Logout</button> -->
  </div>

  <div class="divider"></div>

  <SendsComponent />
</div>