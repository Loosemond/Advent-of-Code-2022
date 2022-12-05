use std::fs;
use std::str;

fn main() {
    println!("Day 4");
    let input = fs::read_to_string("./input-file").expect("Failed to load input file");

    let elf_pair: Vec<&str> = input.rsplit('\n').rev().collect::<Vec<&str>>();
    let mut overlap_count_p1: u32 = 0;
    let mut overlap_count_p2: u32 = 0;

    for elf in elf_pair {
        let groups: Vec<&str> = elf.rsplit(',').rev().collect();
        let mut parsed_group: Vec<Vec<u32>> = Vec::new();
        for group in groups {
            let elf_area: Vec<u32> = group
                .rsplit('-')
                .rev()
                .map(|s| s.parse::<u32>().expect("Failed to parse"))
                .collect();
            parsed_group.push(elf_area);
        }
        // if parsed_group[0][0] <= parsed_group[1][0] && parsed_group[0][1] >= parsed_group[1][1]
        //     || parsed_group[1][0] <= parsed_group[0][0] && parsed_group[1][1] >= parsed_group[0][1]
        // {
        //     // println!("match: {parsed_group:?}");

        //     overlap_count_p1 += 1;
        // }
        // print!("overlap_count_p1 : {overlap_count_p1}");
        if parsed_group[0][0] <= parsed_group[1][0] && parsed_group[0][1] >= parsed_group[1][1]
            || parsed_group[1][0] <= parsed_group[0][0] && parsed_group[1][1] >= parsed_group[0][1]
            || parsed_group[0][0] <= parsed_group[1][1] && parsed_group[0][1] >= parsed_group[1][0]
        {
            println!("match: {parsed_group:?}");

            overlap_count_p2 += 1;
        }
    }
    print!("overlap_count_p2 : {overlap_count_p2}");

    // let elf_pair: Vec<&str> = input.rsplit('\n').collect();
    // println!("{elf_pair:?}")
}
