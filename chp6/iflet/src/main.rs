struct Point(i32);
struct Range(i32, i32);

fn main() {
    let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("Max: {}", max),
    //     _ => (),
    // }

    //more concise:
    if let Some(max) = config_max {
        println!("Max: {}", max);
    }

    enum Location {
        Point(i32),
        Range(i32, i32),
    }

    fn print_range_max(loc: &Location) {
        if let Location::Range(_, second) = loc {
            println!("{second}");
        }
    }

    let my_loc = Location::Range(0, 10);
    let my_loc2 = Location::Point(11);

    print_range_max(&my_loc);
    print_range_max(&my_loc2);
}
