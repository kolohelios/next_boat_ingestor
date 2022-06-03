

## .env file for local development
```
REDIS__URL=redis://127.0.0.1:6379
REDIS__POOL__MAX_SIZE=8
REDIS__POOL__TIMEOUTS__WAIT__SECS=2
REDIS__POOL__TIMEOUTS__WAIT__NANOS=0
WSF_API_KEY=<WSF API KEY>
```

## To install watcher (for dev):
`cargo install cargo-watch`

`cargo watch -c -w src -x run`
to watch changes in the `src` folder
