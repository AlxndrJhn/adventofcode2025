struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    let re = regex::Regex::new(r"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();
    let ingredients: Vec<Ingredient> = re
        .captures_iter(input)
        .map(|cap| Ingredient {
            capacity: cap[2].parse().unwrap(),
            durability: cap[3].parse().unwrap(),
            flavor: cap[4].parse().unwrap(),
            texture: cap[5].parse().unwrap(),
        })
        .collect();

    fn score(ingredients: &Vec<Ingredient>, amounts: &Vec<isize>) -> isize {
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;

        for (i, ingredient) in ingredients.iter().enumerate() {
            capacity += ingredient.capacity * amounts[i];
            durability += ingredient.durability * amounts[i];
            flavor += ingredient.flavor * amounts[i];
            texture += ingredient.texture * amounts[i];
        }

        capacity = capacity.max(0);
        durability = durability.max(0);
        flavor = flavor.max(0);
        texture = texture.max(0);

        capacity * durability * flavor * texture
    }

    fn dfs(
        ingredients: &Vec<Ingredient>,
        amounts: &mut Vec<isize>,
        index: usize,
        remaining: isize,
        max_score: &mut isize,
    ) {
        if index == ingredients.len() - 1 {
            amounts[index] = remaining;
            let s = score(ingredients, amounts);
            if s > *max_score {
                *max_score = s;
            }
            return;
        }

        for amt in 0..=remaining {
            amounts[index] = amt;
            dfs(ingredients, amounts, index + 1, remaining - amt, max_score);
        }
    }
    let mut amounts = vec![0; ingredients.len()];
    let mut max_score = 0;
    dfs(&ingredients, &mut amounts, 0, 100, &mut max_score);
    max_score as usize
}

// #[aoc(day15, part2)]
// pub fn part2(input: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            part1(
                "
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
"
                .trim()
            ),
            62842880
        );
    }
}
