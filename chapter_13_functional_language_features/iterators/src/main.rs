fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // Under the hood this automatically creates and consumes an iterator
    for &item in &v1 {
        println!("for item {item} in v1");
    }

    for &item in v1_iter {
        println!("for item {item} in v1");
    }

    let v2: Vec<i32> = vec![1, 2, 3];
    // Map returns another iterator from the iter() "main iterator", to transform it into the
    // collection you should use collect()
    // But because the all iterators are lazy, you have to call one of the consuming adapter
    // methods to get results from calls to iterator adapters
    // You can also chain iterators
    // This two are valid
    let iter: Vec<i32> = v2.iter().map(|x| x + 1).map(|x1| x1 + 1).collect();
    let iter: Vec<i32> = v2
        .iter()
        .map(|x| x + 1)
        .collect::<Vec<i32>>()
        .iter()
        .map(|x1| x1 + 1)
        .collect();

    println!("{iter:?}");

    // Many iterator adapters take closures as arguments, and commonly the closures we'll specify
    // as arguments to iterator adapters will be closures that capture their environment.
    //
    // For examples, we'll use the filter() method that takes a closure. The closure gets an item
    // from the iterator and returns a bool. If the closure returns true, the value will be
    // included in the iteration produced by filter. If the closure returns false, the value won't
    // be included.
    //
    //
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
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
        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                }
            ]
        )
    }
}
