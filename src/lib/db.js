// src/lib/db.js

import { invoke } from '@tauri-apps/api/core';

/**
 * Fetch all projects from the Rust backend using Tauri.
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
 * Add a new project to the database via Tauri.
 * This function calls the 'insert_project' Rust command to insert a new project into the database.
 * @param {Object} project - The project object to add.
 * @returns {Promise<void>}
 */
export async function addProject(project) {
  try {
    await invoke('insert_project', { project });
    console.log('Project added successfully');
  } catch (error) {
    console.error('Error adding project:', error);
    throw error;
  }
}

/**
 * Delete a project from the database.
 * This function invokes the 'delete_project' Rust command to delete the project by its ID.
 * @param {string} projectId - The ID of the project to delete.
 * @returns {Promise<void>}
 */
export async function deleteProject(projectId) {
  try {
    await invoke('delete_project', { id: projectId });
    console.log('Project deleted successfully');
  } catch (error) {
    console.error('Error deleting project:', error);
    throw error;
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

/**
 * Fetch active projects from the database via Tauri.
 * This function calls the Rust command 'get_active_projects'.
 * @returns {Promise<Array<any>>} - Array of active project objects.
 */
export async function getActiveProjects() {
  try {
    const result = await invoke('get_active_projects');
    return result;
  } catch (error) {
    console.error('Error fetching active projects:', error);
    throw error;
  }
}