export default {
  content: ['./index.html', './src/**/*.{ts,tsx}'],
  theme: {
    extend: {
      fontFamily: {
        display: ['Inter', 'system-ui', 'sans-serif'],
      },
      boxShadow: {
        card: '0 20px 60px rgba(15, 23, 42, 0.08)',
      },
      colors: {
        brand: {
          500: '#0F4C81',
          600: '#0B3A63',
        },
      },
    },
  },
  plugins: [],
};
