fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let p1 = Point { x: 1, y: 2 };
    // let p2 = Point { x: 32.6, y: 69.420 };

    // let p1 = Point { x: 23, y: 43 };
    // let p2 = Point { x: 44.22, y: 67.69 };

    let p1 = Point { x: 32, y: 54 };
    let point_with_label = p1.label("Coordinates");
    // let point_with_label = Point::label(&p1, "Coordinates");
    dbg!(point_with_label);
}

// fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
//     if list.is_empty() {
//         return None;
//     }
//
//     let mut max = &list[0];
//
//     for i in list.iter() {
//         if i > max {
//             max = i;
//         }
//     }
//
//     Some(max)
// }

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// impl<T> Point<T> {
//     fn coordinates(&self) -> (&T, &T) {
//         (&self.x, &self.y)
//     }
// }

impl<T> Point<T> {
    fn label<U>(&self, label: U) -> (U, &T, &T) {
        (label, &self.x, &self.y)
    }
}
