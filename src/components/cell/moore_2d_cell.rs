use crate::components::Cell;
use bevy::prelude::{Component, IVec2, Reflect};
use std::ops::Deref;

const NEIGHBOR_COORDINATES: [IVec2; 8] = [
    // Left
    IVec2::new(-1, 0),
    // Top Left
    IVec2::new(-1, 1),
    // Top
    IVec2::new(0, 1),
    // Top Right
    IVec2::new(1, 1),
    // Right
    IVec2::new(1, 0),
    // Bottom Right
    IVec2::new(1, -1),
    // Bottom
    IVec2::new(0, -1),
    // Bottom Left
    IVec2::new(-1, -1),
];

/// [Moore] 2D cell. It has 8 neighbors and uses `IVec2` coordinates.
///
/// ```ascii
/// +-------+-------+-------+
/// |       |       |       |
/// | -1,1  |  0,1  |  1,1  |
/// |       |       |       |
/// +-------+-------+-------+
/// |       |       |       |
/// | -1,0  |  0,0  |  0,1  |
/// |       |       |       |
/// +-------+-------+-------+
/// |       |       |       |
/// | -1,-1 |  0,-1 |  1,-1 |
/// |       |       |       |
/// +-------+-------+-------+
/// ```
///
/// [Moore]: https://en.wikipedia.org/wiki/Moore_neighborhood
#[derive(Debug, Clone, Component, Reflect)]
pub struct MooreCell2d {
    /// The 2D cell coordinates
    pub coords: IVec2,
}

impl Deref for MooreCell2d {
    type Target = IVec2;

    fn deref(&self) -> &Self::Target {
        &self.coords
    }
}

impl Cell for MooreCell2d {
    type Coordinates = IVec2;

    #[inline]
    fn coords(&self) -> &Self::Coordinates {
        &self.coords
    }

    #[inline]
    fn neighbor_coordinates(&self) -> impl IntoIterator<Item = Self::Coordinates> {
        NEIGHBOR_COORDINATES.map(|c| c + *self.coords())
    }
}

impl MooreCell2d {
    /// Instantiates a new cell with `coords` values
    #[must_use]
    #[inline]
    pub const fn new(coords: IVec2) -> Self {
        Self { coords }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_coordinates() {
        let cell = MooreCell2d {
            coords: IVec2::new(10, 10),
        };
        let neighbors = cell.neighbor_coordinates().into_iter().collect::<Vec<_>>();
        assert_eq!(
            neighbors,
            vec![
                // Left
                IVec2::new(9, 10),
                // Top Left
                IVec2::new(9, 11),
                // Top
                IVec2::new(10, 11),
                // Top Right
                IVec2::new(11, 11),
                // Right
                IVec2::new(11, 10),
                // Bottom Right
                IVec2::new(11, 9),
                // Bottom
                IVec2::new(10, 9),
                // Bottom Left
                IVec2::new(9, 9),
            ]
        );
    }

    #[test]
    fn correct_coordinates_negative() {
        let cell = MooreCell2d {
            coords: IVec2::new(-10, 10),
        };
        let neighbors = cell.neighbor_coordinates().into_iter().collect::<Vec<_>>();
        assert_eq!(
            neighbors,
            vec![
                // Left
                IVec2::new(-11, 10),
                // Top Left
                IVec2::new(-11, 11),
                // Top
                IVec2::new(-10, 11),
                // Top Right
                IVec2::new(-9, 11),
                // Right
                IVec2::new(-9, 10),
                // Bottom Right
                IVec2::new(-9, 9),
                // Bottom
                IVec2::new(-10, 9),
                // Bottom Left
                IVec2::new(-11, 9),
            ]
        );
    }

    #[test]
    fn correct_coordinates_origin() {
        let cell = MooreCell2d {
            coords: IVec2::new(0, 0),
        };
        let neighbors = cell.neighbor_coordinates().into_iter().collect::<Vec<_>>();
        assert_eq!(
            neighbors,
            vec![
                // Left
                IVec2::new(-1, 0),
                // Top Left
                IVec2::new(-1, 1),
                // Top
                IVec2::new(0, 1),
                // Top Right
                IVec2::new(1, 1),
                // Right
                IVec2::new(1, 0),
                // Bottom Right
                IVec2::new(1, -1),
                // Bottom
                IVec2::new(0, -1),
                // Bottom Left
                IVec2::new(-1, -1),
            ]
        );
    }
}
