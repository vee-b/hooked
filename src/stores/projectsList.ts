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
  coordinates?: { lat: number; lng: number; note?: string[] }[];
  style?: string[];
  holds?: string[];
}

// Initialize the project list as a Svelte store.
export const projectsList: Writable<Project[]> = writable([]); // projectsList: Stores an array of Project instances.
export const annotations = writable<{ [key: string]: { x: string; y: string }[] }>({});

// Store for sends count (by grade and total)
export const sendsSummary = writable<{ total: number; byGrade: Record<string, number> }>({
  total: 0,
  byGrade: {},
});

export const stylesSummary = writable<{ style: string; done: number; practicing: number }[]>([]);
export const holdsSummary = writable<{ holds: string; done: number; practicing: number }[]>([]);

// Initialize projects list.
// 'export' Makes this function available for import in other files.
export async function initializeProjectsList(): Promise<void> {
  try {
    // Invoke Rust command to get all projects. 'result' is typed as 'unknown' because the function does not initially know what type the backend will return.
    const result: unknown = await invoke('get_all_projects');

    // Ensure the response is an array (if not an array, throw error). 
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
        coordinates: projectMap.coordinates || [],
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

// Fetch sends summary from backend
export async function fetchSendsSummary(): Promise<void> {
  try {
    const [total, gradeCounts] = await invoke<[number, [string, number][]]>('get_sends_summary');


    console.log('Backend returned data:', { total, gradeCounts }); // Debugging log

    // Ensure gradeCounts is defined and an array
    if (Array.isArray(gradeCounts)) {
      const byGrade: Record<string, number> = {};
      gradeCounts.forEach(([grade, count]) => {
        byGrade[grade] = count;
      });

      sendsSummary.set({ total, byGrade });
    } else {
      console.error('Invalid data format for gradeCounts:', gradeCounts);
    }
  } catch (error) {
    console.error('Error fetching sends summary:', error);
  }
}

export async function fetchStylesSummary() {
  try {
    const result: [string, number, number][] = await invoke('get_styles_summary');
    const summary = result.map(([style, done, practicing]) => ({ style, done, practicing }));
    stylesSummary.set(summary);
    console.log('Fetched styles summary:', summary);
  } catch (err) {
    console.error('Failed to fetch styles summary:', err);
  }
}

export async function fetchHoldsSummary() {
  try {
    const result: [string, number, number][] = await invoke('get_holds_summary');
    const summary = result.map(([holds, done, practicing]) => ({ holds, done, practicing }));
    holdsSummary.set(summary);
    console.log('Fetched holds summary:', summary);
  } catch (err) {
    console.error('Failed to fetch holds summary:', err);
  }
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
        coordinates: Array.isArray(data.coordinates)
        ? data.coordinates.map((coord) =>
            typeof coord.lat === 'number' && typeof coord.lng === 'number'
              ? coord
              : { lat: 0, lng: 0 } // Default invalid coordinates
          )
        : [],
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
        coordinates: data.coordinates || [],
      });
    });

    console.log('Processed Project IDs:', projectInstances.map((p) => p._id));
    return projectInstances;
  } catch (error) {
    console.error('Error fetching active projects:', error);
    return [];
  }
}

// Fetch the active projects with filters.
export async function fetchActiveFilteredProjects(
  grades: string[] = [],
  sentStatus: string = '',
  styles: string[] = [],
  holds: string[] = [],
): Promise<Project[]> {
  try {
    console.log('Sending to Rust:', { grades, sentStatus, styles, holds }); // Log filters

    const projectsData: unknown = await invoke('get_active_filtered_projects', {
      grades,
      sentStatus,
      styles,
      holds,
    });

    console.log('Received from Rust:', projectsData); // Log what comes back

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
        coordinates: Array.isArray(data.coordinates)
          ? data.coordinates.map((coord) =>
              typeof coord.lat === 'number' && typeof coord.lng === 'number'
                ? coord
                : { lat: 0, lng: 0 } // Default invalid coordinates
            )
          : [],
      });
    });

    console.log('Processed Project IDs:', projectInstances.map((p) => p._id));
    return projectInstances;
  } catch (error) {
    console.error('Error fetching active projects:', error);
    return [];
  }
}

// Function to fetch inactive projects with filters
export async function fetchInactiveFilteredProjects(
  grades: string[] = [],
  sentStatus: string = '',
  styles: string[] = [],
  holds: string[] = [],
): Promise<Project[]> {
  try {
    console.log('Sending to Rust:', { grades, sentStatus, styles, holds }); // Log filters

    const projectsData: unknown = await invoke('get_inactive_filtered_projects', {
      grades,
      sentStatus,
      styles,
      holds,
    });

    console.log('Received from Rust:', projectsData); // Log what comes back

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
        coordinates: Array.isArray(data.coordinates)
          ? data.coordinates.map((coord) =>
              typeof coord.lat === 'number' && typeof coord.lng === 'number'
                ? coord
                : { lat: 0, lng: 0 } // Default invalid coordinates
            )
          : [],
      });
    });

    console.log('Processed Project IDs:', projectInstances.map((p) => p._id));
    return projectInstances;
  } catch (error) {
    console.error('Error fetching inactive projects:', error);
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
      coordinates: newProject.coordinates || [],
    });

    // Convert project to map and send to Rust backend to insert
    await invoke('insert_project', { project: projectWithImage.toMap() });

    // Refresh the project list after adding the new project
    await initializeProjectsList();
  } catch (error) {
    console.error('Error adding project:', error);
  }
}

// Function to fetch all project details, including annotations and other data
export async function fetchProjectById(projectId: string): Promise<Project | null> {
  try {
    // Call the Tauri backend to get the project details
    const projectData: unknown = await invoke('get_project_by_id', { id: projectId });

    if (!projectData || typeof projectData !== 'object') {
      console.error('Unexpected response format:', projectData);
      return null;
    }

    // Assume the MongoDB response is structured to include all columns
    const data = projectData as MongoDBProject;

    // Create a new Project instance and include all project data
    const project = new Project({
      ...data,
      _id:
        typeof data._id === 'object' && data._id !== null && '$oid' in data._id
          ? data._id.$oid
          : String(data._id || ''),
    });

    // Return the full project data including annotations (coordinates)
    return project;

  } catch (error) {
    console.error('Error fetching project details:', error);
    return null;
  }
}

export async function editProject(updatedProject: Project, imageFile?: File): Promise<void> {
  try {
    console.log("Editing project, current coordinates:", updatedProject.coordinates);

    let savedImagePath: string = updatedProject.image_path;

    // If a new image is selected, upload it and get the new image URL
    if (imageFile) {
      console.log("Uploading image to Cloudinary...");
      const uploadedImageUrl = await uploadImageToCloudinary(imageFile);
      if (uploadedImageUrl) {
        savedImagePath = uploadedImageUrl;
      }
    }

     // 👇 Fetch the existing project from the backend to preserve coordinates
    const existingProject: Project = await invoke("get_project_by_id", {
      id: updatedProject._id,
    });

    console.log("Fetched existing project for coordinates:", existingProject);

    console.log("Updating project:", updatedProject);

    // Ensure correct formatting before sending to Rust
    const formattedProject = {
      _id: updatedProject._id,
      date_time: typeof updatedProject.date_time === "number" 
        ? updatedProject.date_time 
        : new Date(updatedProject.date_time).getTime(), // Convert to timestamp
      image_path: savedImagePath,
      is_sent: updatedProject.is_sent ? 1 : 0, // Convert boolean to integer
      attempts: updatedProject.attempts,
      grade: updatedProject.grade,
      is_active: updatedProject.is_active ? 1 : 0, // Convert boolean to integer
      coordinates: existingProject.coordinates,
      style: updatedProject.style,
      holds: updatedProject.holds,
    };

    console.log("Project details being sent to backend:", formattedProject);

    await invoke("update_project", { project: formattedProject });
    console.log("Project updated successfully.");

    // Refresh project list
    await initializeProjectsList();
  } catch (error) {
    console.error("Error updating project:", error);
  }
}

// Function to save annotations to the store and persist them if needed
export async function updateAnnotations(projectId: string, annotationsData: { lat: string; lng: string, note: string[] }[]): Promise<void> {
  try {
    // Convert x and y values to f64 (number type)
    const annotationsDataAsNumbers = annotationsData.map(({ lat, lng, note }) => ({
      lat: parseFloat(lat),
      lng: parseFloat(lng),
      note,
    }));

    // Send the annotations to the backend for persistence
    await invoke('save_annotations', {
      request: {  // wrap the projectId and annotationsData inside the request object
        project_id: projectId,
        annotations: annotationsDataAsNumbers,
      },
    });

    console.log(`Annotations for project ${projectId} saved successfully.`);
  } catch (error) {
    console.error('Error saving annotations:', error);
  }
}

// Add or update a project (CURRENTLY NOT IN USE)
export async function saveProject(project: Project, imageFile?: File): Promise<void> {
  try {
    let updatedImagePath = project.image_path;

    // If a new image is provided, upload it.
    if (imageFile) {
      const uploadedUrl = await uploadImageToCloudinary(imageFile);
      if (!uploadedUrl) throw new Error("Image upload failed");
      updatedImagePath = uploadedUrl;
    }

    if (project._id) {
      // Prepare project for update
      const formattedProject = {
        _id: project._id,
        date_time: typeof project.date_time === "number" ? project.date_time : new Date(project.date_time).getTime(),
        image_path: updatedImagePath,
        is_sent: project.is_sent ? 1 : 0,
        attempts: project.attempts,
        grade: project.grade,
        is_active: project.is_active ? 1 : 0,
      };
      await invoke("update_project", { project: formattedProject });
    } else {
      // Create new project with image URL
      const projectWithImage = new Project({
        ...project,
        image_path: updatedImagePath,
      });
      await invoke('insert_project', { project: projectWithImage.toMap() });
    }

    // Refresh the projects list if needed.
    await initializeProjectsList();
  } catch (error) {
    console.error("Error saving project:", error);
  }
}