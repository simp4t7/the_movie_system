# movie_rating_site

Umm, you have to run warp_back in the /movie_rating_site directory.

For example, "cargo r --release --bin warp_back" instead of going into /warp_back and running "cargo r --release"

The reason is sqlx needs the "DATABASE_URL" environment variable set at compile time, and github actions has a different dir structure, pretty annoying.

checking actions...

1. in project root directory: run `cargo r --release --bin warp_back`
2. in yew_front/main.rs: change the urls
3. in yew_front directory: run `trunk serve`
