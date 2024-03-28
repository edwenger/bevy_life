use crate::components::Cell;
use bevy::prelude::{Component, IVec2, Reflect};
use std::ops::Deref;

const NEIGHBOR_COORDINATES: [IVec2; 12] = [
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

    // Dummy values (fn neighbor_coordinates expects same-size return values on match)
    IVec2::new(-9999, -9999),
    IVec2::new(-9999, -9999),
    IVec2::new(-9999, -9999),
    IVec2::new(-9999, -9999),
];

const R: i32 = 50; // dimension of long-range connectivity grid

const COMBO_COORDINATES: [IVec2; 12] = [
    IVec2::new(-1, 0),
    IVec2::new(-1, 1),
    IVec2::new(0, 1),
    IVec2::new(1, 1),
    IVec2::new(1, 0),
    IVec2::new(1, -1),
    IVec2::new(0, -1),
    IVec2::new(-1, -1),

    IVec2::new(-R, 0),
    IVec2::new(0, R),
    IVec2::new(R, 0),
    IVec2::new(0, -R),
];

/// [Complex] 2D cell. It has 8 Moore-cell neighbors + periodically 4 long-range contacts (uses `IVec2` coordinates).
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
/// ```ascii
///         +-------+
///         |       |
///         |  0,R  |  
///         |       |
/// +-------+-------+-------+
/// |       |       |       |
/// | -R,0  |  0,0  |  0,R  |
/// |       |       |       |
/// +-------+-------+-------+
///         |       |
///         |  0,-R |
///         |       |
///         +-------+
/// ```
/// [Moore]: https://en.wikipedia.org/wiki/Moore_neighborhood
/// [Neumann]: https://en.wikipedia.org/wiki/Von_Neumann_neighborhood
#[derive(Debug, Clone, Component, Reflect)]
pub struct ComplexCell2d {
    /// The 2D cell coordinates
    pub coords: IVec2,
}

impl Deref for ComplexCell2d {
    type Target = IVec2;

    fn deref(&self) -> &Self::Target {
        &self.coords
    }
}

impl Cell for ComplexCell2d {
    type Coordinates = IVec2;

    #[inline]
    fn coords(&self) -> &Self::Coordinates {
        &self.coords
    }

    #[inline]
    fn neighbor_coordinates(&self) -> impl IntoIterator<Item = Self::Coordinates> {
        let (x, y) = self.coords().to_array().into();
        match (x, y) {
            (x, y) if ((x%R == 0) && (y%R == 0)) => COMBO_COORDINATES.map(|c| c + *self.coords()),
            _ => NEIGHBOR_COORDINATES.map(|c| c + *self.coords())
        }
    }
}

impl ComplexCell2d {
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

    // #[test]
    // fn correct_coordinates() {
    //     let cell = ComplexCell2d {
    //         coords: IVec2::new(10, 10),
    //     };
    //     let neighbors = cell.neighbor_coordinates().into_iter().collect::<Vec<_>>();
    //     assert_eq!(
    //         neighbors,
    //         vec![
    //             // Left
    //             IVec2::new(9, 10),
    //             // Top Left
    //             IVec2::new(9, 11),
    //             // Top
    //             IVec2::new(10, 11),
    //             // Top Right
    //             IVec2::new(11, 11),
    //             // Right
    //             IVec2::new(11, 10),
    //             // Bottom Right
    //             IVec2::new(11, 9),
    //             // Bottom
    //             IVec2::new(10, 9),
    //             // Bottom Left
    //             IVec2::new(9, 9),
    //         ]
    //     );
    // }

    // #[test]
    // fn correct_coordinates_negative() {
    //     let cell = ComplexCell2d {
    //         coords: IVec2::new(-10, 10),
    //     };
    //     let neighbors = cell.neighbor_coordinates().into_iter().collect::<Vec<_>>();
    //     assert_eq!(
    //         neighbors,
    //         vec![
    //             // Left
    //             IVec2::new(-11, 10),
    //             // Top Left
    //             IVec2::new(-11, 11),
    //             // Top
    //             IVec2::new(-10, 11),
    //             // Top Right
    //             IVec2::new(-9, 11),
    //             // Right
    //             IVec2::new(-9, 10),
    //             // Bottom Right
    //             IVec2::new(-9, 9),
    //             // Bottom
    //             IVec2::new(-10, 9),
    //             // Bottom Left
    //             IVec2::new(-11, 9),
    //         ]
    //     );
    // }

    #[test]
    fn correct_coordinates_origin() {
        let cell = ComplexCell2d {
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

                // Long-range links
                IVec2::new(-20, 0),
                IVec2::new(0, 20),
                IVec2::new(20, 0),
                IVec2::new(0, -20),
            ]
        );
    }
}
