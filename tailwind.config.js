/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        bg: "rgb(var(--bg) / <alpha-value>)",
        sidebar: "rgb(var(--sidebar) / <alpha-value>)",
        hover: "rgb(var(--hover) / <alpha-value>)",
        code: "rgb(var(--code) / <alpha-value>)",
        line: {
          DEFAULT: "rgb(var(--line) / <alpha-value>)",
          strong: "rgb(var(--line-strong) / <alpha-value>)",
        },
        fg: "rgb(var(--fg) / <alpha-value>)",
        heading: "rgb(var(--heading) / <alpha-value>)",
        muted: "rgb(var(--muted) / <alpha-value>)",
        post: "rgb(var(--post) / <alpha-value>)",
        author: "rgb(var(--author) / <alpha-value>)",
        mod: "rgb(var(--mod) / <alpha-value>)",
        logo: "rgb(var(--logo) / <alpha-value>)",
      },
      fontFamily: {
        sans: ['"Fira Sans"', '"Helvetica Neue"', "Arial", "sans-serif"],
        serif: ['"Source Serif 4"', '"Iowan Old Style"', "Georgia", "serif"],
        mono: ['"Source Code Pro"', "ui-monospace", "SFMono-Regular", "monospace"],
      },
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
