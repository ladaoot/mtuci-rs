/*
----> ЗАДАНИЕ 2 "Площадь квадрата"

Создать структуру Rect (квадрат), которая задается координатами левого верхнего угла и длиной стороны.
Добавить для этой структуры методы new(top_left: (f32, f32), width: f32) -> Rect
                                   bottom_right -> (f32, f32), // Выводит координаты правого нижнего угла
                                   area -> f32 // Вычисляет площадь квадрата
                                   perimeter -> f32 // Вычисляет периметр квадрата

 */

fn main() {}

struct Rect {
    x: f32,
    y: f32,
    width: f32,
}

impl Rect {
    fn new(top_left: (f32, f32), width: f32) -> Self {
        let (x, y) = top_left;
        Rect {
            x,
            y,
            width,
        }
    }

    fn bottom_right(&self) -> (f32, f32) {
        let b_r_x = self.x + self.width;
        let b_r_y = self.y - self.width;
        (b_r_x, b_r_y)
    }

    fn area(&self) -> f32 {
        self.width.powi(2)
    }

    fn perimeter(&self) -> f32 {
        self.width * 4.
    }
}


// ----> TESTS
#[cfg(test)]
mod tests {
    use crate::Rect;

    #[test]
    fn bottom_right() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!((6., -3.), rect.bottom_right())
    }

    #[test]
    fn area() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!(25., rect.area())
    }

    #[test]
    fn perimeter() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!(20., rect.perimeter())
    }
}