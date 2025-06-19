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
  
  <!-- <style>
    .register-container {
      max-width: 450px;
      margin: 100px auto;
      padding-left: 40px;
      padding-right: 40px;
      margin-top: 40px;
      border-radius: 10px;
      /* box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1); */
      text-align: center;
      box-sizing: border-box; /* Ensure padding doesn't affect width */
    }

    h2 {
      font-size: 2rem;
      color: white;
      margin-bottom: 20px;
      font-weight: bold;
      -webkit-text-stroke: 1px #00000048;
    }
  
    .register-form {
      display: flex;
      flex-direction: column;
    }
  
    .input-field {
      margin-bottom: 1rem;
      padding: 1rem;
      font-size: 1rem;
      border: 2px solid #444;
      border-radius: 10px;
      width: 100%;
      max-width: 100%; /* Ensures full width within the container */
      box-sizing: border-box; /* Prevents padding from affecting width */
      background-color: #333; /* Dark input background */
      color: #f5f5f5; /* Light text inside the input fields */
      transition: border-color 0.3s ease, box-shadow 0.3s ease;
    }
  
    .input-field:focus {
      border-color: #00ff8092;
      box-shadow: 0 0 5px rgba(0, 123, 255, 0.5);
      outline: none;
    }
  
    .error-message {
      color: red;
      font-size: 0.875rem;
      margin-bottom: 10px;
      animation: fadeIn 0.5s ease;
    }
  
    .button-container {
      display: flex;
      flex-direction: column;
      gap: 1rem;
      margin-top: 1rem;
    }
  
    .submit-button,
    .login-button {
      padding: 1rem;
      border: none;
      border-radius: 10px;
      cursor: pointer;
      font-size: 1rem;
      width: 100%;
      transition: background-color 0.3s ease, transform 0.3s ease;
    }
  
    .submit-button {
      border: 2px solid #00ff8092; /* Thin green outline */
      background-color: transparent; /* No background color */
    }
  
    .submit-button:hover {
      background-color: #00ff8000;
      transform: scale(1.05);
    }
  
    .login-button {
      border: 2px solid #ff8000b9; /* Thin green outline */
      background-color: transparent; /* No background color */
      color: white;
    }
  
    .login-button:hover {
      background-color: #00ff8000;
      transform: scale(1.05);
    }

    /* Animation for error message */
    @keyframes fadeIn {
      0% {
        opacity: 0;
      }
      100% {
        opacity: 1;
      }
    }

    .logo {
      width: 100%;
      max-width: 200px;
      margin-bottom: 20px;
      border-radius: 10px;
    }
  </style> -->

  <style global>
    body {
      /* background-color: #e6f4fd; */
      font-family: 'Roboto', sans-serif;
      margin: 0;
      padding: 0;
    }

    h2 {
      color: rgb(57, 57, 57);
      font-size: 1.5rem;
      font-weight: lighter;
      margin-bottom: 20px;
    }

    .register-container {
      max-width: 450px;
      margin: 100px auto;
      padding-left: 40px;
      padding-right: 40px;
      margin-top: 40px;
      background: #f3f9f9fa;
      border-radius: 20px;
      text-align: center;
      transition: all 0.3s ease;
      box-sizing: border-box; /* Ensure padding doesn't affect width */
    }

    .input-field {
      margin-bottom: 1rem;
      padding: 1rem;
      border-radius: 10px;
      border: none;
      background: #e6f4fd;
      box-shadow: inset 5px 5px 10px #b4d1e3, inset -5px -5px 10px #ffffff;
      color: #333;
      font-size: 1rem;
      outline: none;
      width: 100%;
      max-width: 100%; /* Ensures full width within the container */
      box-sizing: border-box; /* Prevents padding from affecting width */
    }

    .input-field:focus {
      box-shadow: inset 3px 3px 6px #b4d1e3, inset -3px -3px 6px #ffffff;
    }

    .button-container {
      display: flex;
      flex-direction: column;
      gap: 1rem;
      margin-top: 1rem;
    }

    .submit-button,
    .login-button {
      padding: 1rem;
      border: none;
      width: 100%;
      border-radius: 10px;
      font-size: 1rem;
      cursor: pointer;
      background: #e6f4fd;
      box-shadow: 5px 5px 10px #b4d1e3, -5px -5px 10px #ffffff;
      transition: all 0.3s ease;
      color: rgb(57, 57, 57);
    }

    .submit-button:hover,
    .login-button:hover {
      box-shadow: inset 3px 3px 6px #b4d1e3, inset -3px -3px 6px #ffffff;
    }

    .error-message {
      color: red;
      margin-bottom: 10px;
    }

    .logo {
      width: 100%;
      max-width: 400px;
      opacity: 0.85;
      margin-bottom: 20px;
      border-radius: 10px;
    }
  </style>
  
  <div class="register-container">
    <h2>HOOKED</h2>

    <!-- Logo Image -->
    <img src="/images/logo.png" alt="Logo" class="logo" />
  
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
      
      <!-- <button type="submit" class="submit-button">Register</button> -->
      <div class="button-container">
        <button type="submit" class="submit-button">Sign Up</button>
        <button type="button" class="login-button" on:click={() => goto('/login')}>Login</button>
      </div>
    </form>
  </div>
  