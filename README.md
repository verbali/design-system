# Verbali design system

## Running project

1. Run by using docker
```bash
docker-compose up -d
```

2. Install node dependencies
```bash
docker compose exec app npm install
```

3. Watch and compile tailwindcss
```bash
docker compose exec app npx tailwindcss -i ./assets/input.css -o ./assets/ds.css --watch
```
