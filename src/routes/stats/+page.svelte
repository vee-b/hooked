<script>
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation'; // Import the goto function for navigation
    import SendsComponent from '../../components/SendsComponent.svelte'; // Adjust the path as necessary
    import ProjectComponent from '../../components/ProjectComponent.svelte';
    import { writable } from 'svelte/store';
  
    // Define the Project type for JSDoc
    /**
     * @typedef {Object} Project
     * @property {boolean} isSent
     * @property {string} grade
     * @property {string} formattedDateTime
     * @property {string} imagePath
     * @property {number} attempts
     * @property {number} id
     */
  
    // Store for project list (assuming you have a method to fetch projects)
    /** 
     * @type {import('svelte/store').Writable<Project[]>} 
     */
    const projectsList = writable([]); // Replace with your actual project data structure
  
    // You can add your JavaScript logic here
    let message = "Stats";
    let totalSends = 0; // Variable to hold the total sends count
  
    // Function to fetch projects (you'll need to implement this)
    const fetchProjects = async () => {
      // Fetch your project list and sends count here
      const response = await fetch('/api/projects'); // Replace with your actual API endpoint
      const data = await response.json();
      
      // Assuming your API returns a list of projects and total sends
      projectsList.set(data.projects);
      totalSends = data.totalSends; // Set the total sends count
    };
  
    // Fetch projects on mount
    onMount(() => {
      fetchProjects();
    });
  </script>
  
  <style>
    .home {
        text-align: center;
        padding: 2rem;
        font-family: Arial, sans-serif;
    }
  
    .button {
      margin: 1rem;
      padding: 1rem;
      background-color: #007bff;
      color: white;
      border: none;
      border-radius: 5px;
      cursor: pointer;
      font-size: 16px;
    }
  
    .button:hover {
        background-color: #0056b3;
    }
  
    .divider {
      height: 30px;
      border-top: 1px solid #ccc;
      margin: 20px 0;
    }
  </style>
  
  <div class="home">
    <h1>{message}</h1>
    <p>Total Sends: {totalSends}</p>
  
    <SendsComponent {projectsList} /> <!-- Include the SendsCard component -->
    
    <div class="divider"></div>
  
    <!-- Render each project using ProjectComponent -->
      {#each $projectsList as project (project.id)}
        <ProjectComponent {project} /> <!-- Pass the project prop to ProjectsComponent -->
      {/each}
  </div>
  