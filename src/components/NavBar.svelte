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
      /* background: #121212; */
      /* border-top: 1px solid black; */
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
      /* color: #666; */
      color: black;
      font-size: 0.8rem;

      padding: 1rem;
      border: none;
      /* width: 10%; */
      /* height: 10%; */
      border-radius: 10px;
      /* font-size: 1rem; */
      /* cursor: pointer; */
      background: #e6f4fd;
      box-shadow: 5px 5px 10px #b4d1e3, -5px -5px 10px #ffffff;
      transition: all 0.3s ease;
    }

    a span {
      font-size: 0.8rem;
      line-height: 1.2; /* Ensures good readability */
    }

  
    a:hover {
      /* color: #111; */
      /* color: #00ff8092; */
      box-shadow: inset 3px 3px 6px #b4d1e3, inset -3px -3px 6px #ffffff;
    }
  
    .icon {
      width: 24px;
      height: 24px;
      margin-bottom: 0.2rem;
    }
  </style>



<!-- <style global>
  body {
    background-color: #e6f4fd;
    font-family: 'Poppins', sans-serif;
    margin: 0;
    padding: 0;
  }

  h2 {
    color: rgb(57, 57, 57);
    font-size: 1rem;
    font-weight: lighter;
    margin-bottom: 20px;
    letter-spacing: 8px; /* Adjust the value to control the space between letters */
    /* -webkit-text-stroke: 1px #00000048; */
  }

  .login-container {
    max-width: 450px;
    margin: 100px auto;
    padding-left: 40px;
    padding-right: 40px;
    margin-top: 40px;
    background: #e6f4fd;
    border-radius: 20px;
    text-align: center;
    transition: all 0.3s ease;
    box-sizing: border-box; /* Ensure padding doesn't affect width */
  }

  .input-field {
    margin-bottom: 1rem;
    padding: 1rem;
    border-radius: 10px;
    border: none;
    background: #e6f4fd;
    box-shadow: inset 5px 5px 10px #b4d1e3, inset -5px -5px 10px #ffffff;
    color: #333;
    font-size: 1rem;
    outline: none;
    width: 100%;
    max-width: 100%; /* Ensures full width within the container */
    box-sizing: border-box; /* Prevents padding from affecting width */
  }

  .input-field:focus {
    box-shadow: inset 3px 3px 6px #b4d1e3, inset -3px -3px 6px #ffffff;
  }

  .button-container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-top: 1rem;
  }

  .submit-button,
  .register-button {
    padding: 1rem;
    border: none;
    width: 100%;
    border-radius: 10px;
    font-size: 1rem;
    cursor: pointer;
    background: #e6f4fd;
    box-shadow: 5px 5px 10px #b4d1e3, -5px -5px 10px #ffffff;
    transition: all 0.3s ease;
  }

  .submit-button:hover,
  .register-button:hover {
    box-shadow: inset 3px 3px 6px #b4d1e3, inset -3px -3px 6px #ffffff;
  }

  .error-message {
    color: red;
    margin-bottom: 10px;
  }

  .logo {
    width: 100%;
    max-width: 400px;
    opacity: 0.85;
    margin-bottom: 20px;
    border-radius: 10px;
  }
</style> -->




  
  <nav>
    <a href="/">
      <Home class="icon" />
      <!-- <span>Home</span> -->
    </a>
    <a href="/inactiveProjects">
      <CalendarDays class="icon" />
      <!-- <span>Inactive<br>Projects</span> -->
    </a>
    <a href="/stats">
      <ChartLine class="icon" />
      <!-- <span>Stats</span> -->
    </a>
    <a href="/settings">
      <Settings class="icon" />
      <!-- <span>Settings</span> -->
    </a>
  </nav>