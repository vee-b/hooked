<script lang="ts">
  // Svelte store to hold reactive error messages
  import { writable } from 'svelte/store';

  // For navigating programmatically (SvelteKit)
  import { goto } from '$app/navigation';

  // Your backend login function that will talk to Tauri Rust
  import { loginAccount } from '../../controllers/accountsController';

  // Icons from lucide-svelte
  import { LogIn, ArrowRight } from 'lucide-svelte'; 

  // LOCAL STATE 
  let email = ''; // Holds user email input
  let password = ''; // Holds user password input
  let errorMessage = writable(''); // Reactive error message

  // LOGIN FUNCTION 
  const login = async () => {
    // Reset previous errors
    errorMessage.set('');

    // Simple form validation
    if (!email || !password) {
      errorMessage.set('Please fill in both fields.');
      return;
    }

    try {
      // Try to login, e.g. via a Tauri IPC command
      const token = await loginAccount(email, password);
      console.log('Login successful, received token:', token);

      // Save token to localStorage
      localStorage.setItem('token', token);

      // Redirect to home/dashboard
      goto('/');
    } catch (error) {
      console.error('Login error:', error);
      errorMessage.set('Invalid credentials, please try again.');
    }
  };
</script>
  
<style global>
  body {
    background-color: #f3f9f9fa;
    font-family: 'Poppins', sans-serif;
    margin: 0;
    padding: 0;
  }

  h2 {
    color: rgb(57, 57, 57);
    font-size: 1rem;
    font-weight: lighter;
    margin-bottom: 20px;
    letter-spacing: 8px; /* Adjust the value to control the space between letters */
  }

  .login-container {
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
    border: 1px solid #ccc;
    background: #ffffff;
    box-shadow: 5px 5px 10px rgba(0, 0, 0, 0.05), -5px -5px 10px #ffffff;
    color: #333;
    font-size: 1rem;
    outline: none;
    width: 100%;
    max-width: 100%; /* Ensures full width within the container */
    box-sizing: border-box; /* Prevents padding from affecting width */
  }

  .input-field:focus {
    box-shadow: inset 3px 3px 6px rgba(0, 0, 0, 0.1), inset -3px -3px 6px #ffffff;
  }

  button {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.6rem 1rem;
    font-size: 1rem;
  }

  .button-container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-top: 1rem;
  }

  .reset-password-button {
    border: none;
    font-size: 0.8rem;
    cursor: pointer;
    color: rgb(57, 57, 57);
  }

  .submit-button,
  .register-button {
    padding: 1rem;
    border: none;
    width: 100%;
    border-radius: 10px;
    font-size: 1rem;
    cursor: pointer;
    background: #ffffff;
    box-shadow: 5px 5px 10px rgba(0, 0, 0, 0.1), -5px -5px 10px #ffffff;
    transition: all 0.3s ease;
    color: rgb(57, 57, 57);
  }

  .submit-button:hover,
  .register-button:hover {
    box-shadow: inset 3px 3px 6px rgba(0, 0, 0, 0.1), inset -3px -3px 6px #ffffff;
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

<div class="login-container">
  <h2>HOOKED</h2>

  <!-- Site or app logo -->
  <img src="/images/logo.png" alt="Logo" class="logo" />

  <!-- Display error message if there is one -->
  {#if $errorMessage}
    <div class="error-message">{$errorMessage}</div>
  {/if}

  <!-- Login form -->
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
      <!-- Login button -->
      <button type="submit" class="submit-button">
        <LogIn size="20" style="margin-right: 8px;" />
        Login
      </button>
      <!-- Go to register page -->
      <button type="button" class="register-button" on:click={() => goto('/register')}>
        <ArrowRight size="20" style="margin-left: 8px;" />
        Sign Up
      </button>

      <!-- Forgotten password text link -->
      <p class="reset-password-button" on:click={() => goto('/resetPassword')}>
        Forgotten Password?
      </p>
    </div>
  </form>
</div>
