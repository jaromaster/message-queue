# Message Queue

## blazingly fast message queue server written in Rust :rocket:

## Why
- fast and lightweight :rocket:
- concurrency using threadpools (only dependency) :rocket:
- no configuration needed :pray: 
- no special protocol needed (only simple REST calls) :sunglasses:

## Install
- clone repo
- build using cargo
- run ./message-queue

## Planned
- [ ] auth using JWT
- [ ] https

## Usage
### NOTE: replace 'queue-name' with the actual name of the message queue
### NOTE: you can create these REST calls using any tool/programming language you want

### create a new message queue (using curl)
`curl -X POST localhost:8080/new/queue-name`

### send message to message queue (using curl)
`curl -X POST localhost:8080/add/queue-name -d "this is a message"` <br>
`curl -X POST localhost:8080/add/queue-name -d "{"message": "this is a json message"}"` <br>

### retrieve message from message queue (using curl)
`curl -X GET localhost:8080/get/queue-name`

### delete message queue (using curl)
`curl -X DELETE localhost:8080/delete/queue-name`

## API
[click for more information](API.md)
