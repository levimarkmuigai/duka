import { submitAuthForm } from "../api/auth";
import { closeModal } from "../ui/modal";

export const initAuthForms = () => {
  const signupForm = document.querySelector('#signup-modal form');
  const signupModal = document.getElementById('signup-modal');

  if (!signupForm) return;

  signupForm.addEventListener('submit', async (event) => {
    event.preventDefault();

    const submitBtn = signupForm.querySelector('button[type = "submit"]');
    const originalText = submitBtn.innerText;
    submitBtn.innerText = 'Loading...';
    submitBtn.disabled = true;

    try {
      const result = await submitAuthForm(signupForm, 'api/register_merchant');
      console.log('User registered successfully', result);
      closeModal(signupModal, 'signup-card');
      alert("User registered successfully");
    } catch (error) {
      alert("Registration failed. Please try again");
      closeModal(signupModal, 'signup-card')
    } finally {
      submitBtn.innerText = originalText;
      submitBtn.disabled = false;
    }
  });

  const loginForm = document.querySelector('#login-modal form');
  const loginModal = document.getElementById('login-modal');

  if (!loginForm) return;

  loginForm.addEventListener('submit', async (event) => {
    event.preventDefault();

    const submitBtn = loginForm.querySelector('button[ type="submit"]');
    const originalText = submitBtn.innerText;
    submitBtn.innerText = 'Loading...';
    submitBtn.disabled = true;

    try {
      const result = await submitAuthForm(loginForm, 'api/login');

      if (result.status === 'success') {
        console.log('User logged in successfully', result);
        window.location.href = result.redirect;
      }
    } catch (error) {
      alert("Failed to login. Please try again");
      closeModal(loginModal, 'login-card');
    } finally {
      submitBtn.innerText = originalText;
      submitBtn.disabled = false;
    }
  })
};

