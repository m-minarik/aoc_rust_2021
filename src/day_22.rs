use regex::Regex;
use std::cmp::{min, max};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point3D {
    x: isize,
    y: isize,
    z: isize,
}

impl Point3D {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Point3D { x, y, z }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Box3D {
    min_corner: Point3D,
    max_corner: Point3D,
    on: bool,
}

impl Box3D {
    fn new(min_corner: Point3D, max_corner: Point3D, on: bool) -> Self {
        Box3D { min_corner, max_corner , on}
    }

    fn volume(&self) -> isize {
        (self.max_corner.x - self.min_corner.x + 1) *
        (self.max_corner.y - self.min_corner.y + 1) *
        (self.max_corner.z - self.min_corner.z + 1)
    }

    // Method to check if two boxes intersect
    fn intersects(&self, other: &Box3D) -> bool {
        self.min_corner.x <= other.max_corner.x && self.max_corner.x >= other.min_corner.x &&
        self.min_corner.y <= other.max_corner.y && self.max_corner.y >= other.min_corner.y &&
        self.min_corner.z <= other.max_corner.z && self.max_corner.z >= other.min_corner.z
    }

    // Compute the intersection of two boxes
    fn intersection(&self, other: &Box3D) -> Option<Box3D> {
        if !self.intersects(other) {
            return None;
        }
        let min_x = max(self.min_corner.x, other.min_corner.x);
        let min_y = max(self.min_corner.y, other.min_corner.y);
        let min_z = max(self.min_corner.z, other.min_corner.z);

        let max_x = min(self.max_corner.x, other.max_corner.x);
        let max_y = min(self.max_corner.y, other.max_corner.y);
        let max_z = min(self.max_corner.z, other.max_corner.z);

        Some(Box3D::new(Point3D::new(min_x, min_y, min_z), Point3D::new(max_x, max_y, max_z), other.on))
    }

    // Subtract the intersection from the current box and return remaining boxes
    fn subtract(&self, other: &Box3D) -> Vec<Box3D> {
        // Calculate the intersection first
        let intersection = match self.intersection(other) {
            Some(intersect) => intersect,
            None => return vec![self.clone()], // No intersection, return original box
        };

        let mut result = Vec::new();

        // Create new boxes for each remaining region that does not overlap with `other`
        // Split along x-axis
        if self.min_corner.x < intersection.min_corner.x {
            result.push(Box3D::new(
                Point3D::new(self.min_corner.x, self.min_corner.y, self.min_corner.z),
                Point3D::new(intersection.min_corner.x - 1, self.max_corner.y, self.max_corner.z),
                self.on,
            ));
        }
        if self.max_corner.x > intersection.max_corner.x {
            result.push(Box3D::new(
                Point3D::new(intersection.max_corner.x + 1, self.min_corner.y, self.min_corner.z),
                Point3D::new(self.max_corner.x, self.max_corner.y, self.max_corner.z),
                self.on,
            ));
        }

        // Split along y-axis
        if self.min_corner.y < intersection.min_corner.y {
            result.push(Box3D::new(
                Point3D::new(intersection.min_corner.x, self.min_corner.y, self.min_corner.z),
                Point3D::new(intersection.max_corner.x, intersection.min_corner.y - 1, self.max_corner.z),
                self.on,
            ));
        }
        if self.max_corner.y > intersection.max_corner.y {
            result.push(Box3D::new(
                Point3D::new(intersection.min_corner.x, intersection.max_corner.y + 1, self.min_corner.z),
                Point3D::new(intersection.max_corner.x, self.max_corner.y, self.max_corner.z),
                self.on,
            ));
        }

        // Split along z-axis
        if self.min_corner.z < intersection.min_corner.z {
            result.push(Box3D::new(
                Point3D::new(intersection.min_corner.x, intersection.min_corner.y, self.min_corner.z),
                Point3D::new(intersection.max_corner.x, intersection.max_corner.y, intersection.min_corner.z - 1),
                self.on,
            ));
        }
        if self.max_corner.z > intersection.max_corner.z {
            result.push(Box3D::new(
                Point3D::new(intersection.min_corner.x, intersection.min_corner.y, intersection.max_corner.z + 1),
                Point3D::new(intersection.max_corner.x, intersection.max_corner.y, self.max_corner.z),
                self.on,
            ));
        }

        result
    }
}

fn parse_box(s: &str, re: &Regex, filter: Option<&Box3D>) -> Option<Box3D> {
    let captures = re.captures(s).unwrap();

    let command: &str = &captures[1];
    let x1: isize = captures[2].parse::<isize>().unwrap();
    let x2: isize = captures[3].parse::<isize>().unwrap();
    let y1: isize = captures[4].parse::<isize>().unwrap();
    let y2: isize = captures[5].parse::<isize>().unwrap();
    let z1: isize = captures[6].parse::<isize>().unwrap();
    let z2: isize = captures[7].parse::<isize>().unwrap();

    let min_corner = Point3D::new(x1, y1, z1);
    let max_corner = Point3D::new(x2, y2, z2);
    
    if let Some(f) = filter {
        f.intersection(&Box3D::new(min_corner, max_corner, command == "on"))
    } else {
        Some(Box3D::new(min_corner, max_corner, command == "on"))
    }
}

pub fn part_1(input: &str) -> isize {   

    let re: Regex = Regex::new(r"(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)").unwrap();

    let min_corner: Point3D = Point3D::new(-50, -50, -50);
    let max_corner: Point3D = Point3D::new(50, 50, 50);

    let filter: Box3D = Box3D::new(min_corner, max_corner, false);

    let command_boxes: Vec<Box3D> = input
        .lines()
        .filter_map(|line| parse_box(&line, &re, Some(&filter)))
        .collect();

    let mut boxes: Vec<Box3D> = vec![command_boxes.first().unwrap().clone()];

    for command_box in &command_boxes[1..command_boxes.len()] {

        let mut boxes_next: Vec<Box3D> = boxes.iter()
                                                .flat_map(|b| b.subtract(command_box))
                                                .collect();

        boxes_next.push(command_box.clone());
        boxes = boxes_next;
    }

    boxes.iter()
            .filter(|b| b.on)
            .fold(0, |acc, b| acc + b.volume()) as isize
}

pub fn part_2(input: &str) -> isize {
    let re = Regex::new(r"(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)").unwrap();
 
    let command_boxes: Vec<Box3D> = input
        .lines()
        .filter_map(|line| parse_box(&line, &re, None))
        .collect();

    let mut boxes: Vec<Box3D> = vec![command_boxes.first().unwrap().clone()];

    for command_box in &command_boxes[1..command_boxes.len()] {

        let mut boxes_next: Vec<Box3D> = boxes.iter()
                                              .flat_map(|b| b.subtract(command_box))
                                              .collect();

        boxes_next.push(command_box.clone());
        boxes = boxes_next;
    }

    boxes.iter()
         .filter(|b| b.on)
         .fold(0, |acc, b| acc + b.volume()) as isize
}