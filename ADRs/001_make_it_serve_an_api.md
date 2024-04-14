# Serve API for web based pizza dough calculation

As a next step the pizza dough calculator shall serve an api over which pizza dough calculations can be fetched using a HTTP GET request.

## Get Pizza dough
### Body
```json
{
    "portions": "String",   
    "size": "String",
    "yeast_type": "String",
}
```
### Response
```json
    "portions": "String",   // parsed from f32
    "size": "String",
    "flour": "String",      // parsed from f32
    "water": "String",      // parsed from f32
    "salt": "String",       // parsed from f32
    "yeast": "String",      // parsed from f32
    "yeast_type": "String",
```

## Framework
For simplicity of use [actix-web](https://actix.rs/docs/getting-started) was decided to be the framework used to server http requests. 