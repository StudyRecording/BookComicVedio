/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    "./src/**/*.{rs,html,css}", 
    "./dist/**/*.html",
    // "./dist/*.css"
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};
