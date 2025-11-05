# API

This is a json API for interacting with the backend. Later, we will create
a python wrapper for this API.

## Endpoints

### Points

#### POST `/points/<student>/modify`

This requires you to authenticate, and is restricted to users with role 1 or above.

```json
{
    "amount": 10, // can also be negative
    "reason": "my reason here",
    "set": false, // whether to add/subtract or to set the students' point total
}
```

#### GET `/points/<student>`

```json
{
    "id": 10,
    "name": "mathis",
    "points": 10,
}
```

#### GET `/points/<student>/history`

```json
{
    "history": [
        {
            "change": 10,
            "points": 10,
            "reason": "my reason here",
            "date": "YYYY-MM-DDTHH:mm:ssZ"
        },
        // ...
    ]
}
```

### Users & Authentication

#### GET `/users/authenticated`

This endpoint requires you be authenticated.

```json
{
    "id": 2,
    "name": "jimmybob",
    "points": 0,
    "role": 0
}
```

#### POST `/users/register`

Request Body:

```json
{
    "name": "mathis",
    "email": "mathis@asbarcelona.com",
    "password": "asdf1234"  
}
```

Response:

```json
{
    "email": "mathis@asbarcelona.com",
}
```

#### POST `/users/verify?code=xxxx`

Response:

```json
{
    "id": 10,
    "name": "mathis",
    "points": 0,
    "role": 0
}
```

#### POST `/users/list`

```json
{
    "students": [
        {
            "id": 10,
            "name": "mathis",
            "points": 10,
        }
    ]
}
```
