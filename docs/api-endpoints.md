# API Endpoints

Seris uses a small set of HTTP endpoints:

## External APIs

* `GET https://api.jikan.moe/v4/random/anime`
* `GET https://api.jikan.moe/v4/random/manga`
* `GET https://api.nasa.gov/planetary/apod?api_key=...`

## Internal endpoints

* `GET /health` on port `8080` returns `200 OK` when the process is alive.
* `GET /ready` on port `8080` returns `200 OK` when Discord is connected and `503` otherwise.

## Notes

* Jikan is used for `/anime random` and `/manga random`.
* NASA APOD is used for `/nasa apod`.
* The health server is intentionally tiny so container checks stay predictable.
