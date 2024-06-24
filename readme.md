# Pizza Dough Calculator
This is a toy project with the goal of learning the Rust programming language. A simple calculator for Neapolitan style pizza dough. When i have time I try to improve/over engineer this further. It follows the proportions recommended by the [Associazione Verace Pizza Napoletana](https://www.pizzanapoletana.org/en/) and can be used either as a simple CLI tool or a web server.

## CLI Tool
Just running it will give you the default recipe for two medium servings using fresh yeast. To show options for modifications see `--help`

## Web Server
To start the serve use the `--serve` flag. Use `--port <PORT>` to bind to a different port than 8080. Pizza dough is served under `localhost:8080/serve-dough`, if not bound to another port. For API details see [ADR 001](ADRs/001_make_it_serve_an_api.md)

# Building
For faster build times this Project is configured to use the [mold linker](https://github.com/rui314/mold), which needs to be installed separately.