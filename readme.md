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
If database is already prepared, just run
```cargo run```

If you want to isolate all envs such as database, run docker-compose
```docker-compose up -d```

If you encounter `failed to solve with frontend dockerfile.v0` error, then run
```
echo 'export COMPOSE_DOCKER_CLI_BUILD=0' >> ~/.zshrc
echo 'export DOCKER_BUILDKIT=0' >> ~/.zshrc
source ~/.zshrc
```
and run docker-compose up -d again

# Want to join this toy project?
Contact me at `bakjuna@gmail.com`, and let's build this shit together!