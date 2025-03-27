<script lang="ts">
    import { writable } from 'svelte/store';
    import { goto } from '$app/navigation';
  
    let email = '';
    let password = '';
    let errorMessage = writable('');
  
    // Simulate a login API request (you can replace this with your real API call)
    const login = async () => {
      if (!email || !password) {
        errorMessage.set('Please fill in both fields.');
        return;
      }
  
      // Simulate API request here (e.g., fetch to your backend)
      const user = { email, password }; // Replace with real API call
      if (user.email === 'test@example.com' && user.password === 'password') {
        goto('/dashboard'); // Redirect on success
      } else {
        errorMessage.set('Invalid credentials, please try again.');
      }
    };
  </script>
  
  <style>
    .login-container {
      max-width: 400px;
      margin: 50px auto;
      padding: 20px;
      background-color: #f9f9f9;
      border-radius: 8px;
      box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
      text-align: center;
    }
  
    .login-form {
      display: flex;
      flex-direction: column;
    }
  
    .input-field {
      margin-bottom: 1rem;
      padding: 0.75rem;
      font-size: 1rem;
      border: 1px solid #ccc;
      border-radius: 5px;
      width: 100%;
    }
  
    .input-field:focus {
      border-color: #007bff;
      outline: none;
    }
  
    .error-message {
      color: red;
      margin-bottom: 1rem;
    }
  
    .button-container {
      display: flex;
      flex-direction: column;
      gap: 0.75rem;
      margin-top: 1rem;
    }
  
    .submit-button,
    .register-button {
      padding: 1rem;
      border: none;
      border-radius: 5px;
      cursor: pointer;
      font-size: 1rem;
      width: 100%;
    }
  
    .submit-button {
      background-color: #007bff;
      color: white;
    }
  
    .submit-button:hover {
      background-color: #0056b3;
    }
  
    .register-button {
      background-color: #6c757d;
      color: white;
    }
  
    .register-button:hover {
      background-color: #5a6268;
    }
  </style>
  
  <div class="login-container">
    <h2>Login</h2>
  
    <!-- Conditional rendering for error message -->
    {#if $errorMessage}
      <div class="error-message">{$errorMessage}</div>
    {/if}
  
    <form class="login-form" on:submit|preventDefault={login}>
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
  
      <div class="button-container">
        <button type="submit" class="submit-button">Login</button>
        <button type="button" class="register-button" on:click={() => goto('/register')}>Register</button>
      </div>
    </form>
  </div>
  