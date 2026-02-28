/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    extend: {
      fontSize: {
        xxs: "0.625rem",
      },
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
