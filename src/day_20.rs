
type ImageEnhancementAlgorithm = Vec<bool>;
type Image = Vec<Vec<bool>>;

pub fn part_1(input: &str) -> usize {

    let (mut img, iea): (Image, ImageEnhancementAlgorithm) = parse_input(input);
    
    // Test
    // img = enhance(&img, &iea, false);
    // img = enhance(&img, &iea, false);

    // Input
    img = enhance(&img, &iea, false);
    img = enhance(&img, &iea, true);

    img.iter()
       .fold(0, |acc, e| acc + 
            e
            .iter()
            .fold(0, |acc_inner, e_inner| acc_inner + *e_inner as usize)
        )
}

pub fn part_2(input: &str) -> usize {
    let (mut img, iea): (Image, ImageEnhancementAlgorithm) = parse_input(input);
    
    // Test
    // for _ in 0..50 {
    //     img = enhance(&img, &iea, false);
    // }

    // Input
    for i in 0..50 {
        img = enhance(&img, &iea, (i % 2) == 1);
    }

    img.iter()
       .fold(0, |acc, e| acc + 
            e
            .iter()
            .fold(0, |acc_inner, e_inner| acc_inner + *e_inner as usize)
        )
}


fn parse_input(input: &str) -> (Image, ImageEnhancementAlgorithm) {

    let mut img: Image = vec![];
    let mut iea: ImageEnhancementAlgorithm = vec![];

    let mut parsing_algorithm: bool = true;
    for line in input.lines() {
        if line == "" {
            parsing_algorithm = false;
        }
        else if parsing_algorithm {
            iea.extend(line.chars().map(|c| c == '#'));
        } else {            
            img.push(line.chars().map(|c| c == '#').collect());
        }
    }


    (img, iea)
}

fn add_borders(img_in: &Image, pad_value: bool) -> Image {

    let w_in: usize = img_in[0].len();    
    let mut img_out: Image = vec![];

    for _ in 0..2 {
        img_out.push(vec![pad_value; w_in + 4]);
    }
    
    for row_in in img_in {
        let mut row_out: Vec<bool> = vec![pad_value; 2];
        row_out.extend(row_in);
        row_out.extend(vec![pad_value; 2]);

        img_out.push(row_out);
    }
    
    for _ in 0..2 {
        img_out.push(vec![pad_value; w_in + 4]);
    }

    img_out
}

fn enhance(img: &Image, iea: &ImageEnhancementAlgorithm, pad_value: bool) -> Image {

    let h: usize = img.len();
    let w: usize = img[0].len();

    // Pad the image with two values on each side
    let img_in: Image = add_borders(img, pad_value);

    // Prepare one additional value on each side
    let mut img_out: Image = vec![vec![false; w + 2]; h + 2];

    for row in 1..=(h+2) {
        for col in 1..=(w+2) {
            let mut idx: usize = 0;

            idx = idx | ((img_in[row-1] [col-1]    as usize) << 8);
            idx = idx | ((img_in[row-1]   [col]    as usize) << 7);
            idx = idx | ((img_in[row-1] [col+1]    as usize) << 6);
            idx = idx | ((img_in  [row] [col-1]    as usize) << 5);
            idx = idx | ((img_in  [row]   [col]    as usize) << 4);
            idx = idx | ((img_in  [row] [col+1]    as usize) << 3);
            idx = idx | ((img_in[row+1] [col-1]    as usize) << 2);
            idx = idx | ((img_in[row+1]   [col]    as usize) << 1);
            idx = idx | ((img_in[row+1] [col+1]    as usize));

            img_out[row-1][col-1] = iea[idx];
        }
    }
    img_out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 35);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 3351);
    }

    fn data() -> String {
"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###".to_string()
    }
}