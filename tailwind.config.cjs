const defaultTheme = require("tailwindcss/defaultTheme");

module.exports = {
  content: ["./src/**/*.{rs,html}", "./style/**/*.{scss,css}"],
  theme: {
    extend: {
      colors: {
        background: "#0b1220",
        surface: "#111a2b",
        "surface-strong": "#182134",
        border: "rgba(255, 255, 255, 0.08)",
        foreground: "#f8fafc",
        brand: {
          DEFAULT: "#6366f1",
          muted: "#4f46e5",
          foreground: "#0b1220",
        },
        success: "#34d399",
        danger: "#f87171",
      },
      fontFamily: {
        sans: ["'Inter'", ...defaultTheme.fontFamily.sans],
        mono: ["'JetBrains Mono'", ...defaultTheme.fontFamily.mono],
      },
      backgroundImage: {
        "grid-soft":
          "linear-gradient(to right, rgba(255,255,255,0.04) 1px, transparent 1px), linear-gradient(to bottom, rgba(255,255,255,0.04) 1px, transparent 1px)",
      },
      backgroundSize: {
        "grid-soft": "24px 24px",
      },
    },
  },
  plugins: [],
};

