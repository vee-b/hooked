<script lang="ts">
    import { gradeSystem, fontScale, vScale } from '../stores/settingsStore';
    
    // Function to select the correct grade scale based on the grade system
    function getGradeSystem() {
        return $gradeSystem === "Font Scale" ? fontScale : vScale;
    }

    // Reactive assignment for the current grade scale
    $: currentGrades = getGradeSystem();
  </script>
  
  <p>Selected Grade: {currentGrades[0]}</p>
  