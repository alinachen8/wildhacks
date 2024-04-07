<script lang="ts">
  import { goto } from '$app/navigation';
  // ... other imports and script content remain the same ...
  let email: string = ''; 
  let password: string = '';

</script>

<!-- This is the main container that centers everything on the page -->
<div class="page-container">
  <!-- The form itself, which will submit and prevent the default page reload -->
  <form on:submit|preventDefault={login}>
    <!-- Heading for Sign In -->
    <h2 style="color: black; font-weight: bold; font-size: 15px; margin-top: -140px; text-align: center;">SIGN IN</h2>

    <!-- A group for the email input, with a label and input field -->
    <div class="form-group">
      <label for="email">Email</label>
      <input 
        type="email"
        id="email"
        required
        name="email"
        placeholder="Enter your email"
        bind:value={email}
      />
    </div>

    <!-- A group for the password input, with a label and input field -->
    <div class="form-group">
      <label for="password">Your password</label>
      <input 
        type="password"
        id="password"
        required
        name="password"
        placeholder="Enter your password"
        bind:value={password}
      />
    </div>

    <!-- The submit button for the form -->
    <button type="submit">Sign in</button>

    <!-- Text and link for users who need to sign up -->
    <p> Don't have an account? </p>
    <a href="/signup"> Sign up. </a>
  </form>
</div>

<svelte:head>
  <title>Sign In</title>
</svelte:head>

<!-- The styling to make the page look like the provided image -->
<style>
  /* Styles for the main page container to center everything */
  .page-container {
    display: flex;
    justify-content: center; /* Center children vertically */
    align-items: center; /* Center children horizontally */
    height: 100vh; /* Full height of the viewport */
    /* Background color for the entire page, change as needed */
    background-color: #fbe9df;
    /* Font family for the entire page, change as needed */
    font-family: 'Helvetica', 'Arial', sans-serif;
  }

  /* Styles for the form to organize its children */
  form {
    display: flex;
    flex-direction: column; /* Stack children vertically */
    align-items: center; /* Center children horizontally */
    gap: 1rem; /* Space between form items */
    /* Add other styles like padding, background, etc., as needed */
  }

  /* Styles for each form group (label + input) */
  .form-group {
    display: flex;
    flex-direction: column; /* Stack label and input vertically */
    align-items: flex-start; /* Align items to the start of the flex container */
    width: 100%; /* Full width of the form */
  }

  /* Style for the labels */
  label {
    font-weight: bold; /* Make text bold */
    margin-bottom: 0.5rem; /* Space below the label */
    text-transform: uppercase; /* UPPERCASE text */
  }

  /* Style for the input fields */
  input {
    font-size: 1rem;
    padding: 0.5rem; /* Padding inside the input fields */
    margin-bottom: 1rem; /* Space below the input fields */
    border: 2px solid #ccc; /* Border around the input fields */
    border-radius: 8px; /* Rounded corners for the input fields */
  }

  /* Style for the submit button */
  button {
    background-color: #84cc16; /* Background color for the button */
    color: white; /* Text color for the button */
    border: none; /* No border for the button */
    padding: 0.5rem 1rem; /* Padding inside the button */
    border-radius: 20px; /* Rounded corners for the button */
    cursor: pointer; /* Pointer cursor on hover */
    text-transform: uppercase; /* UPPERCASE text */
    font-weight: bold; /* Bold text */
  }

  /* Hover effect for the button */
  button:hover {
    background-color: #6fa914; /* Slightly darker color on hover */
  }

  /* Style for the sign-up link */
  a {
    color: #1e3a8a; /* Text color for the link */
    text-decoration: none; /* No underline by default */
  }

  /* Hover effect for the link */
  a:hover {
    text-decoration: underline; /* Underline on hover */
  }

  /* Style for the paragraph above the sign-up link */
  p {
    margin: 1rem 0; /* Space above and below the paragraph */
  }
</style>
