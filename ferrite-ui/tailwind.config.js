/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        ferrite: {
          50: '#fff7ed',
          100: '#ffe4cc',
          200: '#ffc599',
          300: '#ffa066',
          400: '#ff8040',
          500: '#ff6b35',
          600: '#e55a25',
          700: '#cc4a18',
          800: '#a33a12',
          900: '#7a2c0e',
        },
        surface: {
          DEFAULT: '#08080d',
          50: '#0c0c14',
          100: '#12121c',
          200: '#181824',
          300: '#22222f',
          400: '#2c2c3a',
          500: '#383848',
          600: '#484858',
          700: '#606070',
          800: '#8888a0',
          900: '#b0b0c0',
        },
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', '-apple-system', 'sans-serif'],
      },
      fontSize: {
        '2xs': ['0.625rem', { lineHeight: '0.875rem' }],
      },
      spacing: {
        'sidebar': '240px',
        'sidebar-collapsed': '64px',
      },
      borderRadius: {
        'xl': '0.875rem',
        '2xl': '1rem',
        '3xl': '1.5rem',
      },
      backdropBlur: {
        'xs': '2px',
      },
      animation: {
        'fade-in': 'fadeIn 200ms ease-out',
        'fade-out': 'fadeOut 200ms ease-in',
        'slide-up': 'slideUp 300ms ease-out',
        'slide-down': 'slideDown 300ms ease-out',
        'slide-left': 'slideLeft 300ms ease-out',
        'slide-right': 'slideRight 300ms ease-out',
        'scale-in': 'scaleIn 200ms ease-out',
        'shimmer': 'shimmer 1.5s ease-in-out infinite',
        'pulse-slow': 'pulse 3s ease-in-out infinite',
      },
      keyframes: {
        fadeIn: { '0%': { opacity: '0' }, '100%': { opacity: '1' } },
        fadeOut: { '0%': { opacity: '1' }, '100%': { opacity: '0' } },
        slideUp: { '0%': { transform: 'translateY(16px)', opacity: '0' }, '100%': { transform: 'translateY(0)', opacity: '1' } },
        slideDown: { '0%': { transform: 'translateY(-16px)', opacity: '0' }, '100%': { transform: 'translateY(0)', opacity: '1' } },
        slideLeft: { '0%': { transform: 'translateX(16px)', opacity: '0' }, '100%': { transform: 'translateX(0)', opacity: '1' } },
        slideRight: { '0%': { transform: 'translateX(-16px)', opacity: '0' }, '100%': { transform: 'translateX(0)', opacity: '1' } },
        scaleIn: { '0%': { transform: 'scale(0.95)', opacity: '0' }, '100%': { transform: 'scale(1)', opacity: '1' } },
        shimmer: { '0%': { backgroundPosition: '-200% 0' }, '100%': { backgroundPosition: '200% 0' } },
      },
      transitionDuration: {
        '250': '250ms',
        '350': '350ms',
      },
    },
  },
  plugins: [require('@tailwindcss/forms')],
};
