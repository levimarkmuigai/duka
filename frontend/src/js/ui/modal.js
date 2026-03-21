export const closeModal = (modalElement, cardId) => {
  const cardElement = document.getElementById(cardId);

  cardElement.classList.remove('opacity-100', 'translate-y-0', 'scale-100');
  cardElement.classList.add('opacity-0', 'translate-y-8', 'scale-95');

  setTimeout(() => {

    modalElement.classList.remove('opacity-100', 'pointer-events-auto');
    modalElement.classList.add('opacity-0', 'pointer-events-none');
    document.body.classList.remove('overflow-hidden');
  }, 300);
};



export const openModal = (modalElement, cardId) => {
  const cardElement = document.getElementById(cardId);

  modalElement.classList.remove('opacity-0', 'pointer-events-none');
  modalElement.classList.add('opacity-100', 'pointer-events-auto');

  setTimeout(() => {

    cardElement.classList.remove('opacity-0', 'translate-y-8', 'scale-95');
    cardElement.classList.add('opacity-100', 'translate-y-0', 'scale-100');
  }, 10);
  document.body.classList.add('overflow-hidden');
};

export const initModals = () => {
  const signupBtn = document.getElementById('signup');
  const signupModal = document.getElementById('signup-modal');

  const loginBtn = document.getElementById('login');
  const loginModal = document.getElementById('login-modal');

  loginBtn.addEventListener('click', () => {
    closeModal(signupModal, 'signup-card');
    openModal(loginModal, 'login-card');
  });

  signupBtn.addEventListener('click', () => {
    closeModal(loginModal, 'login-card');
    openModal(signupModal, 'signup-card');
  });

  loginModal.addEventListener('click', (event) => {
    if (event.target === loginModal) {
      closeModal(loginModal, 'login-card');
    }
  });

  signupModal.addEventListener('click', (event) => {
    if (event.target === signupModal) {
      closeModal(signupModal, 'signup-card');
    }
  });

  window.addEventListener('keydown', (event) => {
    if (event.key === 'Escape') {
      closeModal(signupModal, 'signup-card');
      closeModal(loginModal, 'login-card');
    }
  });
};
