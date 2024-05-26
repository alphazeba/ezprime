use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::input::N;
const SMALLEST_PRIME: N = 2;

pub struct PrimeDigestor {
    discovered_prime_numbers: RefCell<HashMap<N,PrimeDigestResult>>,
}

impl PrimeDigestor {
    pub fn new() -> Self {
        Self {
            discovered_prime_numbers: RefCell::new(HashMap::new()),
        }
    }

    pub fn is_prime(&self, number: N) -> PrimeDigestResult {
        if self.discovered_prime_numbers.borrow().contains_key(&number) {
            return self.discovered_prime_numbers
                .borrow()
                .get(&number)
                .unwrap()
                .clone();
        }
        // a number is prime if anything divides into it.
        // the smallest number that counts is 2
        // the largets number thats possible is sqrt rounded down to a whole number
        let max_number = square_root(number);
        let mut current_number = max_number;
        while current_number >= SMALLEST_PRIME {
            if number % current_number == 0 { // divides
                let next_number = number / current_number;
                return self.is_prime(next_number).combine(&self.is_prime(current_number));
            }
            current_number -= 1;
        }
        return PrimeDigestResult::IsPrime(number);
    }
}

type Recipe = Rc<RecipeContents>;
type RecipeContents = HashMap<N,DigestNode>;

fn recipe_to_list(recipe: Recipe) -> Vec<DigestNode> {
    let mut nodes: Vec<DigestNode> = recipe.values().cloned().collect();
    nodes.sort_by(|a, b| a.prime_number.cmp(&b.prime_number));
    nodes
}

fn recipe_to_string(recipe: Recipe) -> String {
    recipe_to_list(recipe)
        .iter()
        .map(|node| node.to_string())
        .collect::<Vec<String>>()
        .join(" * ")
}

fn recipe_content_add(mut recipe_contents: RecipeContents, node: DigestNode) -> RecipeContents {
    if recipe_contents.contains_key(&node.prime_number) {
        recipe_contents
            .get_mut(&node.prime_number)
            .unwrap()
            .add(node.count);
    } else {
        recipe_contents.insert(node.prime_number, node);
    }
    recipe_contents
}

fn recipe_to_contents(recipe: &Recipe) -> RecipeContents {
    let mut map: RecipeContents = HashMap::new();
    for (n, node) in recipe.iter() {
        map.insert(n.clone(), node.clone());
    }
    map
}

fn recipe_combine(a: &Recipe, b: &Recipe) -> Recipe {
    let mut map: RecipeContents = recipe_to_contents(a);
    for (_, node) in b.iter() {
        map = recipe_content_add(map, node.clone());
    }
    Rc::new(map)
}

fn recipe_add(a: &Recipe, n: &N) -> Recipe {
    let mut map: RecipeContents = recipe_to_contents(a);
    map = recipe_content_add(map, DigestNode::new(n.clone()));
    Rc::new(map)
}

fn recipe_create(an: &N, bn: &N) -> Recipe {
    let mut map: RecipeContents = HashMap::new();
    map = recipe_content_add(map, DigestNode::new(an.clone()));
    map = recipe_content_add(map, DigestNode::new(bn.clone()));
    Rc::new(map)
}

#[derive(Clone, Debug)]
pub enum PrimeDigestResult {
    IsPrime(N),
    NotPrime(N,Recipe),
}

impl PrimeDigestResult {
    fn combine(&self, other: &Self) -> Self {
        let new_n = self.get_n() * other.get_n();
        Self::NotPrime(new_n, match self {
            Self::IsPrime(self_n) => match other {
                Self::IsPrime(other_n) => recipe_create(self_n, other_n),
                Self::NotPrime(_, other_recipe) => 
                    recipe_add(other_recipe, self_n),
            },
            Self::NotPrime(_, self_recipe) => match other {
                Self::IsPrime(other_n) => recipe_add(self_recipe, other_n),
                Self::NotPrime(_, other_recipe) => 
                    recipe_combine(self_recipe, other_recipe),
            },
        })
    }

    fn get_n(&self) -> N {
        match self {
            Self::IsPrime(n) => n.clone(),
            Self::NotPrime(n, _) => n.clone(),
        }
    }

    pub fn to_string(self) -> String {
        match self {
            Self::IsPrime(n) => format!("{} is prime.", n),
            Self::NotPrime(n, recipe) =>
                format!("{} is NOT prime. \nRecipe: {}", n, recipe_to_string(recipe)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DigestNode {
    count: u8,
    prime_number: N,
}

impl DigestNode {
    fn new(prime_number: N) -> Self {
        Self {
            count: 1,
            prime_number,
        }
    }

    fn add(&mut self, val: u8) {
        self.count += val;
    }

    fn to_string(&self) -> String {
        format!("{}^{}", self.prime_number, self.count)
    }
}

fn square_root(x: N) -> N {
    (x as f64)
        .sqrt()
        .trunc() as N
}