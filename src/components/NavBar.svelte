<!-- src/components/NavBar.svelte -->
<script lang="ts">
  import { goto } from '$app/navigation';
  import { Home, ChartLine, CalendarDays, Settings, ChartBar, ChartPie, Filter, Camera, Upload, Clock, PlusCircle, Activity, User, LogOut } from 'lucide-svelte'; // Example icons, adjust as needed
  import { page } from '$app/stores'; // To access the current route
  import { logoutAccount } from '../controllers/accountsController';
  import ConfirmationBox from './ConfirmationBox.svelte';

  // STATE
  let currentPath: string; // Store the current route path
  let showLogoutConfirm = false; // Controls showing the confirmation modal

  // REACTIVE: get the current route
  // This runs whenever `$page` changes, keeping `currentPath` updated
  $: currentPath = $page.url.pathname;

  // UTILITY: Check if a route is active
  // Used to highlight the nav item
  function isActive(route: string): boolean {
    return currentPath === route;
  }

  // LOGOUT HANDLING
  // Calls logout logic
  const handleLogout = () => {
    logoutAccount();
  };

  // Confirm logout: logs out then redirects to login page
  async function confirmLogout() {
    await logoutAccount();
    await goto('/login');
  }
</script>
  
<style>
  nav {
    position: fixed;
    bottom: 0;
    left: 0;
    width: 100%;
    display: flex;
    justify-content: space-evenly;
    align-items: center;
    padding: 0.5rem 0;
    z-index: 10;
    background-color: #f3f9f9fa;
    font-family: 'Poppins', sans-serif;
  }

  button {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-decoration: none;
    color: black;
    font-size: 0.8rem;
    padding: 1rem;
    border: none;
    border-radius: 10px;
    background: #f3f9f9fa;
    box-shadow: 5px 5px 10px rgba(0, 0, 0, 0.123), -5px -5px 10px #ffffff;
    transition: all 0.3s ease;
  }

  button:hover,
  button.active {
    box-shadow: inset 3px 3px 6px rgba(0, 0, 0, 0.123), inset -3px -3px 6px #ffffff;
  }

  a {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-decoration: none;
    color: black;
    font-size: 0.8rem;
    padding: 1rem;
    border: none;
    border-radius: 10px;
    background: #f3f9f9fa;
    box-shadow: 5px 5px 10px rgba(0, 0, 0, 0.123), -5px -5px 10px #ffffff;
    transition: all 0.3s ease;
  }

  a:hover,
  a.active {
    box-shadow: inset 3px 3px 6px rgba(0, 0, 0, 0.123), inset -3px -3px 6px #ffffff;
  }

  .icon {
    margin-bottom: 0.2rem;
  }
</style>

<nav>
  <!-- Home Link -->
  <a href="/" class={$page.url.pathname === '/' ? 'active' : ''}>
    <Home class="icon" size={18} />
  </a>
  <!-- Inactive Projects Link -->
  <a href="/inactiveProjects" class={$page.url.pathname === '/inactiveProjects' ? 'active' : ''}>
    <CalendarDays class="icon" size={18}/>
  </a>
  <!-- Stats Page Link -->
  <a href="/stats" class={$page.url.pathname === '/stats' ? 'active' : ''}>
    <ChartLine class="icon" size={18}/>
  </a>
  <!-- Settings Page Link -->
  <a href="/settings" class={$page.url.pathname === '/settings' ? 'active' : ''}>
    <Settings class="icon" size={18}/>
  </a>
  <!-- Logout Button (opens confirmation modal) -->
  <button on:click={() => showLogoutConfirm = true} class="nav-button">
    <LogOut class="icon" size={18} />
  </button>
</nav>

<!-- 
MODAL: Confirmation Box for logging out
Appears only when showLogoutConfirm is true
-->
{#if showLogoutConfirm}
  <ConfirmationBox 
    message="Are you sure you want to log out?"
    onConfirm={async () => {
      showLogoutConfirm = false;
      await confirmLogout();
    }}
    onCancel={() => showLogoutConfirm = false} 
  />
{/if}