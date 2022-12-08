use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coordinates {
    fn from(coords: (usize, usize)) -> Self {
        Self {
            x: coords.0,
            y: coords.1,
        }
    }
}

#[derive(Debug)]
struct Tree {
    coords: Coordinates,
    height: TreeHeight,
}

impl Tree {
    fn top<'a>(&'a self, map: &'a Map) -> impl Iterator<Item = Tree> + 'a {
        (0..self.coords.y)
            .rev()
            .map(|y| (self.coords.x, y))
            .flat_map(|c| map.get_tree(c))
    }
    fn right<'a>(&'a self, map: &'a Map) -> impl Iterator<Item = Tree> + 'a {
        ((self.coords.x + 1)..map.width)
            .map(|x| (x, self.coords.y))
            .flat_map(|c| map.get_tree(c))
    }

    fn down<'a>(&'a self, map: &'a Map) -> impl Iterator<Item = Tree> + 'a {
        ((self.coords.y + 1)..map.height)
            .map(|y| (self.coords.x, y))
            .flat_map(|c| map.get_tree(c))
    }

    fn left<'a>(&'a self, map: &'a Map) -> impl Iterator<Item = Tree> + 'a {
        (0..self.coords.x)
            .rev()
            .map(|x| (x, self.coords.y))
            .flat_map(|c| map.get_tree(c))
    }

    fn visible(&self, map: &Map) -> bool {
        let los = |tree: Tree| tree.height < self.height;
        self.coords.x == 0
            || self.coords.y == 0
            || self.coords.x == map.width - 1
            || self.coords.y == map.height - 1
            || self.top(map).all(los)
            || self.right(map).all(los)
            || self.down(map).all(los)
            || self.left(map).all(los)
    }

    fn scenic_score(&self, map: &Map) -> usize {
        let visible = |tree: Tree| tree.height >= self.height;
        let add_one: fn(usize) -> usize = |n| n + 1;
        self.top(map)
            .position(visible)
            .map(add_one)
            .unwrap_or(self.coords.y)
            * self
                .right(map)
                .position(visible)
                .map(add_one)
                .unwrap_or(map.width - self.coords.x - 1)
            * self
                .down(map)
                .position(visible)
                .map(add_one)
                .unwrap_or(map.height - self.coords.y - 1)
            * self
                .left(map)
                .position(visible)
                .map(add_one)
                .unwrap_or(self.coords.x)
    }
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    trees: Vec<TreeHeight>,
}

impl Map {
    fn get_tree<C: Into<Coordinates>>(&self, at: C) -> Option<Tree> {
        let coords: Coordinates = at.into();
        self.trees
            .get((coords.y * self.width) + coords.x)
            .map(|height| Tree {
                coords: coords,
                height: *height,
            })
    }

    fn iter(&self) -> impl Iterator<Item = Tree> + '_ {
        (0..self.width)
            .cartesian_product(0..self.height)
            .flat_map(|(x, y)| self.get_tree((x, y)))
    }

    fn visible_trees(&self) -> usize {
        self.iter().filter(|tree| tree.visible(self)).count()
    }

    fn most_scenic_tree(&self) -> usize {
        self.iter()
            .map(|tree| tree.scenic_score(&self))
            .max()
            .expect("empty map")
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let width = lines.peek().map(|line| line.len()).ok_or(())?;
        let trees = lines
            .flat_map(|line| {
                line.chars()
                    .flat_map(|c| c.to_digit(10).and_then(|d| TreeHeight::try_from(d).ok()))
            })
            .collect_vec();
        let height = trees.len() / width;
        Ok(Self {
            width,
            height,
            trees,
        })
    }
}

type TreeHeight = u8;

pub fn main() {
    let map = Map::from_str(include_str!("data/day8")).unwrap();
    println!("{}", map.visible_trees());
    println!("{}", map.most_scenic_tree());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_visible_trees() {
        assert_eq!(21, Map::from_str(EXAMPLE).unwrap().visible_trees());
    }

    #[test]
    fn test_most_scenic() {
        let map = Map::from_str(EXAMPLE).unwrap();
        assert_eq!(8, map.most_scenic_tree());
    }
}
