import daisyui from "daisyui";
import forms from "@tailwindcss/forms";
import typography from "@tailwindcss/typography";

/** @type {import('tailwindcss').Config}*/
const config = {
  content: ["./src/**/*.{html,js,svelte,ts}"],

  theme: {
    extend: {}
  },

  plugins: [forms, daisyui, typography],

  daisyui: {
    themes: ["business"],
  },
};

module.exports = config;