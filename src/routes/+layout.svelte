<!-- <link rel="stylesheet" href="/styles/global.css" /> -->

<script lang="ts">
    import NavBar from '../components/NavBar.svelte';
    import { page } from '$app/stores';

    // Check if the current route is not login or register
    $: hideNavBar = $page.url.pathname === '/login' || $page.url.pathname === '/register' || $page.url.pathname === '/resetPassword';
  </script>
  
  <div class="flex flex-col min-h-screen">
    <main class="flex-grow">
      <slot />
    </main>
    <!-- <NavBar /> -->
    {#if !hideNavBar}
      <NavBar />
    {/if}
  </div>