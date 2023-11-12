# Toy project for fun
It's just for fun!

# making plausible lottery numbers for Korea made with rust
This lottery web framework generates and presents plausible lottery numbers very quickly. I've implemented the ability to exclude numbers if they have a lot of consecutive numbers, are reminiscent of a pattern, or overlap very closely with previously won numbers, and the ability to backtest multiple lottery scenarios.

# How this server made?
**disclimer** : This is just for my own amusement, so you should not expect fast development.
I've constructed this server with Axum

[![Build status](https://github.com/tokio-rs/axum/actions/workflows/CI.yml/badge.svg?branch=main)](https://github.com/tokio-rs/axum/actions/workflows/CI.yml)
[![Crates.io](https://img.shields.io/crates/v/axum)](https://crates.io/crates/axum)
[![Documentation](https://docs.rs/axum/badge.svg)](https://docs.rs/axum)

# To look at how this server works
In `main.rs`, there are Axum & tokio-scheduler.

# Key features
- User login system with google, saves strategy and settings
  - Not yet done
- redis BullMQ implementation
   - Not yet done
- Fetch latest lottery numbers by excel, parses it and save it to the database
  - Not yet done
- Exclude lottery number with duplicated numbers
  - Done
- Backtesting lottery strategy
  - Not yet done
- Database Connection
  - Done
- Dockerfile
  - Done
- Conventional lottery community
  - Not yet done

# How to run this server
## Build
### dev
```
make build-dev
```
### prod
```
make build-prod
```

## Lint
```
make lint
```

## Compile
Compile contains copy env files, lint, and build process.
### dev
```
make compile-dev
```
### prod
```
make compile-prod
```
## start server
This should ensure appropriate database is already started. Otherwise, start docker, not local server.
### dev
```
make start-dev
```
### prod
```
make start-prod
```

## watch
To watch source codes,
```
make watch
```
If you want to watch test codes, then
```
make watch-tests
```

## Dockerizing
If you want to isolate all envs such as database, run docker-compose. You can simply start dockerizing with this command.
```
make dcr
```

If you want to stop it, then type
```
make dcs
```

If you want to remove docker images, then type
```
make dcd
```

## Encountered error?
If you encounter `failed to solve with frontend dockerfile.v0` error, then run
```
make fix-docker-issue
```
and run `make dcs` again

## Building an image
Acutally every time you run docker-compose, it automatically makes an image of this server. But if you want to do that in your local without docker-compose, then see here.

### For ARM Architecture
```
make image-mac
```
### For AMD64 Architecture
```
make image-amd64
```

# Want to join this toy project?
Contact me at `bakjuna@gmail.com`, and let's build this shit together!