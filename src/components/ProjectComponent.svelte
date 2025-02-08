<script>
    import { goto } from '$app/navigation'; // Import goto for navigation
    //import '../models/Project.js';

    // /**
    //  * @type {{ id: any; formatted_date_time: any; image_path: any; grade: any; is_sent: any; attempts: any; is_active: any; }}
    //  */
    // @ts-ignore

  //   /**
  //  * @type {{ id: any; formatted_date_time: string; image_path: any; grade: any; is_sent: any; attempts: any; }}
  //  */
     export let project; // Receive project object as prop
    
    // Navigation to edit the project
    async function editProject() {
      // Navigate to the edit project page
      goto('/editProject');
    }
    
    // Function to delete the project
    async function deleteProject() {
        // @ts-ignore
        console.log(`Project ID to delete: ${project.id}`);
        // Call delete function from the projectsList store
        // @ts-ignore
        await deleteProject(project.id);
        // Refresh the current route after deletion
        goto(window.location.pathname); // Navigate to the current path to refresh the view
    }
    
    // // Helper function to format text like in the Flutter method
    // /**
    //  * @param {string} text
    //  */
    // function buildText(text) {
    //   return text;
    // }
  </script>
  
  <style>
    .project-card {
      background-color: #f9f9f9;
      padding: 16px;
      margin: 12px 0;
      border-radius: 8px;
      box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }

    .project-label {
      font-weight: bold;
      color: #333;
    }

    .text-content {
      font-size: 16px;
      margin: 4px 0;
    }
  
    .button {
      margin-top: 8px;
      padding: 8px 12px;
      background-color: #007bff;
      color: white;
      border: none;
      border-radius: 4px;
      cursor: pointer;
    }

    .button:hover {
      background-color: #0056b3;
    }

    img {
      max-width: 100px;
      max-height: 100px;
      border-radius: 4px;
      margin-top: 8px;
    }
    
  </style>
    <div class="project-card">
      <p class="project-label">Date & Time: {project.formatted_date_time}</p>
      <!-- <p class="text-content">{project.date_time}</p> -->

    {#if project.image_path}
      <p class="project-label">Image:</p>
      <!-- <img src={project.image_path} alt="Project Image"> -->
    {/if}

    <p class="project-label">Grade: {project.grade}</p>
    <!-- <p class="text-content">{project.grade}</p> -->

    <p class="project-label">Sent: {project.is_sent ? "Yes" : "No"}</p>
    <!-- <p class="text-content">{project.is_sent ? "Yes" : "No"}</p> -->

    <p class="project-label">Attempts: {project.attempts}</p>
    <!-- <p class="text-content">{project.attempts}</p> -->

    <!-- <p class="project-label">Project ID:</p>
    <p class="text-content">{project.id}</p> -->

    <!-- <p class="project-label">Date & Time: {project.formatted_date_time}</p> -->
    <!-- <p class="text-content">{buildText(`Date & Time: ${project.formatted_date_time}`)}</p>
    {#if project.image_path} 
        <span class="project-label">Image:</span> <img src={project.image_path}>
    {/if}
    <p class="text-content">{buildText(`Grade: ${project.grade}`)}</p>
    <p class="text-content">{buildText(`Sent: ${project.is_sent}`)}</p>
    <p class="text-content">{buildText(`Attempts: ${project.attempts}`)}</p>
    <p class="text-content">{buildText(`ID: ${project.id}`)}</p> -->
  
    <button class="button" on:click={editProject}>
      Edit Project
    </button>
  
    <button class="button" on:click={deleteProject}>
      Delete Project
    </button>
  </div>