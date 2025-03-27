// src/stores/settingsStore.ts

import { writable } from 'svelte/store';

// Check localStorage for the user's last selection
const storedSystem = localStorage.getItem('gradeSystem') || 'V-Scale';

// Writable store for the grading system (false = V-Scale, true = Font Scale)
export const gradeSystem = writable<string>(storedSystem);

// Export the grade scales so they can be reused
export const vScale = ['V0', 'V1', 'V2', 'V3', 'V4', 'V5', 'V6', 'V7', 'V8', 'V9', 'V10', 'V11', 'V12', 'V13', 'V14', 'V15', 'V16', 'V17'];
export const fontScale = ['4', '5', '5+', '6A/6A+', '6B/6B+', '6C/6C+', '7A', '7A+', '7B/7B+', '7B+/7C', '7C+', '8A', '8A+', '8B', '8B+', '8C', '8C+', '9A'];

// Helper function to get the current grade scale
export function getCurrentGrades(currentSystem: string): string[] {
    return currentSystem === "Font Scale" ? fontScale : vScale;
  }

// Convert a v-scale grade to a font-scale grade
export function convertVScaleGrade(grade: string, currentSystem: string): string {
    if (currentSystem === "Font Scale") {
      const index = vScale.indexOf(grade);
      return index !== -1 ? fontScale[index] : grade;
    }
    return grade;
  }

// Convert a font-scale grade to a v-scale grade.
export function convertFontScaleGrade(grade: string, currentSystem: string): string {
  if (currentSystem === "Font Scale") {
    // Find the index in the fontScale array
    const index = fontScale.indexOf(grade);
    return index !== -1 ? vScale[index] : grade;
  }
  // If not in Font Scale mode, return the grade unchanged.
  return grade;
}

// Toggle function to switch between grading systems
export const setGradeSystem = (system: string) => {
    gradeSystem.set(system);
    localStorage.setItem('gradeSystem', system);
  };
