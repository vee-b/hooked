// src/controllers/accountsController.svelte

import { invoke } from '@tauri-apps/api/core';
import { goto } from '$app/navigation';

/**
 * Registers a new account by invoking the Rust backend command.
 * @param email - The user's email.
 * @param password - The user's password.
 * @returns A promise that resolves with the new account's ID as a string.
 */
export async function registerAccount(email: string, password: string): Promise<string> {
  try {
    // Invoke the Tauri command 'create_account' (which should be registered in main.rs)
    const accountId = await invoke<string>('create_account', { email, password });
    return accountId;
  } catch (error) {
    console.error("Error registering account:", error);
    throw error;
  }
}

/**
 * Logs in an account by invoking the Rust backend command.
 * @param email - The user's email.
 * @param password - The user's password.
 * @returns A promise that resolves with a JWT token as a string.
 */
export async function loginAccount(email: string, password: string): Promise<[string, string]> {
  try {
    // Invoke the Tauri command 'login' (registered in main.rs)
    const token = await invoke<[string, string]>('login', { email, password });
    return token;
  } catch (error) {
    console.error("Error logging in:", error);
    throw error;
  }
}

// Check login status once
export const checkLoginStatus = (): boolean => {
  const token = localStorage.getItem('token'); // Retrieve JWT token
  if (!token) {
    goto('/login'); // Redirect to login if not logged in
    return false; // No token found, return false
  }
  return true; // Token exists, return true
}

/**
 * Logs out the user by removing the token and redirecting to login.
 */
export function logoutAccount() {
  localStorage.removeItem('token'); // Remove token from localStorage
  goto('/login'); // Redirect to login page
}