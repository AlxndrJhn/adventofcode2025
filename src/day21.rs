const ITEM_SHOP: &str = "
Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0

Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5

Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3
";

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ItemCategory {
    Weapon,
    Armor,
    Ring,
}

pub struct Item {
    category: ItemCategory,
    cost: usize,
    damage: usize,
    armor: usize,
}

pub struct Character {
    hit_points: isize,
    damage: usize,
    armor: usize,
}

pub fn get_shop_items() -> Vec<Item> {
    let mut items: Vec<Item> = Vec::new();
    let mut category = ItemCategory::Weapon;
    for line in ITEM_SHOP.trim().lines().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts[1] == "Cost" {
            match parts[0] {
                "Armor:" => category = ItemCategory::Armor,
                "Rings:" => category = ItemCategory::Ring,
                _ => {}
            }
            continue;
        }
        match parts.as_slice() {
            [_, cost, damage, armor] => {
                items.push(Item {
                    category: category,
                    cost: cost.parse().unwrap(),
                    damage: damage.parse().unwrap(),
                    armor: armor.parse().unwrap(),
                });
            }
            [_, _, cost, damage, armor] => {
                items.push(Item {
                    category: category,
                    cost: cost.parse().unwrap(),
                    damage: damage.parse().unwrap(),
                    armor: armor.parse().unwrap(),
                });
            }
            _ => {}
        }
    }
    items
}

pub fn battle(player: &Character, boss: &Character) -> bool {
    let mut player_hp = player.hit_points;
    let mut boss_hp = boss.hit_points;

    let player_damage = (player.damage as isize - boss.armor as isize).max(1);
    let boss_damage = (boss.damage as isize - player.armor as isize).max(1);

    loop {
        boss_hp -= player_damage;
        if boss_hp <= 0 {
            return true; // Player wins
        }
        player_hp -= boss_damage;
        if player_hp <= 0 {
            return false; // Boss wins
        }
    }
}

pub fn dfs(
    items: &Vec<Item>,
    index: usize,
    selected: &mut Vec<&Item>,
    player_hit_points: isize,
    boss: &Character,
    min_cost: &mut usize,
) {
    if index == items.len() {
        let total_cost: usize = selected.iter().map(|item| item.cost).sum();
        let total_damage: usize = selected.iter().map(|item| item.damage).sum();
        let total_armor: usize = selected.iter().map(|item| item.armor).sum();
        let player = Character {
            hit_points: player_hit_points,
            damage: total_damage,
            armor: total_armor,
        };
        if total_cost >= *min_cost {
            return;
        }
        if battle(&player, boss) {
            *min_cost = (*min_cost).min(total_cost);
        }
        return;
    }

    // Include the current item
    let next_item = &items[index];
    let has_no_weapon = selected
        .iter()
        .all(|item| !matches!(item.category, ItemCategory::Weapon));
    let has_zero_armor = selected
        .iter()
        .all(|item| !matches!(item.category, ItemCategory::Armor));
    let ring_count_lt_2 = selected
        .iter()
        .filter(|item| matches!(item.category, ItemCategory::Ring))
        .count()
        < 2;
    let mut can_add_item = false;
    match next_item.category {
        ItemCategory::Weapon => {
            if has_no_weapon {
                can_add_item = true;
            }
        }
        ItemCategory::Armor => {
            if has_zero_armor {
                can_add_item = true;
            }
        }
        ItemCategory::Ring => {
            if ring_count_lt_2 {
                can_add_item = true;
            }
        }
    }
    if can_add_item {
        let mut selected_copy: Vec<&Item> = selected.clone();
        selected_copy.push(&items[index]);
        dfs(
            items,
            index + 1,
            &mut selected_copy,
            player_hit_points,
            boss,
            min_cost,
        );
    }

    dfs(
        items,
        index + 1,
        selected,
        player_hit_points,
        boss,
        min_cost,
    );
}

pub fn get_lowest_cost(input: &str, player_hitpoints: isize) -> usize {
    let items = get_shop_items();
    let mut boss_stats = Character {
        hit_points: 0,
        damage: 0,
        armor: 0,
    };
    for line in input.trim().lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let value = parts[1].trim().parse().unwrap();
        if parts[0].trim() == "Hit Points" {
            boss_stats.hit_points = value;
        }
        if parts[0].trim() == "Damage" {
            boss_stats.damage = value as usize;
        }
        if parts[0].trim() == "Armor" {
            boss_stats.armor = value as usize;
        }
    }
    let mut min_cost = usize::MAX;
    let mut selected: Vec<&Item> = Vec::new();
    dfs(
        &items,
        0,
        &mut selected,
        player_hitpoints,
        &boss_stats,
        &mut min_cost,
    );
    min_cost
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> usize {
    get_lowest_cost(input, 100)
}

pub fn dfs_max_lost(
    items: &Vec<Item>,
    index: usize,
    selected: &mut Vec<&Item>,
    player_hit_points: isize,
    boss: &Character,
    max_cost: &mut usize,
) {
    if index == items.len() {
        let total_cost: usize = selected.iter().map(|item| item.cost).sum();
        let total_damage: usize = selected.iter().map(|item| item.damage).sum();
        let total_armor: usize = selected.iter().map(|item| item.armor).sum();
        let player = Character {
            hit_points: player_hit_points,
            damage: total_damage,
            armor: total_armor,
        };
        let has_no_weapon = selected
            .iter()
            .all(|item| !matches!(item.category, ItemCategory::Weapon));
        if has_no_weapon {
            return;
        }

        if total_cost <= *max_cost {
            return;
        }
        if !battle(&player, boss) {
            *max_cost = (*max_cost).max(total_cost);
        }
        return;
    }

    // Include the current item
    let next_item = &items[index];
    let has_no_weapon = selected
        .iter()
        .all(|item| !matches!(item.category, ItemCategory::Weapon));
    let has_zero_armor = selected
        .iter()
        .all(|item| !matches!(item.category, ItemCategory::Armor));
    let ring_count_lt_2 = selected
        .iter()
        .filter(|item| matches!(item.category, ItemCategory::Ring))
        .count()
        < 2;
    let mut can_add_item = false;
    match next_item.category {
        ItemCategory::Weapon => {
            if has_no_weapon {
                can_add_item = true;
            }
        }
        ItemCategory::Armor => {
            if has_zero_armor {
                can_add_item = true;
            }
        }
        ItemCategory::Ring => {
            if ring_count_lt_2 {
                can_add_item = true;
            }
        }
    }
    if can_add_item {
        let mut selected_copy: Vec<&Item> = selected.clone();
        selected_copy.push(&items[index]);
        dfs_max_lost(
            items,
            index + 1,
            &mut selected_copy,
            player_hit_points,
            boss,
            max_cost,
        );
    }

    dfs_max_lost(
        items,
        index + 1,
        selected,
        player_hit_points,
        boss,
        max_cost,
    );
}

pub fn get_highest_lost_cost(input: &str, player_hitpoints: isize) -> usize {
    let items = get_shop_items();
    let mut boss_stats = Character {
        hit_points: 0,
        damage: 0,
        armor: 0,
    };
    for line in input.trim().lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let value = parts[1].trim().parse().unwrap();
        if parts[0].trim() == "Hit Points" {
            boss_stats.hit_points = value;
        }
        if parts[0].trim() == "Damage" {
            boss_stats.damage = value as usize;
        }
        if parts[0].trim() == "Armor" {
            boss_stats.armor = value as usize;
        }
    }
    let mut max_cost = usize::MIN;
    let mut selected: Vec<&Item> = Vec::new();
    dfs_max_lost(
        &items,
        0,
        &mut selected,
        player_hitpoints,
        &boss_stats,
        &mut max_cost,
    );
    max_cost
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> usize {
    get_highest_lost_cost(input, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            get_lowest_cost(
                "   Hit Points: 12
                    Damage: 7
                    Armor: 2",
                8
            ),
            18
        );
    }
    #[test]
    fn example_2() {
        let player = Character {
            hit_points: 8,
            damage: 5,
            armor: 5,
        };
        let boss = Character {
            hit_points: 12,
            damage: 7,
            armor: 2,
        };
        assert!(battle(&player, &boss));
    }
}
