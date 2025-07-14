<script lang="ts">
    import { writable } from 'svelte/store';
    import { goto } from '$app/navigation';
    import { loginAccount } from '../../controllers/accountsController';
    import { Mail, ArrowLeft } from 'lucide-svelte'; 
  
    let email = '';
    let errorMessage = writable('');
  
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

    <!-- Logo Image -->
    <img src="/images/logo.png" alt="Logo" class="logo" />
  
    <!-- Conditional rendering for error message -->
    {#if $errorMessage}
      <div class="error-message">{$errorMessage}</div>
    {/if}
  
    <!-- on:submit|preventDefault={login} -->
    <form class="reset-password-form">
      <input 
        type="email" 
        class="input-field" 
        placeholder="Email" 
        bind:value={email} 
        required
      />

      <div class="button-container">
        <button type="submit" class="submit-button">
          <Mail size="20" style="margin-right: 8px;" />
          Email Reset Link
        </button>
        <button type="button" class="login-button" on:click={() => goto('/login')}>
          <ArrowLeft size="20" style="margin-left: 8px;" />
          Return To Login
        </button>
      </div>
    </form>
  </div>
  