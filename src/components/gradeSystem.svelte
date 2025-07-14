<!-- src/components/gradeSystem.svelte -->

<script lang="ts">
    import { gradeSystem, fontScale, vScale } from '../stores/settingsStore';
    
    // HELPER FUNCTION: Select the correct grade scale
    // Function to select the correct grade scale based on the grade system
    function getGradeSystem() {
        // If the selected grade system is "Font Scale", return the font scale array.
        // Otherwise default to V scale.
        return $gradeSystem === "Font Scale" ? fontScale : vScale;
    }

    // REACTIVE STATEMENT: whenever $gradeSystem changes,
    // this will automatically reassign `currentGrades`.
    $: currentGrades = getGradeSystem();
</script>
  
<!-- 
MARKUP: Display the first item of the selected scale.
This could be expanded to a dropdown or list.
-->
<p>Selected Grade: {currentGrades[0]}</p>
  