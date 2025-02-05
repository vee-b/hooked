<script>
    import { goto } from '$app/navigation'; // Import goto for navigation
    // @ts-ignore
    import { projectsList } from '../stores/projectsList'; 
    //import { dummyProjects } from '../stores/projectsList'; // Import the mockdata store

    /**
     * @type {{ id: any; formattedDateTime: any; imagePath: any; grade: any; isSent: any; attempts: any; }}
     */
    // @ts-ignore
    export let project; // Receive project object as prop
    
    // Function to edit the project
    async function editProject() {
      // Navigate to the edit project page
      goto('/editProject');
    }
    
    // Function to delete the project
    async function deleteProject() {
        console.log(`Project ID to delete: ${project.id}`);

        // Call delete function from the projectsList store
        // @ts-ignore
        await deleteProject(project.id);
        
        // Refresh the current route after deletion
        goto(window.location.pathname); // Navigate to the current path to refresh the view
    }
    
    // Helper function to format text like in the Flutter method
    /**
     * @param {string} text
     */
    function buildText(text) {
      return text;
    }
  </script>
  
  <style>
    .project-card {
      margin: 16px 16px 0;
      padding: 12px;
      border: 1px solid #ccc;
      border-radius: 4px;
    }
  
    .text-content {
      font-size: 18px;
      color: grey;
      margin: 6px 0;
    }
  
    .button {
      margin-top: 6px;
      display: block;
    }
  </style>
  
  <div class="project-card">
    <p class="text-content">{buildText(`Date & Time: ${project.formattedDateTime}`)}</p>
    <p class="text-content">{buildText(`Image: ${project.imagePath}`)}</p>
    <p class="text-content">{buildText(`Grade: ${project.grade}`)}</p>
    <p class="text-content">{buildText(`Sent: ${project.isSent}`)}</p>
    <p class="text-content">{buildText(`Attempts: ${project.attempts}`)}</p>
    <p class="text-content">{buildText(`ID: ${project.id}`)}</p>
  
    <button class="button" on:click={editProject}>
      Edit Project
    </button>
  
    <button class="button" on:click={deleteProject}>
      Delete Project
    </button>
  </div>
  