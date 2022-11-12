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

pub struct PizzaDough {
    portions: f32,
    size: String,
    flour: f32,
    water: f32,
    salt: f32,
    yeast: f32,
    yeast_type: String,
}

impl PizzaDough {
    pub fn new(p: f32, s: String, y: String) -> Self {
        let base = 125.0;
        let s_f = match s.trim().to_uppercase().as_str() {
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
            flour: base * p * s_f,
            water: (base * 0.6) * p * s_f,
            salt: (base * 0.03) * p * s_f,
            yeast: (base * yeast.0) * p * s_f,
            yeast_type: yeast.1.to_string(),
        }
    }

    pub fn printout(self) {
        if self.portions > 1.0 {
            println!("For {} pizzas size {} you knead:", self.portions, self.size);
        } else {
            println!("For {} pizza size {} you knead:", self.portions, self.size);
        }
        println!("Flour:   {}g", self.flour);
        println!("Water: {}g", self.water);
        println!("Salt:   {}g", self.salt);
        println!("{}:   {}g", self.yeast_type, self.yeast);

        let ta = self.flour + self.water + self.salt + self.yeast;

        println!("Doug weight total: {}g", ta);
        println!("Doug weight portion: {}g", ta / self.portions);
    }
}

#[cfg(test)]
mod tests {
    use crate::PizzaDough;

    #[test]
    fn test_one_medium() {
        let dough = PizzaDough::new(1.0, String::from("m"), String::from("f"));
        assert_eq!(dough.flour, 125.0);
        assert_eq!(dough.water, 75.0);
        assert_eq!(dough.salt, 3.75);
        assert_eq!(dough.yeast, 0.125);
    }
}
