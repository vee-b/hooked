<script>
    import { goto } from '$app/navigation';
    import { Home, ChartLine, CalendarDays, Settings, ChartBar, ChartPie, Filter, Camera, Upload, Clock, PlusCircle, Activity, User, LogOut } from 'lucide-svelte'; // Example icons, adjust as needed
  </script>
  
  <style>
    nav {
      position: fixed;
      bottom: 0;
      left: 0;
      width: 100%;
      background: #121212;
      /* border-top: 1px solid #00ff8092; */
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
      /* color: #666; */
      color: white;
      font-size: 0.8rem;
    }

    a span {
      font-size: 0.8rem;
      line-height: 1.2; /* Ensures good readability */
    }

  
    a:hover {
      /* color: #111; */
      color: #00ff8092;
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
    <a href="/inactiveProjects">
      <CalendarDays class="icon" />
      <span>Inactive<br>Projects</span>
    </a>
    <a href="/stats">
      <ChartLine class="icon" />
      <span>Stats</span>
    </a>
    <a href="/settings">
      <Settings class="icon" />
      <span>Settings</span>
    </a>
  </nav>