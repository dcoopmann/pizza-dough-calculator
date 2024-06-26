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

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
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

impl PizzaDough {
    pub fn new(portions: f32, size: String, yeast_type: String) -> Self {
        let base = 125.0;
        let size_factor = match size.trim().to_uppercase().as_str() {
            "S" => 0.75,
            "M" => 1.0,
            "L" => 1.25,
            "XL" => 1.5,
            _ => 1.0,
        };
        let yeast = match yeast_type.trim().to_uppercase().as_str() {
            "F" => (0.001, "Fresh Yeast"),
            "D" => (0.0003, "Dry Yeast"),
            "L" => (0.15, "Lievito Madre"),
            _ => (0.001, "Fresh Yeast"),
        };

        let size = size.to_uppercase();
        let flour = Some(base * portions * size_factor);
        let water = Some((base * 0.6) * portions * size_factor);
        let salt = Some((base * 0.03) * portions * size_factor);
        let yeast_weight = Some((base * yeast.0) * portions * size_factor);
        let yeast_type = yeast.1.to_string();
        let total_weight =
            Some(flour.unwrap() + water.unwrap() + salt.unwrap() + yeast_weight.unwrap());
        let portion_weight = Some(total_weight.unwrap() / portions);

        PizzaDough {
            portions,
            size,
            flour,
            water,
            salt,
            yeast: yeast_weight,
            yeast_type,
            total_weight,
            portion_weight,
            measurement_unit: "g".to_string(),
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
        println!("Doug weight total: {}g", self.total_weight.unwrap());
        println!("Doug weight portion: {}g", self.portion_weight.unwrap());
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
