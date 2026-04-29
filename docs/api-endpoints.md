# API Endpoints

Seris uses a small set of external HTTP endpoints:

## External APIs

* `GET https://api.jikan.moe/v4/random/anime`
* `GET https://api.jikan.moe/v4/random/manga`
* `GET https://api.nasa.gov/EPIC/api/natural/images?api_key=...`
* `GET https://api.nasa.gov/planetary/apod?api_key=...`

## Notes

* Jikan is used for `/anime random` and `/manga random`.
* EPIC is used for `/epic`.
* NASA APOD is used for `/nasa apod`.
* Seris does not expose internal HTTP endpoints.
