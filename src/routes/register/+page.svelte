<script lang="ts">
    import { writable } from 'svelte/store';
    import { goto } from '$app/navigation';
    import { registerAccount } from '../../controllers/accountsController';

    let email = '';
    let password = '';
    let confirmPassword = '';
    let errorMessage = writable('');
    let successMessage = writable('');
  
    const register = async () => {
      errorMessage.set('');
      successMessage.set('');
  
      if (!email || !password || !confirmPassword) {
        errorMessage.set('Please fill in all fields.');
        return;
      }
  
      if (password !== confirmPassword) {
        errorMessage.set('Passwords do not match.');
        return;
      }
  
      try {
        // Directly invoke the Rust function via Tauri
        //const userId = await invoke('create_account', { email, password });

        const accountId = await registerAccount(email, password);
        successMessage.set('Registration successful! Redirecting...');
        setTimeout(() => goto('/login'), 2000); // Redirect after 2 seconds
      } catch (error) {
        console.error('Registration error:', error);
        errorMessage.set('Registration failed. Please try again.');
      }
    };
  </script>
  
  <style>
    .register-container {
      max-width: 400px;
      margin: 50px auto;
      padding: 20px;
      background-color: #f9f9f9;
      border-radius: 8px;
      box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    }
  
    .register-form {
      display: flex;
      flex-direction: column;
    }
  
    .input-field {
      margin-bottom: 1rem;
      padding: 0.75rem;
      font-size: 1rem;
      border: 1px solid #ccc;
      border-radius: 5px;
    }
  
    .input-field:focus {
      border-color: #007bff;
      outline: none;
    }
  
    .error-message {
      color: red;
      margin-bottom: 1rem;
    }
  
    .success-message {
      color: green;
      margin-bottom: 1rem;
    }
  
    .submit-button {
      padding: 1rem;
      background-color: #007bff;
      color: white;
      border: none;
      border-radius: 5px;
      cursor: pointer;
      font-size: 1rem;
    }
  
    .submit-button:hover {
      background-color: #0056b3;
    }
  </style>
  
  <div class="register-container">
    <h2>Register</h2>
  
    {#if $errorMessage}
      <div class="error-message">{$errorMessage}</div>
    {/if}
  
    {#if $successMessage}
      <div class="success-message">{$successMessage}</div>
    {/if}
  
    <form class="register-form" on:submit|preventDefault={register}>
      <input 
        type="email" 
        class="input-field" 
        placeholder="Email" 
        bind:value={email} 
        required
      />
      
      <input 
        type="password" 
        class="input-field" 
        placeholder="Password" 
        bind:value={password} 
        required
      />
      
      <input 
        type="password" 
        class="input-field" 
        placeholder="Confirm Password" 
        bind:value={confirmPassword} 
        required
      />
      
      <button type="submit" class="submit-button">Register</button>
    </form>
  </div>
  