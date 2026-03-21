import { initAuthForms } from "./features/auth";
import { initModals } from "./ui/modal";

document.addEventListener('DOMContentLoaded', () => {
  initModals();

  initAuthForms();
});
