use std::cmp::max;

#[derive(Debug, Clone)]
struct SnailfishNumber {
    values: Vec<u16>,
    depths: Vec<u16>,
}

pub fn part_1(input: &str) -> () {

    let mut result: SnailfishNumber = SnailfishNumber{
        values: vec![],
        depths: vec![],
    };

    let mut first = true;

    for line in input.lines().into_iter() {
        let mut other: SnailfishNumber = parse_number(line);

        result.values.append(&mut other.values);
        result.depths.append(&mut other.depths);

        if !first {
            for i in 0..result.depths.len() {
                result.depths[i] += 1;
            }
    
            explode_split(&mut result);
        } else {
            first = false;
        }
    }

    let magnitude: u16 = compute_magnitude(&mut result);
    println!("{magnitude}");

}

pub fn part_2(input: &str) -> () {

    let mut numbers: Vec<SnailfishNumber> = vec![];

    for line in input.lines().into_iter() {
        numbers.push(parse_number(line));
    }

    let mut largest: u16 = 0;

    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }

            let mut a = numbers[i].clone();
            let mut b = numbers[j].clone();

            a.values.append(&mut b.values);
            a.depths.append(&mut b.depths);

            for i in 0..a.depths.len() {
                a.depths[i] += 1;
            }
    
            explode_split(&mut a);
            let magnitude: u16 = compute_magnitude(&mut a);

            largest = max(magnitude, largest);
            
        }
    }

    println!("{largest}");
}


fn parse_number(input: &str) -> SnailfishNumber {
    // Returns (values, depths)
    let mut values: Vec<u16> = vec![];
    let mut depths: Vec<u16> = vec![];
    let mut depth: u16 = 0;

    for c in input.chars() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' => continue,
            _ => {
                values.push(c.to_digit(10).unwrap() as u16);
                depths.push(depth);
            }
        }
    }

    SnailfishNumber{values, depths}
}

fn explode_split(n: &mut SnailfishNumber) -> () {
    'outer: loop {
        let mut i: usize = 0;
        while i < n.values.len() {
            // Explode
            if n.depths[i] > 4 {
                // Add first value to the left
                if i > 0 {
                    n.values[i-1] += n.values[i];
                }

                // Add second value to the right
                if i < n.values.len() - 2 {
                    n.values[i+2] += n.values[i+1];
                }

                n.values.remove(i);
                n.depths.remove(i);

                n.values[i] = 0;
                n.depths[i] -= 1;

            }
            i += 1;
        }

        i = 0;
        while i < n.values.len() {
            // Split
            if n.values[i] > 9 {
                let original_depth: u16 = n.depths[i];
                let original_value = n.values[i];

                let l: u16 = original_value >> 1;
                let r: u16 = l + (original_value & 1);
                n.values[i] = l;
                n.values.insert(i+1, r);

                n.depths[i] = original_depth + 1;
                n.depths.insert(i+1, original_depth + 1);
    
                continue 'outer;
            }
            i += 1;
        }

        break;
    }
}

fn compute_magnitude(n: &mut SnailfishNumber) -> u16 {

    for depth in (1..=4).rev() {
        let mut i: usize = 0;
        while i < n.values.len() {
            if n.depths[i] == depth {

                n.values[i] = 3 * n.values[i] + 2 * n.values[i+1];
                n.depths[i] -= 1;

                n.values.remove(i+1);
                n.depths.remove(i+1);
            }
            i += 1;
        }
    }

    assert!(n.values.len() == 1);
    n.values[0]
}
