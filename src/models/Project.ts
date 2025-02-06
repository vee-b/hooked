// Project.ts

import { v4 as uuidv4 } from 'uuid';
import { format } from 'date-fns';
import { invoke } from '@tauri-apps/api/core';

// Define the Project class
export class Project {
  id: string;
  date_time: Date;
  image_path: string;
  is_sent: boolean;
  attempts: number;
  grade: string;
  is_active: boolean;

  constructor({ 
    id = uuidv4(), 
    date_time = new Date(), 
    image_path = 'No Image', 
    is_sent = false, 
    attempts = 0, 
    grade = 'Unknown', 
    is_active = true 
  }) {
    this.id = id;
    this.date_time = typeof date_time === 'string' || typeof date_time === 'number'
      ? new Date(date_time)
      : date_time;
    this.image_path = image_path;
    this.is_sent = is_sent;
    this.attempts = attempts;
    this.grade = grade;
    this.is_active = is_active;
  }

  // Format the date
  get formatted_date_time(): string {
    return this.date_time ? format(this.date_time, 'dd-MM-yyyy HH:mm:ss') : 'Invalid Date';
  }

  // Convert the object to a map (for storage, etc.)
  toMap() {
    return {
      id: this.id,
      date_time: this.date_time.getTime(), // Store as milliseconds
      image_path: this.image_path,
      is_sent: this.is_sent ? 1 : 0,
      attempts: this.attempts,
      grade: this.grade,
      is_active: this.is_active ? 1 : 0,
    };
  }

  // Static method to create a Project from a map (e.g., fetched from a database)
  static fromMap(map: any): Project {
    if (!map.date_time) {
      console.warn("Missing or invalid date_time in project data", map);
    } else {
      console.warn("date_time is valid");
    }

    return new Project({
      id: map.id,
      date_time: new Date(map.date_time),
      image_path: map.image_path,
      is_sent: map.is_sent === 1,
      attempts: map.attempts,
      grade: map.grade,
      is_active: map.is_active === 1,
    });
  }
}

// Function to fetch active projects
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

/*
// Project.d.ts
declare module "../models/Project" {  // The string here should match the import path used in your .svelte files
    export type Project = {
        id: string;
        date_time: Date;
        image_path: string | null;
        is_sent: boolean;
        attempts: number;
        grade: string;
        is_active: boolean;
        formattedDateTime: string;
    };

    export class ProjectClass {
        constructor(options: {
            id?: string;
            date_time?: Date | string | number;
            image_path?: string | null;
            is_sent?: boolean;
            attempts?: number;
            grade?: string;
            is_active?: boolean;
        });  // No return type on the constructor

        formattedDateTime: string;
        toMap(): {
            id: string;
            date_time: number;
            image_path: string | null;
            is_sent: number;
            attempts: number;
            grade: string;
            is_active: number;
        };
        static fromMap(map: any): ProjectClass;
    }

    export function getActiveProjects(): Promise<ProjectClass[]>;
}
*/