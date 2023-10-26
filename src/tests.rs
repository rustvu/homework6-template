//////////////////////////////////////////////////////////////////////////////
// DO NOT EDIT THIS FILE
use super::*;
use paste::paste;

macro_rules! test_resource_with_capacity {
    ($factory:tt, $resource:tt) => {
        paste! {
            #[test]
            fn [< $factory:lower >]() {
                let mut source = $factory::new(1);

                // The factory should produce the proper resource type
                let _item: $resource = source.next().unwrap();

                let source = <$factory>::new(42);
                assert_eq!(
                    source.take(100).count(),
                    42,
                    "The {} should produce {} up to its capacity",
                    stringify!($factory), stringify!($resource)
                );
            }
        }
    };
}

#[test]
fn faucet() {
    let mut faucet = Faucet::new();

    // The factory should produce the proper resource type
    let _water: Water = faucet.next().unwrap();

    assert_eq!(
        faucet.take(100).count(),
        100,
        "The faucet should produce water indefinitely"
    );
}

test_resource_with_capacity!(Sack, Flour);
test_resource_with_capacity!(Jar, Yeast);
test_resource_with_capacity!(Shaker, Salt);

#[test]
fn breadboard() {
    let faucet = Faucet::new();
    let sack = Sack::new(1);
    let jar = Jar::new(2);
    let shaker = Shaker::new(3);

    let mut board = BreadBoard::new(faucet, sack, shaker, jar);

    // The factory should produce the proper resource type
    let _dough: Dough = board.next().unwrap();

    assert!(
        board.next().is_none(),
        "The breadboard should produce dough only once (smallest resource capacity)"
    );
}

test_resource_with_capacity!(Can, Sauce);

#[test]
fn cheese() {
    let mut grater = Grater::new(1);

    // The factory should produce the proper resource type
    let _cheese: Cheese = grater.next().unwrap();

    const TEST_THROUGHPUT: usize = 3;

    // Test if the grater produces cheese at the proper throughput

    // Case 1: asking only for throughput number of cheese
    let grater = Grater::new(TEST_THROUGHPUT);
    let n = TEST_THROUGHPUT;
    let start = Instant::now();
    let count = grater.take(n).count();
    let elapsed = start.elapsed();
    assert_eq!(count, n, "The grater should produce cheese indefinitely");
    assert!(
        elapsed < Duration::from_millis(10),
        "The grater should produce throughput number of cheese immediately"
    );

    // Case 2: asking for more than throughput number of cheese (as fast as possible)
    let grater = Grater::new(TEST_THROUGHPUT);
    let n = TEST_THROUGHPUT + 1; // more than throughput
    let start = Instant::now();
    let count = grater.take(n).count();
    let elapsed = start.elapsed();
    assert_eq!(count, n, "The grater should produce cheese indefinitely");
    assert!(
        elapsed > Duration::from_millis(1000),
        "The grater should throttle cheese production if being asked too fast"
    );

    // Case 3: asking for more than throughput number of cheese (slowly)
    let mut grater = Grater::new(TEST_THROUGHPUT);
    let n = TEST_THROUGHPUT + 1; // more than throughput
    for _ in 0..n {
        let start = Instant::now();
        if grater.next().is_none() {
            panic!("The grater should produce cheese indefinitely")
        }
        assert!(
            start.elapsed() < Duration::from_millis(10),
            "The grater should not throttle if being asked slowly"
        );
        sleep(Duration::from_millis(1000 / TEST_THROUGHPUT as u64 + 10));
    }
}

#[test]
fn prep_table() {
    // dummy resources
    let breadboard = std::iter::from_fn(|| {
        Some(Dough {
            water: Water,
            flour: Flour,
            salt: Salt,
            yeast: Yeast,
        })
    });

    let can = std::iter::from_fn(|| Some(Sauce));

    let mut cheese_cnt = 0;
    let grater = std::iter::from_fn(|| {
        cheese_cnt += 1;
        Some(Cheese)
    });

    let n_cheese = 3;
    let mut prep_table = PrepTable::new(breadboard, can, grater, n_cheese);

    // The factory should produce the proper resource type
    let _raw_pizza: RawPizza = prep_table.next().unwrap();

    assert_eq!(
        cheese_cnt,
        n_cheese,
        "The prep table should produce raw pizza with the proper amount of cheese"
    );
}

#[test]
fn oven() {
    // dummy resource
    let prep_table = std::iter::from_fn(|| {
        Some(RawPizza{
            dough: Dough {
                water: Water,
                flour: Flour,
                salt: Salt,
                yeast: Yeast,
            },
            cheese: vec![Cheese],
            sauce: Sauce,
        })
    });

    let baking_time = Duration::from_millis(100);
    let mut oven = Oven::new(prep_table, baking_time);

    let start = Instant::now();
    let _pizza = oven.next().unwrap();
    let elapsed = start.elapsed();

    assert_eq!(
        elapsed >= baking_time,
        true,
        "The oven should bake pizza for the proper amount of time"
    );


}

