// src/stores/projectsList.js

import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { getActiveProjects, Project } from '../models/Project';

// /**
//  * @type {import('svelte/store').Writable<Project[]>}
//  */
// Initialize the project list as a Svelte store
export const projectsList = writable([]);

/**
 * @type {import('svelte/store').Writable<{ [key: string]: number }>}
/**
 * Store to hold the sends count by grade
 */
export const sendsCount = writable({}); // Initialize as an empty object

// Function to initialize projects from the database
export async function initializeProjectsList() {
  try {
    // Invoke the Rust command to get all projects from the database
    const result = await invoke('get_all_projects');
    // Convert the result to a list of Project objects
    const projects = result.map((/** @type {{ id: any; date_time: string | number | Date; image_path: any; is_sent: number; attempts: any; grade: any; is_active: any; }} */ projectMap) => Project.fromMap(projectMap));
    // Set the projects list in the Svelte store
    projectsList.set(projects);
  } catch (error) {
    console.error('Error initializing projects list:', error);
  }
}

// Function to add a new project
/**
 * @param {Project} newProject
 */
export async function addProject(newProject) {
  try {
    // Convert the new project to a map and invoke the Rust command
    await invoke('insert_project', { project: newProject.toMap() });
    
    // Reinitialize the projects list after adding the new project
    initializeProjectsList();
  } catch (error) {
    console.error('Error adding project:', error);
  }
}

// Function to delete a project by its ID
/**
 * @param {any} id
 */
export async function deleteProject(id) {
  try {
    // Invoke the Rust command to delete the project from the database
    await invoke('delete_project', { id });
    // Reinitialize the projects list after deleting the project
    initializeProjectsList();
  } catch (error) {
    console.error('Error deleting project:', error);
  }
}

// Function to fetch the sends count by grade
/**
 * @param {any} grade
 */
export async function fetchSendsCount(grade) {
  try {
    /**
     * @type {Project[]}
     */
    let projects = [];

    const unsubscribe = projectsList.subscribe(value => {
      projects = value;
    });

    // Cleanup the subscription
    unsubscribe();

    if (projects) {
      const count = projects.filter(project => project.is_sent && project.grade === grade).length;

      // Update sends count
      sendsCount.update(currentCount => {
        return { ...currentCount, [grade]: count };
      });
    } else {
      console.error('No projects found');
    }
  } catch (error) {
    console.error('Error fetching sends count:', error);
  }
}

// Function to initialize the sends count
export function initializeSendsCount() {
  sendsCount.set({});
}

// Function to fetch only active projects
// export async function getActiveProjects() {
//   try {
//     const result = await invoke('get_all_projects');
//     const projects = result.map((/** @type {{ id: any; date_time: string | number | Date; image_path: any; is_sent: number; attempts: any; grade: any; is_active: number; }} */ projectMap) => Project.fromMap(projectMap));
    
//     // Filter active projects
//     const activeProjects = projects.filter((/** @type {{ is_active: any; }} */ project) => project.is_active);
    
//     return activeProjects;
//   } catch (error) {
//     console.error('Error fetching active projects:', error);
//     return []; // Return an empty array on error
//   }
// }
export async function fetchActiveProjects() {
  try {
    const projectsData = await getActiveProjects(); // Should return an array
    if (Array.isArray(projectsData)) {
      const projectInstances = projectsData.map(data => new Project(data))
      return projectInstances;
    }
    console.error('Unexpected response format:', projectsData);
    return [];
  } catch (error) {
    console.error('Error fetching active projects:', error);
    return [];
  }
}