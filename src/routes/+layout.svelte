<script>
    import NavBar from '../components/NavBar.svelte';
  </script>
  
  <div class="flex flex-col min-h-screen">
    <main class="flex-grow">
      <slot />
    </main>
    <NavBar />
  </div>
  

<!-- <script>
    import NavBar from '../components/NavBar.svelte';
  </script>
  
  <slot />
  <NavBar />
   -->