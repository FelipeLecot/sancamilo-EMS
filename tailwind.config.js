const { fontFamily } = require("tailwindcss/defaultTheme");
/** @type {import('tailwindcss').Config} */ module.exports = {
	content: ["./templates/*.html"],
	theme: {
		extend: { 
      fontFamily: { sans: ["Inter var", ...fontFamily.sans] },
      screens: {
        sm: '480px',
        md: '768px',
        lg: '976px',
        xl: '1440px',
      },
      container: {
        center: true,
        padding: "1rem",
      },
      colors: {
        'primary': '#729B79',
        'secondary': '#475B63',
        'white': '#F3E8EE',
        'black': '#2E2C2F',
      }
    }
	}
};
