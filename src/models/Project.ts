// src/models/Project.ts

import { format } from 'date-fns'; // library to format JS dates

// Definition of the Project class
export class Project {
  _id?: string; // Optional, since MongoDB generates _id if not provided
  date_time: Date; // when the project was created / logged
  image_path: string; // path or URL to the project image
  is_sent: boolean; // true if the climb was completed
  sent_date: Date; // when it was sent (if ever)
  attempts: number; // how many tries
  grade: string; // e.g. V5
  is_active: boolean; // for filtering
  coordinates: { lat: number; lng: number; note?: string[] }[]; // annotation points on image
  style: string[]; // styles tags (e.g. "slab", "overhang")
  holds: string[]; // holds tags (e.g. "pinch", "jug")

  // The constructor
  // accepts either defaults or passed values
  // also auto-converts dates from string/number
  constructor({ 
    _id,
    date_time = new Date(), 
    image_path = 'No Image', 
    is_sent = false, 
    sent_date = new Date(0),
    attempts = 0, 
    grade = 'Unknown', 
    is_active = true, 
    coordinates = [],
    style = [],
    holds = [],
  } : {
    _id?: string;
    date_time?: Date | string | number;
    image_path?: string;
    is_sent?: boolean;
    sent_date?: Date | string | number;
    attempts?: number;
    grade?: string;
    is_active?: boolean;
    coordinates?: { lat: number; lng: number }[];
    style?: string[];
    holds?: string[];
  }) {
    this._id = _id;

    // Convert to Date object if it's a string or timestamp
    this.date_time = typeof date_time === 'string' || typeof date_time === 'number'
      ? new Date(date_time)
      : date_time;

    this.image_path = image_path;
    this.is_sent = is_sent;

    // Support sent_date as timestamp (seconds or ms)
    this.sent_date = typeof sent_date === 'string' || typeof sent_date === 'number'
      ? new Date(Number(sent_date) < 10_000_000_000 ? Number(sent_date) * 1000 : Number(sent_date))
      : sent_date;

    this.attempts = attempts;
    this.grade = grade;
    this.is_active = is_active;
    this.coordinates = coordinates;
    this.style = style;
    this.holds = holds; 
  }

  // Getter: returns formatted date & time
  // e.g. "13-07-2025 14:55:00"
  get formatted_date_time(): string {
    return this.date_time ? format(this.date_time, 'dd-MM-yyyy HH:mm:ss') : 'Invalid Date';
  }

  // Getter: returns formatted sent date
  // or "Not Sent" if no valid sent_date
  get formatted_sent_date(): string {
    return this.sent_date && this.sent_date.getTime() > 0
      ? format(this.sent_date, 'dd-MM-yyyy HH:mm:ss')
      : 'Not Sent';
  }

  // Convert the object into a plain map
  // suitable for sending to MongoDB
  // (turns Dates into timestamps etc.)
  toMap() {
    return {
      ...(this._id ? { _id: this._id } : {}),
      
      account_id: localStorage.getItem("account_id"),  // or wherever you store the user id

      date_time: this.date_time.getTime(),  // Store as milliseconds (or use ISODate in MongoDB)
      image_path: this.image_path,
      is_sent: this.is_sent ? 1 : 0,
      sent_date: Math.floor(this.sent_date.getTime() / 1000),
      attempts: this.attempts,
      grade: this.grade,
      is_active: this.is_active ? 1 : 0,
      coordinates: this.coordinates,
      style: this.style,
      holds: this.holds,
    };
  }

  // Static helper to reconstruct a Project
  // from MongoDB document (map/object)
  static fromMap(map: any): Project {
    return new Project({
      _id: map._id ? String(map._id) : undefined,
      date_time: new Date(map.date_time),
      image_path: map.image_path,
      is_sent: map.is_sent === 1,
      sent_date: map.sent_date,
      attempts: map.attempts,
      grade: map.grade,
      is_active: map.is_active === 1,
      coordinates: map.coordinates || [],
      style: map.style || [],
      holds: map.holds || [],
    });
  }
}