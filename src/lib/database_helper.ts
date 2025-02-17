// // For Capacitor SQLite DB

// import { CapacitorSQLite, SQLiteConnection } from '@capacitor-community/sqlite';
// import type { SQLiteDBConnection } from '@capacitor-community/sqlite';

// export default class DatabaseHelper {
//   private db: SQLiteDBConnection | null = null;
//   private dbName: string = 'project.db';

//   // Open or create the database
//   async initializeDatabase(): Promise<void> {
//     const sqliteConnection = new SQLiteConnection(CapacitorSQLite);

//     // Open the connection to the database
//     this.db = await sqliteConnection.createConnection(
//       this.dbName,        // database name
//       false,              // encrypted
//       'no-encryption',    // mode
//       1,                  // version
//       false               // readonly
//     );

//     if (!this.db) {
//       throw new Error('Failed to create database connection');
//     }

//     await this.db.open();
//     await this.db.execute(`
//       CREATE TABLE IF NOT EXISTS projectTable (
//         id TEXT PRIMARY KEY,
//         date_time INTEGER,
//         image_path TEXT,
//         is_sent INTEGER,
//         attempts INTEGER,
//         grade TEXT,
//         is_active INTEGER NOT NULL DEFAULT 1
//       )
//     `);
//   }

//   // Insert a project into the database
//   async insertProject(project: {
//     id: string;
//     date_time: number;
//     image_path: string;
//     is_sent: number;
//     attempts: number;
//     grade: string;
//     is_active: number;
//   }): Promise<void> {
//     if (!this.db) {
//       throw new Error('Database not initialized');
//     }

//     const query = `
//       INSERT INTO projectTable (id, date_time, image_path, is_sent, attempts, grade, is_active)
//       VALUES (?, ?, ?, ?, ?, ?, ?)
//     `;
//     await this.db.run(query, [
//       project.id,
//       project.date_time,
//       project.image_path,
//       project.is_sent,
//       project.attempts,
//       project.grade,
//       project.is_active,
//     ]);
//   }

//   // Fetch all projects from the database
//   async getAllProjects(): Promise<any[]> {
//     if (!this.db) {
//       throw new Error('Database not initialized');
//     }

//     const result = await this.db.query(`
//       SELECT * FROM projectTable ORDER BY date_time ASC
//     `);

//     return result.values || [];
//   }

//   // Update a project in the database
//   async updateProject(project: {
//     id: string;
//     date_time: number;
//     image_path: string;
//     is_sent: number;
//     attempts: number;
//     grade: string;
//     is_active: number;
//   }): Promise<void> {
//     if (!this.db) {
//       throw new Error('Database not initialized');
//     }

//     const query = `
//       UPDATE projectTable
//       SET date_time = ?, image_path = ?, is_sent = ?, attempts = ?, grade = ?, is_active = ?
//       WHERE id = ?
//     `;
//     await this.db.run(query, [
//       project.date_time,
//       project.image_path,
//       project.is_sent,
//       project.attempts,
//       project.grade,
//       project.is_active,
//       project.id,
//     ]);
//   }

//   // Delete a project from the database
//   async deleteProject(id: string): Promise<void> {
//     if (!this.db) {
//       throw new Error('Database not initialized');
//     }

//     await this.db.run(`
//       DELETE FROM projectTable WHERE id = ?
//     `, [id]);
//   }

//   // Get the count of projects in the database
//   async getSendsCount(): Promise<number> {
//     if (!this.db) {
//       throw new Error('Database not initialized');
//     }

//     const result = await this.db.query(`
//       SELECT COUNT(*) as count FROM projectTable
//     `);
//     return result.values?.[0]?.count || 0;
//   }

//   async getActiveProjects(): Promise<any[]> {
//     if (!this.db) {
//       throw new Error('Database not initialized');
//     }

//     const result = await this.db.query(`
//       SELECT * FROM projectTable WHERE is_active = 1
//     `);
//     return result.values || [];
//   }
// }