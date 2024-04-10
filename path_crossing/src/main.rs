// # 1496 Given a string path, where path[i] = 'N', 'S', 'E' or 'W', each representing moving one unit north, south, east, or west, respectively. You start at the origin (0, 0) on a 2D plane and walk on the path specified by path.

// Return true if the path crosses itself at any point, that is, if at any time you are on a location you have previously visited. Return false otherwise.

fn main() {
    let path = String::from("NEEEEEEENWWWWS");

    let result = Solution::is_path_crossing(path);

    println!("result = {}", result);
}

struct Solution;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {

        let mut path_vec = Vec::new();

        let mut cords = (0,0);

        let mut cord_vec = vec![(0,0)];

        for c in path.chars() {
            path_vec.push(c);
        }

        for ch in path_vec.iter() {
            match ch {
                'N' => cords.1 += 1,
                'S' => cords.1 -= 1,
                'E' => cords.0 += 1,
                'W' => cords.0 -= 1,
                _ => panic!("Invalid character '{}'", ch)
            }

            if cord_vec.contains(&cords) {
                return true;
            }
            
            cord_vec.push(cords);
        }

        false
    }
}