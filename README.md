# Welcome to the Appointments App

[demo](https://appointments-demo.dsegovia.io/)

First things first, we need a better name. Feel free to suggest one!

Second, this is not ready for production use. If you're adventurous, you can try it out. Please note that this application is still in development and may contain bugs or unexpected behavior. If you encounter any issues, please report them on the GitHub repository.

## Installation

Pull repo and use the Dockerfile to build the application, it uses sqlite as the database, so make sure you persist its storage. You can configure the config/production.yaml file to suite your needs. Hopefully we can simplify the configuration process in the future.

## Development

We use [loco.rs](https://loco.rs) as a framework. Its quite similar to Rails, but for rust. For the front end, we use [nuxt](https://nuxtjs.org) with [nuxtui](https://ui.nuxt.com/), and [bun](https://bun.sh/) as a runtime (you could use whatever you want, its only for development since the nuxt app is served as static files in production, so no runtime is needed).

### Local Development

I typically use [bacon](https://github.com/Canop/bacon):
```sh
bacon run
```
or you can use:
```sh
cargo loco start
```

### Tests
To run tests, run the following command with [just](https://github.com/casey/just) and [nextest](https://nexte.st/):
```sh
just test # you can pass arguments to the test command just like cargo test
```
Without just nor nextest:
```sh
cargo test --features mock-time
```
