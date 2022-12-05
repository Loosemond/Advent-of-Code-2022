use std::fs;
use std::str;

struct Elves {
    pub food_calories: Vec<i32>,
}

impl Elves {
    fn total_calories(&self) -> i32 {
        return self.food_calories.iter().sum();
    }
}
impl std::fmt::Display for Elves {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.food_calories)
    }
}

fn main() {
    println!("Day 1");
    let file_path = "./input-file";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut elves: Vec<Elves> = Vec::new();

    let v: Vec<&str> = contents.rsplit("\n\n").collect();
    // println!("With text:\n{:?}", v);
    for i in &v {
        let i: &str = i;
        let v2: Vec<&str> = i.rsplit('\n').collect();
        let mut food = Elves {
            food_calories: Vec::new(),
        };
        for j in &v2 {
            let j = j;
            let calories = j.parse::<i32>();
            match calories {
                Ok(v) => food.food_calories.push(v),
                Err(e) => println!("error parsing string: {e:?}, {j:?}"),
            }

            // println!("{}", calories)
        }
        elves.push(food);
    }

    // for k in &elves {
    //     println!("{}", k.total_calories());
    // }
    // let total = elves.iter_mut().flat_map(|e| e.total_calories());

    // println!("{:?}", total.iter().max());

    let mut total: Vec<i32> = Vec::new();
    for l in elves {
        total.push(l.total_calories());
    }
    let maxValue = total.iter().max();
    match maxValue {
        Some(max) => println!("Max value: {}", max),
        None => println!("Vector is empty"),
    }
    total.sort();
    let t: i32 = total.iter().rev().take(3).sum();
    println!("Sum of the top 3: {}", t);
}
