<script lang="ts">
    import { writable } from 'svelte/store';
    import { goto } from '$app/navigation';
    import { loginAccount } from '../../controllers/accountsController';
  
    let email = '';
    let password = '';
    let errorMessage = writable('');
  
    // Simulate a login API request (you can replace this with your real API call)
    const login = async () => {
      // Clear previous error messages
      errorMessage.set('');

      if (!email || !password) {
        errorMessage.set('Please fill in both fields.');
        return;
      }
  
      try {
        // Call the backend via the login function (you will get a token if login is successful)
        const token = await loginAccount(email, password);  // Call the Tauri command login function
        console.log('Login successful, received token:', token);

        // Store the token in localStorage
        localStorage.setItem('token', token);

        // Redirect to the dashboard (or any other page)
        goto('/');
      } catch (error) {
        console.error('Login error:', error);
        errorMessage.set('Invalid credentials, please try again.');
      }
    };
  </script>
  
  <!-- <style global>
    /* Body and Background */
    body {
      background-color: #121212; /* Dark background */
      font-family: 'Poppins', sans-serif;
      margin: 0;
      padding: 0;
      color: #f5f5f5; /* Light text for contrast */
    }

    .login-container {
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
  
    .login-form {
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
      /* box-shadow: 0 0 5px rgba(0, 123, 255, 0.5); */
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
    .register-button {
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
      /* background-color: #00ff8092; */
      /* background-color: #ff8000b9; */
      color: white;
    }
  
    .submit-button:hover {
      /* border: 2px solid #00ff8000; */
      /* background-color: #00ff8000; */
      transform: scale(1.05);
    }
  
    .register-button {
      /* background-color: #6c757d; */
      border: 2px solid #ff8000b9; /* Thin green outline */
      background-color: transparent; /* No background color */
      color: white;
    }
  
    .register-button:hover {
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

  <!-- Poppins
  SF Pro Display (Apple's system font)
  Inter
  Montserrat -->

    <style global>
      body {
        background-color: #e6f4fd;
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
        background: #e6f4fd;
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
      .register-button {
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
      .register-button:hover {
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
  
  
  <div class="login-container">
    <h2>HOOKED</h2>

    <!-- Logo Image -->
    <img src="/images/logo.png" alt="Logo" class="logo" />
  
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
        <button type="button" class="register-button" on:click={() => goto('/register')}>Sign Up</button>
      </div>
    </form>
  </div>
  