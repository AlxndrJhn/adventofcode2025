use core::panic;

#[derive(PartialEq, Eq, Debug)]
pub enum GameResult {
    PlayerWin,
    BossWin,
    NotEnoughSpells,
    NotEnoughMana,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SpellName {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Clone, Copy)]
pub struct Spell {
    name: SpellName,
    mana_cost: usize,
    damage: usize,
    heal: usize,
    armor: usize,
    mana_recharge: usize,
    duration: usize,
}

pub const MAGIC_MISSILE: Spell = Spell {
    name: SpellName::MagicMissile,
    mana_cost: 53,
    damage: 4,
    heal: 0,
    armor: 0,
    mana_recharge: 0,
    duration: 0,
};
pub const DRAIN: Spell = Spell {
    name: SpellName::Drain,
    mana_cost: 73,
    damage: 2,
    heal: 2,
    armor: 0,
    mana_recharge: 0,
    duration: 0,
};
pub const SHIELD: Spell = Spell {
    name: SpellName::Shield,
    mana_cost: 113,
    damage: 0,
    heal: 0,
    armor: 7,
    mana_recharge: 0,
    duration: 6,
};
pub const POISON: Spell = Spell {
    name: SpellName::Poison,
    mana_cost: 173,
    damage: 3,
    heal: 0,
    armor: 0,
    mana_recharge: 0,
    duration: 6,
};
pub const RECHARGE: Spell = Spell {
    name: SpellName::Recharge,
    mana_cost: 229,
    damage: 0,
    heal: 0,
    armor: 0,
    mana_recharge: 101,
    duration: 5,
};

pub struct Character {
    hit_points: isize,
    damage: usize,
    armor: usize,
    mana: isize,
    spell_sequence: Vec<Spell>,
    current_effects: Vec<Spell>,
}

pub fn battle(player: &mut Character, boss: &mut Character, verbose: bool) -> GameResult {
    for round in 0.. {
        if verbose {
            println!(
                "-- Player turn --\n- Player has {} hit points, {} armor, {} mana\n- Boss has {} hit points",
                player.hit_points, player.armor, player.mana, boss.hit_points
            );
        }
        process_current_effects(player, verbose);
        process_current_effects(boss, verbose);
        if boss.hit_points <= 0 {
            if verbose {
                println!("Boss is defeated!");
            }
            return GameResult::PlayerWin;
        }
        let spell = player.spell_sequence.get(round);
        if spell.is_none() {
            return GameResult::NotEnoughSpells;
        }
        let spell = spell.unwrap();
        player.mana -= spell.mana_cost as isize;
        if player.mana < 0 {
            return GameResult::NotEnoughMana;
        }
        match spell.name {
            SpellName::MagicMissile => {
                if verbose {
                    println!(
                        "Player casts Magic Missile, dealing {} damage.",
                        spell.damage
                    );
                }
                boss.hit_points -= spell.damage as isize;
            }
            SpellName::Drain => {
                if verbose {
                    println!(
                        "Player casts Drain, dealing {} damage and healing {} hit points.",
                        spell.damage, spell.heal
                    );
                }
                boss.hit_points -= spell.damage as isize;
                player.hit_points += spell.heal as isize;
            }
            SpellName::Shield => {
                if verbose {
                    println!("Player casts Shield.");
                }
                player.current_effects.push(spell.clone());
            }
            SpellName::Poison => {
                if verbose {
                    println!("Player casts Poison.");
                }
                boss.current_effects.push(spell.clone());
            }
            SpellName::Recharge => {
                if verbose {
                    println!("Player casts Recharge.");
                }
                player.current_effects.push(spell.clone());
            }
        }
        if boss.hit_points <= 0 {
            if verbose {
                println!("Boss is defeated!");
            }
            return GameResult::PlayerWin;
        }

        if verbose {
            println!(
                "\n-- Boss turn --\n- Player has {} hit points, {} armor, {} mana\n- Boss has {} hit points",
                player.hit_points, player.armor, player.mana, boss.hit_points
            );
        }
        process_current_effects(player, verbose);
        process_current_effects(boss, verbose);
        if boss.hit_points <= 0 {
            if verbose {
                println!("Boss is defeated!");
            }
            return GameResult::PlayerWin;
        }
        let boss_damage = (boss.damage as isize - player.armor as isize).max(1);
        if verbose {
            println!("Boss attacks for {} damage.", boss_damage);
        }
        player.hit_points -= boss_damage;
        if player.hit_points <= 0 {
            if verbose {
                println!("Player is defeated!");
            }
            return GameResult::BossWin;
        }
        if verbose {
            println!();
        }
    }
    panic!("Unreachable code reached in battle function");
}

fn process_current_effects(character: &mut Character, verbose: bool) {
    character.armor = 0;
    for effect_spell in character.current_effects.iter_mut() {
        if effect_spell.duration == 0 {
            panic!("Effect spell with zero duration in current effects");
        }
        match effect_spell.name {
            SpellName::Shield => {
                if verbose {
                    println!(
                        "Shield's armor increases by {}; its timer is now {}",
                        effect_spell.armor,
                        effect_spell.duration - 1
                    );
                }
                character.armor += effect_spell.armor;
            }
            SpellName::Poison => {
                if verbose {
                    println!(
                        "Poison deals {} damage; its timer is now {}",
                        effect_spell.damage,
                        effect_spell.duration - 1
                    );
                }
                character.hit_points -= effect_spell.damage as isize;
            }
            SpellName::Recharge => {
                if verbose {
                    println!(
                        "Recharge provides {} mana; its timer is now {}",
                        effect_spell.mana_recharge,
                        effect_spell.duration - 1
                    );
                }
                character.mana += effect_spell.mana_recharge as isize;
            }
            _ => {
                panic!("Invalid effect spell in current effects");
            }
        }
        effect_spell.duration -= 1;
    }
    character
        .current_effects
        .retain(|effect_spell| effect_spell.duration > 0);
}

pub fn dfs_min_mana(
    player_hp: isize,
    player_mana: isize,
    player_spell_sequence: Vec<Spell>,
    boss_hp: isize,
    boss_damage: usize,
    min_mana_used: &mut usize,
) {
    let mana_used: usize = player_spell_sequence.iter().map(|s| s.mana_cost).sum();
    if mana_used >= *min_mana_used {
        return;
    }
    let mut player = Character {
        hit_points: player_hp,
        damage: 0,
        armor: 0,
        mana: player_mana,
        spell_sequence: player_spell_sequence.clone(),
        current_effects: vec![],
    };
    let mut boss = Character {
        hit_points: boss_hp,
        damage: boss_damage,
        armor: 0,
        mana: 0,
        spell_sequence: vec![],
        current_effects: vec![],
    };
    match battle(&mut player, &mut boss, false) {
        GameResult::PlayerWin => {
            let mana_used: usize = player_spell_sequence.iter().map(|s| s.mana_cost).sum();
            if mana_used < *min_mana_used {
                *min_mana_used = mana_used;
            }
        }
        GameResult::BossWin => {
            return;
        }
        GameResult::NotEnoughMana => {
            return;
        }
        GameResult::NotEnoughSpells => {}
    }

    let mut available_next_spells = vec![MAGIC_MISSILE, DRAIN];
    for spell_to_check in &[SHIELD, POISON, RECHARGE] {
        let already_active = player_spell_sequence
            .iter()
            .rev()
            .take(3)
            .any(|s| s.name == spell_to_check.name);
        if !already_active {
            available_next_spells.push(*spell_to_check);
        }
    }

    for next_spell in available_next_spells {
        let mut new_player_spell_sequence = player_spell_sequence.clone();
        new_player_spell_sequence.push(next_spell);
        dfs_min_mana(
            player_hp,
            player_mana,
            new_player_spell_sequence,
            boss_hp,
            boss_damage,
            min_mana_used,
        );
    }
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> usize {
    let mut boss_hp = 0;
    let mut boss_damage = 0;
    for line in input.trim().lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let value = parts[1].trim().parse().unwrap();
        if parts[0].trim() == "Hit Points" {
            boss_hp = value;
        }
        if parts[0].trim() == "Damage" {
            boss_damage = value as usize;
        }
    }

    let mut least_mana_used = usize::MAX;
    dfs_min_mana(50, 500, vec![], boss_hp, boss_damage, &mut least_mana_used);
    least_mana_used
}

// #[aoc(day22, part2)]
// pub fn part2(input: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut player = Character {
            hit_points: 10,
            damage: 0,
            armor: 0,
            mana: 250,
            spell_sequence: vec![POISON, MAGIC_MISSILE],
            current_effects: vec![],
        };
        let mut boss = Character {
            hit_points: 13,
            damage: 8,
            armor: 0,
            mana: 0,
            spell_sequence: vec![],
            current_effects: vec![],
        };
        assert_eq!(battle(&mut player, &mut boss, false), GameResult::PlayerWin);
    }

    #[test]
    fn example_2() {
        let mut player = Character {
            hit_points: 10,
            damage: 0,
            armor: 0,
            mana: 250,
            spell_sequence: vec![RECHARGE, SHIELD, DRAIN, POISON, MAGIC_MISSILE],
            current_effects: vec![],
        };
        let mut boss = Character {
            hit_points: 14,
            damage: 8,
            armor: 0,
            mana: 0,
            spell_sequence: vec![],
            current_effects: vec![],
        };
        assert_eq!(battle(&mut player, &mut boss, false), GameResult::PlayerWin);
    }
}
