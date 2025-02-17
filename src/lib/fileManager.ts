// // src/lib/fileManager.ts

// import { writeBinaryFile, readBinaryFile, BaseDirectory } from '@tauri-apps/api/fs';

// let fs: any;

// if (import.meta.env.MODE !== 'production') {
//   // Dynamically import the Tauri API only in non-production environments
//   fs = await import('@tauri-apps/api/fs');
//   await fs.init();
// }

// // Save image to the local file system
// export async function saveImageToFileSystem(imageBlob: Blob, imageName: string): Promise<string> {
//   try {
//     if (fs) {
//       //await fs.init();
//       const arrayBuffer = await imageBlob.arrayBuffer();
//       const imageBytes = new Uint8Array(arrayBuffer);

//       const imagePath = `images/${imageName}`;

//       // Save to the AppData directory
//       await writeBinaryFile(imagePath, imageBytes, { dir: BaseDirectory.AppData });

//       return imagePath; // Return relative path for storage in MongoDB
//     }
//   } catch (error) {
//     console.error('Error saving image:', error);
//     throw error;
//   }
// }

// // Load image from the local file system
// export async function loadImageFromFileSystem(imagePath: string): Promise<string | null> {
//   try {  
//     if (fs) {
//       //await fs.init();
//       const imageBytes = await readBinaryFile(imagePath, { dir: BaseDirectory.AppData });
//       return URL.createObjectURL(new Blob([imageBytes]));
//     }
//   } catch (error) {
//     console.error('Error loading image:', error);
//     return null;
//   }
// }
