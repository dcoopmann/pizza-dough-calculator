/*
Pizza dough regulations according to https://www.pizzanapoletana.org/en/ricetta_pizza_napoletana

The following doses are based on 1 litre (1000ml) of water;:
- Water: 1 litre (1000 ml)
- Salt: 40-60 grams
- Yeast (based on temperature and humidity):
Fresh beer yeast 0.1-3 grams
Mother Yeast 5-20% of flour used
Dry yeast 1/3 of fresh yeast used (1 gram of dry for 3 grams of fresh)
- Flour: 1,600/1,800 (depending on the degree of absorption).
*/
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PizzaDough {
    pub portions: f32,
    pub size: String,
    flour: Option<f32>,
    water: Option<f32>,
    salt: Option<f32>,
    yeast: Option<f32>,
    pub yeast_type: String,
}

impl PizzaDough {
    pub fn new(p: f32, s: String, y: String) -> Self {
        let base = 125.0;
        let size_factor = match s.trim().to_uppercase().as_str() {
            "S" => 0.75,
            "M" => 1.0,
            "L" => 1.25,
            "XL" => 1.5,
            _ => 1.0,
        };
        let yeast = match y.trim().to_uppercase().as_str() {
            "F" => (0.001, "Fresh Yeast"),
            "D" => (0.0003, "Dry Yeast"),
            "L" => (0.15, "Lievito Madre"),
            _ => (0.001, "Fresh Yeast"),
        };
        PizzaDough {
            portions: p,
            size: s.to_uppercase(),
            flour: Some(base * p * size_factor),
            water: Some((base * 0.6) * p * size_factor),
            salt: Some((base * 0.03) * p * size_factor),
            yeast: Some((base * yeast.0) * p * size_factor),
            yeast_type: yeast.1.to_string(),
        }
    }

    pub fn printout(self) {
        if self.portions > 1.0 {
            println!("For {} pizzas size {} you knead:", self.portions, self.size);
        } else {
            println!("For {} pizza size {} you knead:", self.portions, self.size);
        }
        println!("Flour:   {}g", self.flour.unwrap());
        println!("Water: {}g", self.water.unwrap());
        println!("Salt:   {}g", self.salt.unwrap());
        println!("{}:   {}g", self.yeast_type, self.yeast.unwrap());

        let ta =
            self.flour.unwrap() + self.water.unwrap() + self.salt.unwrap() + self.yeast.unwrap();

        println!("Doug weight total: {}g", ta);
        println!("Doug weight portion: {}g", ta / self.portions);
    }
}

#[cfg(test)]
mod tests {
    use crate::pizza_dough::PizzaDough;

    #[test]
    fn test_one_medium() {
        let dough = PizzaDough::new(1.0, String::from("m"), String::from("f"));
        assert_eq!(dough.flour, Some(125.0));
        assert_eq!(dough.water, Some(75.0));
        assert_eq!(dough.salt, Some(3.75));
        assert_eq!(dough.yeast, Some(0.125));
    }
}
