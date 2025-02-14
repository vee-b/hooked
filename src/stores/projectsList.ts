// src/stores/projectsList.ts

import { writable, type Writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { Project } from '../models/Project';

/**
 * Type definition for a MongoDB Project.
 */
export interface MongoDBProject {
  _id: string | { $oid: string };
  date_time: number;
  image_path: string;
  grade: string;
  is_sent: boolean;
  attempts: number;
  is_active: boolean;
}

// Initialize the project list as a Svelte store
export const projectsList: Writable<Project[]> = writable([]);

// Store to hold the sends count by grade
export const sendsCount: Writable<Record<string, number>> = writable({});

// Function to initialize projects from the database
export async function initializeProjectsList(): Promise<void> {
  try {
    // Invoke Rust command to get all projects
    const result: unknown = await invoke('get_all_projects');

    if (!Array.isArray(result)) {
      throw new Error('Unexpected response format');
    }

    const projects: Project[] = result.map((projectMap: MongoDBProject) =>
      Project.fromMap({
        ...projectMap,
        _id:
          typeof projectMap._id === 'object' && projectMap._id !== null && '$oid' in projectMap._id
            ? projectMap._id.$oid
            : String(projectMap._id || ''),
      })
    );

    // Set the projects list in the Svelte store
    projectsList.set(projects);
  } catch (error) {
    console.error('Error initializing projects list:', error);
  }
}

// Function to add a new project
export async function addProject(newProject: Project): Promise<void> {
  try {
    // Convert project to map and send to Rust backend
    await invoke('insert_project', { project: newProject.toMap() });

    // Refresh project list
    await initializeProjectsList();
  } catch (error) {
    console.error('Error adding project:', error);
  }
}

// Function to fetch active projects from MongoDB
export async function getActiveProjects(): Promise<Project[]> {
  try {
    const projectsData = await invoke('get_active_projects');
    if (Array.isArray(projectsData)) {
      console.log("Raw projects data:", projectsData);
      return projectsData.map(Project.fromMap);
    }
    console.error('Data format unexpected:', projectsData);
    return [];
  } catch (error) {
    console.error('Error fetching active projects:', error);
    return [];
  }
}

// Function to delete a project by its ID
export async function deleteProject(_id: string): Promise<void> {
  try {
    await invoke('delete_project', { id: _id });
    await initializeProjectsList();
  } catch (error) {
    console.error('Error deleting project:', error);
  }
}

// Function to fetch the sends count by grade
export async function fetchSendsCount(grade: string): Promise<void> {
  try {
    let projects: Project[] = [];
    const unsubscribe = projectsList.subscribe((value) => {
      projects = value;
    });
    unsubscribe(); // Cleanup subscription

    if (projects.length > 0) {
      const count = projects.filter((project) => project.is_sent && project.grade === grade).length;

      // Update sends count store
      sendsCount.update((currentCount) => ({ ...currentCount, [grade]: count }));
    } else {
      console.error('No projects found');
    }
  } catch (error) {
    console.error('Error fetching sends count:', error);
  }
}

// Function to initialize the sends count store
export function initializeSendsCount(): void {
  sendsCount.set({});
}

// Function to fetch active projects
export async function fetchActiveProjects(): Promise<Project[]> {
  try {
    const projectsData: unknown = await invoke('get_active_projects');

    if (!Array.isArray(projectsData)) {
      console.error('Unexpected response format:', projectsData);
      return [];
    }

    const typedProjectsData: MongoDBProject[] = projectsData;

    const projectInstances: Project[] = typedProjectsData.map((data) => {
      return new Project({
        ...data,
        _id:
          typeof data._id === 'object' && data._id !== null && '$oid' in data._id
            ? data._id.$oid
            : String(data._id || ''),
      });
    });

    console.log('Processed Project IDs:', projectInstances.map((p) => p._id));
    return projectInstances;
  } catch (error) {
    console.error('Error fetching active projects:', error);
    return [];
  }
}
