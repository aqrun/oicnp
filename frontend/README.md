# frontend

dioxus

```
dx serve --features web

dx build --features web --release
cargo run --features ssr --release

dx serve --features ssr --hot-reload --platform desktop
```

## sass 安装

```fish
npm install -g sass
```

## tailwind

```fish
sass -w input.scss node_modules/input.css
pnpm tailwindcss -i node_modules/input.css -o ./public/tailwind.css --watch
```
