<script lang="ts">
  import { writable } from 'svelte/store';
  import { goto } from '$app/navigation';
  import { registerAccount } from '../../controllers/accountsController';
  import { UserPlus, ArrowLeft } from 'lucide-svelte'; 

  // FORM STATE
  let email = '';
  let password = '';
  let confirmPassword = '';

  // ERROR & SUCCESS MESSAGES
  // These use svelte writable stores so they update the UI when set
  let errorMessage = writable('');
  let successMessage = writable('');

  // REGISTRATION HANDLER
  const register = async () => {
    // Clear any existing messages
    errorMessage.set('');
    successMessage.set('');

    // Simple validation
    if (!email || !password || !confirmPassword) {
      errorMessage.set('Please fill in all fields.');
      return;
    }

    if (password !== confirmPassword) {
      errorMessage.set('Passwords do not match.');
      return;
    }

    try {
      // Attempt to register via your backend (Rust or Node etc)
      const accountId = await registerAccount(email, password);

      successMessage.set('Registration successful! Redirecting...');
      // Wait 2 seconds then navigate to login page
      setTimeout(() => goto('/login'), 2000); // Redirect after 2 seconds
    } catch (error) {
      console.error('Registration error:', error);
      errorMessage.set('Registration failed. Please try again.');
    }
  };
</script>
  
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
    /* background: #e6f4fd; */
    border: 1px solid #ccc;
    background: #ffffff;
    /* box-shadow: inset 5px 5px 10px #b4d1e3, inset -5px -5px 10px #ffffff; */
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

  .submit-button,
  .login-button {
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
  .login-button:hover {
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
  
<div class="register-container">
  <h2>HOOKED</h2>

  <!-- Logo Image -->
  <img src="/images/logo.png" alt="Logo" class="logo" />

  <!-- ERROR / SUCCESS MESSAGES -->
  {#if $errorMessage}
    <div class="error-message">{$errorMessage}</div>
  {/if}

  {#if $successMessage}
    <div class="success-message">{$successMessage}</div>
  {/if}

  <!-- REGISTRATION FORM -->
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
    
    <div class="button-container">
      <button type="submit" class="submit-button">
        <UserPlus size="20" style="margin-right: 8px"/>
        Sign Up
      </button>
      <button type="button" class="login-button" on:click={() => goto('/login')}>
        <ArrowLeft size="20" style="margin-left: 8px;" />
        Login
      </button>
    </div>
  </form>
</div>
