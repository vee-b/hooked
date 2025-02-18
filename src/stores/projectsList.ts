// src/stores/projectsList.ts

import { writable, type Writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { Project } from '../models/Project';

// Define the shape of a MongoDB project object.
export interface MongoDBProject {
  _id: string | { $oid: string };
  date_time: number;
  image_path: string;
  grade: string;
  is_sent: boolean;
  attempts: number;
  is_active: boolean;
}

// Initialize the project list as a Svelte store.
export const projectsList: Writable<Project[]> = writable([]); // projectsList: Stores an array of Project instances.

// Store to hold the sends count by grade.
export const sendsCount: Writable<Record<string, number>> = writable({}); // sendsCount: Stores a record (object) mapping grades to their "send" counts.

// Initialize projects list.
export async function initializeProjectsList(): Promise<void> {
  try {
    // Invoke Rust command to get all projects.
    const result: unknown = await invoke('get_all_projects');

    // Ensures the response is an array.
    if (!Array.isArray(result)) {
      throw new Error('Unexpected response format');
    }

    // Convert MongoDB-style objects into Project instances.
    const projects: Project[] = result.map((projectMap: MongoDBProject) =>
      Project.fromMap({
        ...projectMap,
        _id:
          typeof projectMap._id === 'object' && projectMap._id !== null && '$oid' in projectMap._id
            ? projectMap._id.$oid
            : String(projectMap._id || ''),
      })
    );

    // Update the projectsList store with the fetched projects.
    projectsList.set(projects);
  } catch (error) {
    console.error('Error initializing projects list:', error);
  }
}

// Delete a project by its ID.
export async function deleteProject(_id: string): Promise<void> {
  try {
    await invoke('delete_project', { id: _id });
    // Refresh the projectsList store by re-fetching all projects.
    await initializeProjectsList();
  } catch (error) {
    console.error('Error deleting project:', error);
  }
}

// Fetch the sends count by grade.
export async function fetchSendsCount(grade: string): Promise<void> {
  try {
    let projects: Project[] = [];
    const unsubscribe = projectsList.subscribe((value) => {
      projects = value;
    });
    unsubscribe(); // Cleanup subscription

    if (projects.length > 0) {
      const count = projects.filter((project) => project.is_sent && project.grade === grade).length;

      // Update sends count.
      sendsCount.update((currentCount) => ({ ...currentCount, [grade]: count }));
    } else {
      console.error('No projects found');
    }
  } catch (error) {
    console.error('Error fetching sends count:', error);
  }
}

// Initialize the sends count.
export function initializeSendsCount(): void {
  sendsCount.set({});
}

// Fetch the active projects.
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

// Function to fetch inactive projects
export async function fetchInactiveProjects(): Promise<Project[]> {
  try {
    const projectsData: unknown = await invoke('get_inactive_projects');

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

// Sanitize file name - key change to prevent double uploads
export const sanitizeFileName = (image: any): string => {
  const timestamp = Date.now();
  const extension = image.format || 'jpg';
  const originalFileName = image.name || `image_${timestamp}`;
  const sanitizedName = originalFileName.replace(/\s+/g, "_")
                                      .replace(/[^a-zA-Z0-9._-]/g, "")
                                      .replace(/\//g, "_");
  return `${timestamp}_${sanitizedName}.${extension}`;
};

export async function uploadImageToCloudinary(imageFile: File): Promise<string | null> {
  try {
    const sanitizedFileName = imageFile.name;
    console.log("Uploading image with sanitized name:", sanitizedFileName);

    // Convert the image to binary (Uint8Array)
    const imageData = new Uint8Array(await imageFile.arrayBuffer());

    // Invoke the Rust function to upload the image
    const imagePath = await invoke<string>('upload_image', {
      imageData,
      imageName: sanitizedFileName,
    });

    console.log('Cloudinary response:', imagePath);
    console.log('Image size (bytes):', imageFile.size);

    // Ensure the upload was successful and return the Cloudinary URL
    if (!imagePath || typeof imagePath !== 'string') {
      throw new Error('Image upload failed or invalid response.');
    }

    return imagePath; // Return Cloudinary image URL
  } catch (error) {
    console.error('Error uploading image:', error);
    return null;
  }
}

// Function to add a new project with image upload to Cloudinary
export async function addProject(newProject: Project, imageFile: File): Promise<void> {
  try {
    // Upload the image to Cloudinary
    const imagePath = await uploadImageToCloudinary(imageFile);
    if (!imagePath) throw new Error('Image upload failed.');

    // Create a new Project instance with the Cloudinary URL for the image
    const projectWithImage = new Project({
      ...newProject,
      image_path: imagePath, // Store the Cloudinary URL here
    });

    // Convert project to map and send to Rust backend to insert
    await invoke('insert_project', { project: projectWithImage.toMap() });

    // Refresh the project list after adding the new project
    await initializeProjectsList();
  } catch (error) {
    console.error('Error adding project:', error);
  }
}