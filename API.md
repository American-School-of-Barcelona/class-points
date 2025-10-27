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
        },
        // ...
    ]
}
```

### POST `/students/new`

```json
{
    "id": 10,
    "name": "mathis"  
}
```

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
