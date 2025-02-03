use std::collections::HashMap;
use std::sync::LazyLock;

static COINS: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    let map: HashMap<&str, &str> = HashMap::from([
        ("USDC", "3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh"),
        ("SOL", "So11111111111111111111111111111111111111111"),
        ("BTC", "3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh"),
        ("WBTC", "3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh")
    ]);
    map
});
