import type { Config } from "tailwindcss";

// We want each package to be responsible for its own content.
const config: Omit<Config, "content"> = {
  theme: {
    extend: {
      fontFamily: {
        sans: ["Inter"],
      },
      backgroundImage: {
        "glow-conic": "conic-gradient(from 180deg at 50% 50%, #2a8af6 0deg, #a853ba 180deg, #e92a67 360deg)",
      },
      colors: {
        olive: {
          50: "#FAF9E0",
          100: "#F5F2C2",
          200: "#EBE586",
          300: "#E0D849",
          400: "#D7CF1A",
          500: "#C0B60B",
          600: "#A39B09",
          700: "#807707",
          800: "#5D5405",
          900: "#3A3203",
          950: "#1A1A00",
        },
      },
      height: {
        "4.5": "1.125rem",
      },
      width: {
        "4.5": "1.125rem",
        "57": "14.313rem",
        "65": "16.25rem",
      },
      margin: {
        "13": "3.25rem",
      },
    },
  },
  plugins: [],
};

export default config;
