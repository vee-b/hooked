<script lang="ts">
    import { goto } from '$app/navigation';
    import { Home, ChartLine, CalendarDays, Settings, ChartBar, ChartPie, Filter, Camera, Upload, Clock, PlusCircle, Activity, User, LogOut } from 'lucide-svelte'; // Example icons, adjust as needed
    import { page } from '$app/stores'; // To access the current route
  
    // Subscribe to the page store to reactively track the current path
    let currentPath: string;

    // Subscribe to the page store and set currentPath to the current pathname
    $: currentPath = $page.url.pathname;

    // Helper function to check if the route is active
    function isActive(route: string): boolean {
      return currentPath === route;
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
      background-color: #e6f4fd;
      font-family: 'Poppins', sans-serif;
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
      background: #e6f4fd;
      box-shadow: 5px 5px 10px #b4d1e3, -5px -5px 10px #ffffff;
      transition: all 0.3s ease;
    }

    a span {
      font-size: 0.8rem;
      line-height: 1.2; /* Ensures good readability */
    }

  
    a:hover,
    a.active {
      box-shadow: inset 3px 3px 6px #b4d1e3, inset -3px -3px 6px #ffffff;
    }
  
    .icon {
      width: 24px;
      height: 24px;
      margin-bottom: 0.2rem;
    }
  </style>
  
  <nav>
    <a href="/" class={$page.url.pathname === '/' ? 'active' : ''}>
      <Home class="icon" />
    </a>
    <a href="/inactiveProjects" class={$page.url.pathname === '/inactiveProjects' ? 'active' : ''}>
      <CalendarDays class="icon" />
    </a>
    <a href="/stats" class={$page.url.pathname === '/stats' ? 'active' : ''}>
      <ChartLine class="icon" />
    </a>
    <a href="/settings" class={$page.url.pathname === '/settings' ? 'active' : ''}>
      <Settings class="icon" />
    </a>
  </nav>