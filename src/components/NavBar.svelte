<script>
    import { goto } from '$app/navigation';
    import { Home, ChartBar, ChartPie, ChartLine, Filter, Camera, Upload, Clock, PlusCircle, Activity, User, CalendarDays } from 'lucide-svelte'; // Example icons, adjust as needed
  </script>
  
  <style>
    nav {
      position: fixed;
      bottom: 0;
      left: 0;
      width: 100%;
      background: #ffffff;
      border-top: 1px solid #e0e0e0;
      display: flex;
      justify-content: space-evenly;
      align-items: center;
      padding: 0.5rem 0;
      z-index: 10;
    }
  
    a {
      display: flex;
      flex-direction: column;
      align-items: center;
      text-decoration: none;
      color: #666;
      font-size: 0.8rem;
    }
  
    a:hover {
      color: #111;
    }
  
    .icon {
      width: 24px;
      height: 24px;
      margin-bottom: 0.2rem;
    }
  </style>
  
  <nav>
    <a href="/">
      <Home class="icon" />
      <span>Home</span>
    </a>
    <!-- <a href="/camera">
      <PlusCircle class="icon" />
      <span>Add Project</span>
    </a> -->
    <a href="/inactiveProjects">
      <CalendarDays class="icon" />
      <span>History</span>
    </a>
    <a href="/stats">
      <ChartLine class="icon" />
      <span>Stats</span>
    </a>
    <!-- <a href="/profile">
      <User class="icon" />
      <span>Profile</span>
    </a> -->
  </nav>
  
  

<!-- <script>
    import { goto } from '$app/navigation';
  </script>
  
  <nav class="fixed bottom-0 left-0 right-0 bg-white border-t shadow-md flex justify-around py-3">
    <button on:click={() => goto('/')} class="text-gray-600 hover:text-blue-600">
      Home
    </button>
    <button on:click={() => goto('/newProject')} class="text-gray-600 hover:text-blue-600">
      New Project
    </button>
    <button on:click={() => goto('/settings')} class="text-gray-600 hover:text-blue-600">
      Settings
    </button>
  </nav>
  
  <style>
    nav {
      z-index: 10;
    }
  </style> -->
  