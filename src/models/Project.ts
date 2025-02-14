// src/models/Project.ts

import { format } from 'date-fns';
import { invoke } from '@tauri-apps/api/core';

// Define the Project class
export class Project {
  _id?: string; // MongoDB uses _id for the document identifier
  date_time: Date;
  image_path: string;
  is_sent: boolean;
  attempts: number;
  grade: string;
  is_active: boolean;

  constructor({ 
    _id,
    date_time = new Date(), 
    image_path = 'No Image', 
    is_sent = false, 
    attempts = 0, 
    grade = 'Unknown', 
    is_active = true 
  } : {
    _id?: string;
    date_time?: Date | string | number;
    image_path?: string;
    is_sent?: boolean;
    attempts?: number;
    grade?: string;
    is_active?: boolean;
  }) {
    this._id = _id;
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

  // Convert the object to a format MongoDB expects
  toMap() {
    return {
      ...(this._id ? { _id: this._id } : {}),
      date_time: this.date_time.getTime(),  // Store as milliseconds (or use ISODate in MongoDB)
      image_path: this.image_path,
      is_sent: this.is_sent ? 1 : 0,
      attempts: this.attempts,
      grade: this.grade,
      is_active: this.is_active ? 1 : 0,
    };
  }

  // Static method to create a Project from MongoDB document format
  static fromMap(map: any): Project {
    // MongoDB may return _id as an ObjectId or string, so we handle that properly
    //const _id = map._id ? map._id.toString() : undefined;

    return new Project({
      _id: map._id ? String(map._id) : undefined,
      date_time: new Date(map.date_time),
      image_path: map.image_path,
      is_sent: map.is_sent === 1,
      attempts: map.attempts,
      grade: map.grade,
      is_active: map.is_active === 1,
    });
  }
}

// Function to fetch active projects from MongoDB
// export async function getActiveProjects(): Promise<Project[]> {
//   try {
//     const projectsData = await invoke('get_active_projects');
//     if (Array.isArray(projectsData)) {
//       console.log("Raw projects data:", projectsData);
//       return projectsData.map(Project.fromMap);
//     }
//     console.error('Data format unexpected:', projectsData);
//     return [];
//   } catch (error) {
//     console.error('Error fetching active projects:', error);
//     return [];
//   }
// }