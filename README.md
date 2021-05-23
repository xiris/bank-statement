# bank-statement
Bank statement reader

# Available endpoints
```
    GET /transactions
    GET /transactions/{id}
    POST /transactions
    DELETE /transactions/{id}
```

# How to use

After getting the token from auth0, pass it via curl
```
$ export TOKEN=<token-from-auth0>
$ curl -H "Authorization: Bearer $TOKEN" -v 127.0.0.1:8081/transactions
```
