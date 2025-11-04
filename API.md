# API

This is a json API for interacting with the backend. Later, we will create
a python wrapper for this API.

## Endpoints

### POST `/points/<student>/modify`

```json
{
    "amount": 10, // can also be negative
    "reason": "my reason here",
    "set": false, // whether to add/subtract or to set the students' point total
}
```

### GET `/points/<student>/amount`

```json
{
    "amount": 10,
}
```

### GET `/points/<student>/history`

```json
{
    "history": [
        {
            "change": 10,
            "reason": "my reason here"
            "date": "YYYY-MM-DDTHH:mm:ssZ"
        },
        // ...
    ]
}
```

### POST `/students/new`

Request Body:

```json
{
    "name": "mathis"  
}
```

Response:

```json
{
    "name": "mathis",
    "id": 10,
}

### POST `/students/list`

```json
{
    "students": [
        {
            "id": 10,
            "name": "mathis",
            "amount": 10,
        }
    ]
}
```
