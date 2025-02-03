// src/mockData.js

import { Project } from './models/Project';

export const mockProjects = [
    new Project({
        id: "1",
        dateTime: new Date("2024-10-13T10:00:00Z"),
        imagePath: "path/to/image1.jpg",
        grade: "V2",
        isSent: true,
        attempts: 3, // Adjusted to match the expected type (number)
    }),
    new Project({
        id: "2",
        dateTime: new Date("2024-10-12T14:30:00Z"),
        imagePath: "path/to/image2.jpg",
        grade: "V0",
        isSent: false,
        attempts: 1,
    }),
    new Project({
        id: "3",
        dateTime: new Date("2024-10-11T11:45:00Z"),
        imagePath: "path/to/image3.jpg",
        grade: "V1",
        isSent: true,
        attempts: 2,
    }),
    // Add more dummy projects as needed
];