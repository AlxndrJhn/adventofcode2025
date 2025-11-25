pub fn is_real_room(room: &str) -> bool {
    let parts: Vec<&str> = room.rsplitn(2, '-').collect();
    let (name_and_sector, checksum_part) = (parts[1], parts[0]);
    let (_, checksum_str) = checksum_part.split_at(checksum_part.len() - 7);
    let checksum = &checksum_str[1..6];
    let name = name_and_sector.replace("-", "");
    let mut letter_counts = std::collections::HashMap::new();
    for ch in name.chars() {
        *letter_counts.entry(ch).or_insert(0) += 1;
    }
    let mut letters: Vec<(char, usize)> = letter_counts.into_iter().collect();
    letters.sort_unstable_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let calculated_checksum: String = letters.iter().take(5).map(|&(ch, _)| ch).collect();
    calculated_checksum == checksum
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| is_real_room(line))
        .map(|line| {
            let parts: Vec<&str> = line.rsplitn(2, '-').collect();
            let sector_id_str = &parts[0][..parts[0].len() - 7];
            sector_id_str.parse::<usize>().unwrap()
        })
        .sum()
}

pub fn decrypt_name(name: &str, sector_id: usize) -> String {
    name.chars()
        .map(|ch| {
            if ch == '-' {
                ' '
            } else {
                let shifted = ((ch as u8 - b'a' + (sector_id % 26) as u8) % 26) + b'a';
                shifted as char
            }
        })
        .collect()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    for line in input.lines() {
        if is_real_room(line) {
            let parts: Vec<&str> = line.rsplitn(2, '-').collect();
            let sector_id_str = &parts[0][..parts[0].len() - 7];
            let sector_id = sector_id_str.parse::<usize>().unwrap();
            let name_part = parts[1];
            let decrypted_name = decrypt_name(name_part, sector_id);
            if decrypted_name.contains("northpole") {
                return sector_id;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(is_real_room("aaaaa-bbb-z-y-x-123[abxyz]"));
    }
    #[test]
    fn example_2() {
        assert!(is_real_room("a-b-c-d-e-f-g-h-987[abcde]"));
    }
    #[test]
    fn example_3() {
        assert!(is_real_room("not-a-real-room-404[oarel]"));
    }
    #[test]
    fn example_4() {
        assert!(!is_real_room("totally-real-room-200[decoy]"));
    }
    #[test]
    fn example_5() {
        assert!(decrypt_name("qzmt-zixmtkozy-ivhz", 343) == "very encrypted name");
    }
}
