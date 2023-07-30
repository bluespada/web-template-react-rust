const mix = require('laravel-mix');

mix
.ts("app/main.tsx", "dist")
.react()