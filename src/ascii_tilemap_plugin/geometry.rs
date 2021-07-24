use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    x_left: u32,
    y_bottom: u32,
    x_right: u32,
    y_top: u32,
}

impl Rect {
    /// Creates a Rect with the x,y at the bottom left and the given dimensions
    pub fn with_dimension(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x_left: x,
            y_bottom: y,
            x_right: x + width,
            y_top: y + height,
        }
    }

    /// Returns true if this intersects with another Rect
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x_left <= other.x_right
            && self.x_right >= other.x_left
            && self.y_bottom <= other.y_top
            && self.y_top >= other.y_bottom
    }

    /// Returns the center of the Rect
    pub fn center(&self) -> UVec2 {
        UVec2::new(
            (self.x_left + self.x_right) / 2,
            (self.y_bottom + self.y_top) / 2,
        )
    }

    /// Returns true if a point is inside the Rect
    pub fn point_in_rect(&self, point: UVec2) -> bool {
        point.x >= self.x_left
            && point.x < self.x_right
            && point.y >= self.y_bottom
            && point.y < self.y_top
    }

    /// Returns an iterator that iterates over each points inside the Rect
    pub fn points(&self) -> Points {
        Points {
            curr_index: 0,
            width: self.width(),
            len: self.width() * self.height(),
            x: self.x_left,
            y: self.y_bottom,
        }
    }

    /// Returns the Rect width
    pub fn width(&self) -> u32 {
        i64::abs(i64::from(self.x_right) - i64::from(self.x_left)) as u32
    }

    /// Returns the Rect height
    pub fn height(&self) -> u32 {
        i64::abs(i64::from(self.y_top) - i64::from(self.y_bottom)) as u32
    }
}

pub struct Points {
    curr_index: u32,
    width: u32,
    len: u32,
    x: u32,
    y: u32,
}

impl Iterator for Points {
    type Item = UVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_index >= self.len {
            None
        } else {
            let x = self.curr_index % self.width;
            let y = self.curr_index / self.width;
            self.curr_index += 1;

            Some(UVec2::new((self.x + x) as u32, (self.y + y) as u32))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn iter() {
        let rect = super::Rect::with_dimension(0, 0, 2, 2);
        assert_eq!(
            rect.points().collect::<Vec<UVec2>>(),
            vec![
                UVec2::new(0, 0),
                UVec2::new(1, 0),
                UVec2::new(0, 1),
                UVec2::new(1, 1),
            ]
        );
    }

    #[test]
    fn iter_offset() {
        let rect = super::Rect::with_dimension(1, 1, 2, 2);
        assert_eq!(
            rect.points().collect::<Vec<UVec2>>(),
            vec![
                UVec2::new(1, 1),
                UVec2::new(2, 1),
                UVec2::new(1, 2),
                UVec2::new(2, 2),
            ]
        );
    }
}
