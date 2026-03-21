import { defineConfig } from "vite";
import { resolve } from "path";

export default defineConfig({
  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, 'index.html'),

        adminLogin: resolve(__dirname, 'admin/index.html'),
        adminDashboard: resolve(__dirname, 'admin/dashboard.html'),

        cart: resolve(__dirname, 'pages/cart.html'),
        checkout: resolve(__dirname, 'pages/checkout.html'),
      },
    },
  },
});
