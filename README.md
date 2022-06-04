# Next Boat - Ingestor

## .env file for local development
```
REDIS__URL=redis://127.0.0.1:6379
REDIS__POOL__MAX_SIZE=8
REDIS__POOL__TIMEOUTS__WAIT__SECS=2
REDIS__POOL__TIMEOUTS__WAIT__NANOS=0
WSF_API_KEY=<WSF API KEY>
```

Key construction:
V__{vessel ID}__{property}
V__2__latitude
V__2__longitude
V__2__speed
V__2__heading
V__2__eta
V__2__in_service
V__2__at_dock
V__2__left_dock
V__2__eta_basis
V__2__departing_terminal_id
V__2__arriving_terminal_id
V__2__vessel_position
V__2__scheduled_departure

## To install watcher (for dev):
`cargo install cargo-watch`

`cargo watch -c -w src -x run`
to watch changes in the `src` folder
