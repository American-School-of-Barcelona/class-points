# API

This is a json API for interacting with the backend. Later, we will create
a python wrapper for this API.

## Table of Contents

- [API](#api)
  - [Table of Contents](#table-of-contents)
  - [Endpoints](#endpoints)
    - [`/users` - Users \& Authentication](#users---users--authentication)
      - [POST `/users/register`](#post-usersregister)
      - [POST `/users/verify?code=xxxx`](#post-usersverifycodexxxx)
      - [POST `/users/login`](#post-userslogin)
      - [GET `/users/list`](#get-userslist)
      - [GET `/users/authenticated`](#get-usersauthenticated)
    - [`/points` - Points](#points---points)
      - [POST `/points/<student>/modify`](#post-pointsstudentmodify)
      - [GET `/points/<student>`](#get-pointsstudent)
      - [GET `/points/<student>/history`](#get-pointsstudenthistory)

## Endpoints

### `/users` - Users & Authentication

#### POST `/users/register`

Registers a new user.

Request Body:

```json
{
    "name": "john",
    "email": "john@asbarcelona.com",
    "password": "john"  
}
```

Response:

```json
{
    "email": "john@asbarcelona.com",
}
```

#### POST `/users/verify?code=xxxx`

Verifies a newly registered user with a verification code.
This must be done *before* the first login.

Response:

```json
{
    "id": 2,
    "name": "john",
    "points": 0,
    "role": 0
}
```

#### POST `/users/login`

Logs in an existing user.

Request Body:

```json
{
  "name": "john",
  "password": "password"
}
```

Response:

```json
{
    "token": "<JWT_TOKEN>"
}
```

#### GET `/users/list`

```json
{
    "students": [
        {
            "id": 2,
            "name": "john",
            "points": 10,
        },
        // ...
    ]
}
```

#### GET `/users/authenticated`

Returns data about the authenticated user.
This endpoint requires you be authenticated.

```json
{
    "id": 2,
    "name": "john",
    "points": 0,
    "role": 0
}
```

### `/points` - Points

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
