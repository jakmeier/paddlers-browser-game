use super::{Town, X, Y};
use paddlers_shared_lib::game_mechanics::town::TileIndex;
use pathfinding::prelude::{absdiff, astar};

impl Town {
    pub fn shortest_path(&self, s: TileIndex, t: TileIndex) -> Option<(Vec<TileIndex>, u32)> {
        let successors = |v: &TileIndex| self.successors(*v);
        let success = |v: &TileIndex| *v == t;
        let heuristic = |v: &TileIndex| (absdiff(v.0, t.0) + absdiff(v.1, t.1)) as u32;
        astar(&s, successors, heuristic, success)
    }

    /// Find the tile on the town screen that is closest to the start point while also in the defined, rectified circular area
    pub fn closest_walkable_tile_in_range(
        &self,
        start: TileIndex,
        destination: TileIndex,
        radius: f32,
    ) -> Option<TileIndex> {
        let valid = self.tiles_in_rectified_circle(destination, radius);

        let successors = |v: &TileIndex| self.successors(*v);
        let success = |v: &TileIndex| valid.contains(v);
        let heuristic =
            |v: &TileIndex| (absdiff(v.0, destination.0) + absdiff(v.1, destination.1)) as u32;
        let path = astar(&start, successors, heuristic, success);
        path.and_then(|p| p.0.last().cloned())
    }

    fn successors(&self, index: TileIndex) -> Vec<(TileIndex, u32)> {
        let (x, y) = index;
        let mut nbrs = vec![];

        if x + 1 < X {
            nbrs.push((x + 1, y));
        }
        if y + 1 < Y {
            nbrs.push((x, y + 1));
        }
        if x > 0 {
            nbrs.push((x - 1, y));
        }
        if y > 0 {
            nbrs.push((x, y - 1));
        }
        nbrs.into_iter()
            .filter(|idx| self.is_walkable(*idx))
            .map(|idx| (idx, 1))
            .collect()
    }
}
