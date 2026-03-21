import { submitAuthForm } from "../api/auth";
import { closeModal } from "../ui/modal";

export const initAuthForms = () => {
  const signupForm = document.querySelector('#signup-modal form');
  const signupModal = document.getElementById('signup-modal');

  if (!signupForm) return;

  signupForm.addEventListener('submit', async (event) => {
    event.preventDefault();

    const submitBtn = signupForm.querySelector('button[type = "submit');
    const originalText = submitBtn.innerText;
    submitBtn.innerText = 'Loading...';
    submitBtn.disable = true;

    try {
      const result = await submitAuthForm(signupForm, 'register');
      console.log('User registered successfully', result);

      closeModal(signupModal, 'signup-card');
    } catch (error) {
      alert("Registration failed. Please try again");
    } finally {
      submitBtn.innerText = originalText;
      submitBtn.disable = false;
    }
  });
};
