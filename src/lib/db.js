// src/lib/db.js

import { invoke } from '@tauri-apps/api/core';

/**
 * Fetch all projects from the MongoDB backend using Tauri.
 * This function calls the 'get_all_projects' command in the Rust backend.
 * It returns an array of project objects.
 */
export async function getAllProjects() {
  try {
    const result = await invoke('get_all_projects');
    return result; // The result should be the list of projects
  } catch (error) {
    console.error('Error fetching projects:', error);
    throw error; // Rethrow for handling at the calling side
  }
}

/**
 * Fetch sends count by grade from the Rust backend.
 * This function calls the 'fetch_sends_count' command to get the number of sends for a specific grade.
 * @param {string} grade - The grade to count sends for.
 * @returns {Promise<number>} - The number of sends for the given grade.
 */
export async function getSendsCount(grade) {
  try {
    const result = await invoke('get_sends_count', { grade });
    return result; // The result should be the sends count for the given grade
  } catch (error) {
    console.error('Error fetching sends count:', error);
    throw error;
  }
}