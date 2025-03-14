use crate::components::Cell;
use bevy::prelude::{Component, IVec3, Reflect};
use std::ops::Deref;

const NEIGHBOR_COORDINATES: [IVec3; 6] = [
    IVec3::new(0, 1, -1),
    IVec3::new(1, 0, -1),
    IVec3::new(1, -1, 0),
    IVec3::new(0, -1, 1),
    IVec3::new(-1, 0, 1),
    IVec3::new(-1, 1, 0),
];

/// Hexagonal 2D cell. It has 6 neighbors and uses `IVec3` coordinates (Cubic
/// coordinates).
///
/// ````ascii
///               X
///             _____
///            /     \
///           /       \
///     ,----(  0,1,-1 )----.
///    /      \       /      \
///   /        \_____/        \
///   \ -1,1,0 /     \ 1,0,-1  /
///    \      /       \      /
///     )----(  0,0,0  )----(
///    /      \       /      \
///   /        \_____/        \
///   \ -1,0,1 /     \ 1,-1,0  /
/// Y  \      /       \      /  Z
///     `----(  0,-1,1 )----'
///           \       /
///            \_____/
///
/// [Moore]: https://en.wikipedia.org/wiki/Moore_neighborhood
#[derive(Debug, Clone, Component, Reflect)]
pub struct HexagonCell2d {
    /// The 2D cell coordinates
    pub coords: IVec3,
}

impl Deref for HexagonCell2d {
    type Target = IVec3;

    fn deref(&self) -> &Self::Target {
        &self.coords
    }
}

impl Cell for HexagonCell2d {
    type Coordinates = IVec3;

    #[inline]
    fn coords(&self) -> &Self::Coordinates {
        &self.coords
    }

    #[inline]
    fn neighbor_coordinates(&self) -> impl IntoIterator<Item = Self::Coordinates> {
        NEIGHBOR_COORDINATES.map(|c| c + *self.coords())
    }
}

impl HexagonCell2d {
    /// Instantiates a new cell with `coords` values
    #[must_use]
    #[inline]
    pub const fn new(coords: IVec3) -> Self {
        Self { coords }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_coordinates() {
        let cell = HexagonCell2d {
            coords: IVec3::new(10, 10, 10),
        };
        let neighbors = cell.neighbor_coordinates().into_iter().collect::<Vec<_>>();
        assert_eq!(
            neighbors,
            vec![
                IVec3::new(10, 11, 9),
                IVec3::new(11, 10, 9),
                IVec3::new(11, 9, 10),
                IVec3::new(10, 9, 11),
                IVec3::new(9, 10, 11),
                IVec3::new(9, 11, 10)
            ]
        );
    }

    #[test]
    fn correct_coordinates_negative() {
        let cell = HexagonCell2d {
            coords: IVec3::new(-10, 8, 5),
        };
        let neighbors = cell.neighbor_coordinates().into_iter().collect::<Vec<_>>();
        assert_eq!(
            neighbors,
            vec![
                IVec3::new(-10, 9, 4),
                IVec3::new(-9, 8, 4),
                IVec3::new(-9, 7, 5),
                IVec3::new(-10, 7, 6),
                IVec3::new(-11, 8, 6),
                IVec3::new(-11, 9, 5)
            ]
        );
    }

    #[test]
    fn correct_coordinates_origin() {
        let cell = HexagonCell2d {
            coords: IVec3::new(0, 0, 0),
        };
        let neighbors = cell.neighbor_coordinates().into_iter().collect::<Vec<_>>();
        assert_eq!(
            neighbors,
            vec![
                IVec3::new(0, 1, -1),
                IVec3::new(1, 0, -1),
                IVec3::new(1, -1, 0),
                IVec3::new(0, -1, 1),
                IVec3::new(-1, 0, 1),
                IVec3::new(-1, 1, 0),
            ]
        );
    }
}
