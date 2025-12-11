use std::collections::HashMap;

fn main() {
    let input = include_str!("../input").trim_end();
    parts(&input);
}

fn distance(x1: i64, y1: i64, z1: i64, x2: i64, y2: i64, z2: i64) -> f64 {
    return f64::sqrt(((x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)) as f64);
}

fn parts(input: &str) {
    let mut coord_distances: HashMap<(&str, &str), f64> = HashMap::new();

    let mut circuits: Vec<Vec<&str>> = vec![];
    let mut circuit_indices: HashMap<&str, usize> = HashMap::new();

    for coordinate1 in input.lines() {
        let c_i = circuits.len();
        circuits.push(vec![coordinate1]);
        circuit_indices.insert(coordinate1, c_i);

        let vals1 = coordinate1.split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let (x1, y1, z1) = (vals1[0], vals1[1], vals1[2]);
        for coordinate2 in input.lines() {
            if coordinate1 != coordinate2 {
                let vals2 = coordinate2.split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
                let (x2, y2, z2) = (vals2[0], vals2[1], vals2[2]);
                if !coord_distances.contains_key(&(coordinate1, coordinate2)) && !coord_distances.contains_key(&(coordinate2, coordinate1)) {
                    coord_distances.insert((coordinate1, coordinate2), distance(x1, y1, z1, x2, y2, z2));
                }
            }
        }
    }
    let mut sorted_coord_distances: Vec<(&(&str, &str), &f64)> = coord_distances.iter().collect();
    sorted_coord_distances.sort_by(|a, b| a.1.partial_cmp(b.1).expect("nan"));

    let mut num_connections = 0;
    let mut last_coord1 = "";
    let mut last_coord2 = "";
    for coord_distance in &sorted_coord_distances {
        let (coord1, coord2) = coord_distance.0;

        let i1 = *circuit_indices.get(coord1).unwrap();
        let i2 = *circuit_indices.get(coord2).unwrap();
        if i1 != i2 {
            last_coord1 = coord1;
            last_coord2 = coord2;
            let to_extend = circuits[i2].clone();
            circuits[i1].extend(to_extend);
            //move all indices from coords in i2
            for coord in &circuits[i2] {
                *circuit_indices.get_mut(coord).unwrap() = i1;
            }
            //empty the vec instead of removing so that other indices stay the same
            circuits[i2] = vec![];
        }
        if num_connections == 999 {
            let mut result = 1;
            let mut temp_circ = circuits.clone();
            temp_circ.sort_by(|a, b| b.len().cmp(&a.len()));
            for i in 0..3 {
                result *= temp_circ[i].len();
            }
            println!("Part 1: {}", result);
        }
        num_connections += 1;
    }

    let result = last_coord1.split(",").collect::<Vec<&str>>()[0].parse::<u64>().unwrap() * last_coord2.split(",").collect::<Vec<&str>>()[0].parse::<u64>().unwrap();
    println!("Part 2: {}", result);

}
