<script>
  import { goto } from "$app/navigation";

  // Function to navigate to the home page
  const navigateToHome = () => {
      goto('/'); // Change the path to your actual route for the new project page
    };
</script>

<div class="settings">
    <h1>Settings</h1>
  </div>

<button class="button" on:click={navigateToHome}>Home</button>