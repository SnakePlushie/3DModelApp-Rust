# 3D Modeling app

Usefull commands

```sh
cargo build  # build
cargo run    # run
cargo test   # test
cargo fmt    # format
cargo clippy # lint
```

Subolders designations :

- Domain:
  - Entity: unique object (ex: user)
  - ValueObject: non unique object, data holder (ex: adress with city, country, etc)
  - Service: execute an operation and verify business logic

- Application:
  - Dto: tranfer object from infra to application
  - Interface: port, allow infra <-> application comunication
    - In / Out
  - UseCase: process made of multiple actions (ex: withdrawing money)

- Infrastructure:
  - In:
    - Controller: input requests
    - Presenter: mapper / formatter for outgoing data, injected by controller in use case
  - Out:
    - Gateway: outgoing requests
