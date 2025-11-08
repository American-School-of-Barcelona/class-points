# API

This is a json API for interacting with the backend. Later, we will create
a python wrapper for this API.

## Table of Contents

- [API](#api)
  - [Table of Contents](#table-of-contents)
  - [Basics of Authenticating](#basics-of-authenticating)
  - [Endpoints](#endpoints)
    - [`/api/users` - Users \& Authentication](#apiusers---users--authentication)
      - [POST `/api/users/register`](#post-apiusersregister)
      - [POST `/api/users/verify?code=xxxx`](#post-apiusersverifycodexxxx)
      - [POST `/api/users/login`](#post-apiuserslogin)
      - [GET `/api/users/list`](#get-apiuserslist)
      - [GET `/api/users/authenticated`](#get-apiusersauthenticated)
    - [`/api/points` - Points](#apipoints---points)
      - [POST `/api/points/<student>/modify`](#post-apipointsstudentmodify)
      - [GET `/api/points/<student>`](#get-apipointsstudent)
      - [GET `/api/points/<student>/history`](#get-apipointsstudenthistory)

## Basics of Authenticating

This project has a simplified version of an OATH like authentication system.

To setup authentication in your app, you simply need to redirect the user to the `/login` page
with a callback to your website. The request will include a `token` argument, which your app
can store in a cookie client side to verify the authenticity of a user.

The `/login` page takes two URL parameters: `callback`, and `issuer`. `callback` should point to whichever
URL you want to handle the signin, and `issuer` should just be the name of your app (without spaces).
An example authentication flow goes like so:

1. User visits `https://myapp.com/`, and clicks a "login with ASBCS" button.
2. The app directs them to `https://asbcs.com/register?callback=https://myapp.com/auth/&issuer=myapp`
3. ASBCS then processes the login, and redirects the user to `https://myasb.com/auth/?token=<JWT_TOKEN>`
4. The frontend stores `<JWT_TOKEN>` as a cookie, and from there can send it to the backend when needed.

## Endpoints

### `/api/users` - Users & Authentication

#### POST `/api/users/register`

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

#### POST `/api/users/verify?code=xxxx`

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

#### POST `/api/users/login`

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

#### GET `/api/users/list`

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

#### GET `/api/users/authenticated`

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

### `/api/points` - Points

#### POST `/api/points/<student>/modify`

This requires you to authenticate, and is restricted to users with role 1 or above.

```json
{
    "amount": 10, // can also be negative
    "reason": "my reason here",
    "set": false, // whether to add/subtract or to set the students' point total
}
```

#### GET `/api/points/<student>`

```json
{
    "id": 10,
    "name": "mathis",
    "points": 10,
}
```

#### GET `/api/points/<student>/history`

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
