#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s: &Shoe| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes: Vec<Shoe> = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size: Vec<Shoe> = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
// these are test iterators
#[test]
    fn iterator_demonstration() {
        let v1: Vec<i32> = vec![1, 2, 3];

        /*
            If we wanted to create an iterator that takes ownership of
            v1 and returned owned values, we could use the 
            `into_iter` method. e.g. v1.into_iter();

            Alternatively, if we wanted to iterate over mutable
            references, we could use the `iter_mut` method.
            e.g. v1.iter_mut();
         */
        let mut v1_iter: std::slice::Iter<'_, i32> = v1.iter();

        /*
            Methods that call next are called consumer adapters because
            calling them takes up the iterator.
         */
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

#[test]
    fn iterator_sum() {
        let v1: Vec<i32> = vec![1, 2, 3];

        let v1_iter: std::slice::Iter<'_, i32> = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
