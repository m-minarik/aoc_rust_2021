use std::collections::HashSet;

const OCCURENCE_MAP_LEN: usize = 20_000;
const OCCURENCE_MAP_OFFSET: isize = 10_000;

struct ScannerMeasurement {
    data: Vec<Vec<i16>>,
    axis_permutation: Vec<usize>,
}

pub fn part_1(input: &str) -> usize {
    let mut measurements: Vec<ScannerMeasurement> = parse_input(input);
    let (beacons, _scanners) = align_scans(&mut measurements);
    
    beacons.len()
}

pub fn part_2(input: &str) -> usize {
    let mut measurements: Vec<ScannerMeasurement> = parse_input(input);
    let (_beacons, scanners) = align_scans(&mut measurements);

    scanners.iter().map(|a| -> usize {
        scanners.iter().map(|b| -> usize {
            compute_manhattan_distance(a, b)
        }).max().unwrap()
    }).max().unwrap()
}


fn parse_input(input: &str) -> Vec<ScannerMeasurement> {
    let mut result: Vec<ScannerMeasurement> = vec![];

    for line in input.lines().into_iter() {
        // Save the previous scanner reading and proceed
        if line.is_empty() {
            continue;
        }

        if line.starts_with("---"){
            result.push(ScannerMeasurement{
                    data: vec![vec![], vec![], vec![]],
                    axis_permutation: vec![0, 1, 2],
            });
            continue;
        }

        // Parse one comma-separated line
        let parsed_line: Vec<i16> = 
            line
            .split(',')
            .map(|v| v.parse::<i16>().unwrap())
            .collect::<Vec<i16>>();

        result.last_mut().unwrap().data[0].push(parsed_line[0]);
        result.last_mut().unwrap().data[1].push(parsed_line[1]);
        result.last_mut().unwrap().data[2].push(parsed_line[2]);
    }

    result
}

fn compute_manhattan_distance(a: &[i16; 3], b: &[i16; 3]) -> usize {
    let mut res: i16 = 0;
    for i in 0..3{
        res += (a[i] - b[i]).abs();
    }

    res as usize
}

fn align_axis(a: &[i16], b: &[i16], occurence_count: &mut[u8; OCCURENCE_MAP_LEN], occurence_count_flipped: &mut[u8; OCCURENCE_MAP_LEN]) -> Option<(i16, bool)> {

    occurence_count.fill(0);
    occurence_count_flipped.fill(0);

    for ai in a {
        for bj in b {

            {
                let key = (((ai-bj) as isize) + OCCURENCE_MAP_OFFSET) as usize;
                if occurence_count[key] == 11 {
                    return Some((ai-bj, false));
                }
                occurence_count[key] += 1;
            }
            

            {
                let key = (((ai+bj) as isize) + OCCURENCE_MAP_OFFSET) as usize;
                if occurence_count_flipped[key] == 11 {
                    return Some((ai+bj, true));
                }
                occurence_count_flipped[key] += 1;
            }
        }
    }

    None

}

fn align_scans(measurements: &mut [ScannerMeasurement]) -> (HashSet<[i16; 3]>, HashSet<[i16; 3]>) {
    let mut aligned: Vec<usize> = vec![0];
    let num_scanners: usize = measurements.len();

    let mut beacons_relative_to_0: HashSet<[i16; 3]> = HashSet::new();
    for i in 0..measurements[0].data[0].len() {
        let b: [i16; 3] = (
            measurements[0].data[0][i],
            measurements[0].data[1][i],
            measurements[0].data[2][i],
        ).into();
        beacons_relative_to_0.insert(b);
    }

    let mut scanners_relative_to_0: HashSet<[i16; 3]> = HashSet::new();
    scanners_relative_to_0.insert([0, 0, 0]);

    let s: HashSet<usize> = HashSet::from([0, 1, 2]);

    let mut occurence_count: [u8; OCCURENCE_MAP_LEN] = [0; OCCURENCE_MAP_LEN];
    let mut occurence_count_flipped: [u8; OCCURENCE_MAP_LEN] = [0; OCCURENCE_MAP_LEN];

    for i in 0..num_scanners {
        let src_id: usize = aligned[i];

        'outer: for dst_id in 0..num_scanners {
            if aligned.contains(&dst_id) {
                continue;
            }

            if src_id == dst_id {
                continue;
            }

            let src_perm = &measurements[src_id].axis_permutation;

            for &x_perm in s.iter() {
                if let Some((x_offset, x_flipped)) = align_axis(&measurements[src_id].data[src_perm[0]], &measurements[dst_id].data[x_perm], &mut occurence_count, &mut occurence_count_flipped){
                    
                    for &y_perm in s.difference(&HashSet::from([x_perm])) {
                        if let Some((y_offset, y_flipped)) = align_axis(&measurements[src_id].data[src_perm[1]], &measurements[dst_id].data[y_perm], &mut occurence_count, &mut occurence_count_flipped){
                            
                            for &z_perm in s.difference(&HashSet::from([x_perm, y_perm])) {
                                if let Some((z_offset, z_flipped)) = align_axis(&measurements[src_id].data[src_perm[2]], &measurements[dst_id].data[z_perm], &mut occurence_count, &mut occurence_count_flipped){
                                    measurements[dst_id].axis_permutation = vec![x_perm, y_perm, z_perm];

                                    let dst_perm = &measurements[dst_id].axis_permutation;
                                    let offsets: [i16; 3] = [x_offset, y_offset, z_offset];
                                    let flipped: [bool; 3] = [x_flipped, y_flipped, z_flipped];

                                    for i in 0..3 {
                                        let offset: i16 = offsets[i];
                                        let flip: i16 = match flipped[i] {
                                            true => -1,
                                            false => 1,
                                        };

                                        for j in 0..measurements[dst_id].data[0].len(){
                                            measurements[dst_id].data[dst_perm[i]][j] = flip * measurements[dst_id].data[dst_perm[i]][j] + offset;
                                        }
                                    }

                                    for j in 0..measurements[dst_id].data[0].len(){
                                        let x: i16 = measurements[dst_id].data[dst_perm[0]][j];
                                        let y: i16 = measurements[dst_id].data[dst_perm[1]][j];
                                        let z: i16 = measurements[dst_id].data[dst_perm[2]][j];

                                        beacons_relative_to_0.insert([x, y, z].into());
                                    }

                                    scanners_relative_to_0.insert(offsets);

                                    aligned.push(dst_id);
                                    continue 'outer;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    (beacons_relative_to_0, scanners_relative_to_0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 79);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 3621);
    }

    fn data() -> String {
"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14".to_string()
    }
}