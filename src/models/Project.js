// src/models/Project.js
import { v4 as uuidv4 } from 'uuid';
import { format } from 'date-fns';

export class Project {
  constructor({ id = uuidv4(), date_time = new Date(), image_path = 'No Image', is_sent = false, attempts = 0, grade = 'Unknown', is_active = true}) {
    this.id = id;

    // Check if date_time is a string, then convert it to a Date object
    if (typeof date_time === 'string') {
      this.date_time = new Date(date_time); // Convert string to Date
    } else {
      this.date_time = date_time instanceof Date ? date_time : new Date(date_time); // Fallback if not already a Date; // If it's already a Date, use it directly
    }

    //this.date_time = date_time;
    this.image_path = image_path;
    this.is_sent = is_sent;
    this.attempts = attempts;
    this.grade = grade;
    this.is_active = is_active;
  }

  get formattedDate_time() {
    return format(this.date_time, 'dd-MM-yyyy HH:mm:ss'); // Custom format: day-month-year hours:minutes:seconds
  }

  toMap() {
    return {
      id: this.id,
      date_time: this.date_time.getTime(), // Store as milliseconds
      image_path: this.image_path,
      is_sent: this.is_sent ? 1 : 0,
      attempts: this.attempts,
      grade: this.grade,
      is_active: this.is_active ? 1: 0
    };
  }

  /**
   * @param {{ id: any; date_time: string | number | Date; image_path: any; is_sent: number; attempts: any; grade: any; is_active: number; }} map
   */
  static fromMap(map) {
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
