# Serve API for web based pizza dough calculation

As a next step the pizza dough calculator shall serve an api over which pizza dough calculations can be fetched using a HTTP POST request.

## Framework
For simplicity of use [actix-web](https://actix.rs/docs/getting-started) was decided to be the framework used to server http requests. 

## POST to get pizza dough
In the communication between client and server the PizzaDough struct is used to transfer the needed data. Fields not needed in the request body become optional.

```rust
pub struct PizzaDough {
    pub portions: f32,
    pub size: String,
    pub yeast_type: String,
    flour: Option<f32>,
    water: Option<f32>,
    salt: Option<f32>,
    yeast: Option<f32>,
    total_weight: Option<f32>,
    portion_weight: Option<f32>,
    measurement_unit: String,
}
```
### Example

#### Request body

```json
{
    "portions": 2.0,
    "size": "L",
    "yeast_type": "F"
}
```

#### Response
```json
{
    "portions": 2.0,   
    "size": "L",
    "yeast_type": "Fresh Yeast",
    "flour": 312.5,      
    "water": 187.5,      
    "salt": 9.375,       
    "yeast": 0.3125,
    "total_weight": 509.6875,
    "portion_weight": 254.84375,      
    "measurement_unit": "g",
}
```