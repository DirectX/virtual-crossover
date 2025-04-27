/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"]
    },
    theme: {
        extend: {
            container: {
                padding: '2rem',
            },
        },
    },
    plugins: [],
}