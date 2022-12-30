# API Documentation

### NOTE: ':queuename' is just a placeholder for the actual name of the message queue

## POST
---
### /new/:queuename
create a new message queue
| Description | Status code |
| --- | --- |
| queue already exists | 400 |
| success | 200 |

### /add/:queuename
add message to specific queue (as request body)
| Description | Status code |
| --- | --- |
| queue does not exist | 404 |
| success | 200 |

## GET
---
### /get/:queuename
retrieve message from specific queue
| Description | Status code |
| --- | --- |
| queue does not exist | 404 |
| success | 200 |

## DELETE
---
### /delete/:queuename
delete the target queue
| Description | Status code |
| --- | --- |
| could not remove queue | 400 |
| success | 200 |