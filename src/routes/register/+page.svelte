<script lang="ts">
    import { writable } from 'svelte/store';
    import { goto } from '$app/navigation';
  
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
  
      // Simulate an API request (Replace this with your real API call)
      try {
        const response = await fetch('/api/register', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ email, password })
        });
  
        if (!response.ok) {
          throw new Error('Registration failed. Try again.');
        }
  
        successMessage.set('Registration successful! Redirecting...');
        setTimeout(() => goto('/login'), 2000); // Redirect after 2 seconds
      } catch (error) {
        if (error instanceof Error) {
            errorMessage.set(error.message);
        } else {
            errorMessage.set('An unknown error occurred.');
        }
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
  