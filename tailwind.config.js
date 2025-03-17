/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html",
  ],
  theme: {
    extend: {
      colors: {
        primary: '#6B46C1', // Example purple color
        primaryDark: '#553C9A',
        secondary: '#D53F8C',
        neutralDark: '#2D3748',
        white: '#FFFFFF',
      },
    },
  },
  plugins: [],
}

