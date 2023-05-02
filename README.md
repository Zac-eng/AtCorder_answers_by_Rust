# AtCorder_answers_by_Rust
I push some answers of AtCorder Contest Problems written　by Rust here. I hope this repository will help someone trying to learn Rust. I am learning Rust too ! I am waiting for some advice or better practice !

Cargo.toml I choose libraries from is:
```

# AtCoder 2019年言語アップデート以降に使用できるクレート

# 各クレートの説明
# https://github.com/rust-lang-ja/atcoder-rust-resources/wiki/2020-Update

# 数値型の抽象化、多倍長整数、複素数、分数、及び整数型の拡張
num = "=0.2.1"
num-bigint = "=0.2.6"
num-complex = "=0.2.4"
num-integer = "=0.1.42"
num-iter = "=0.1.40"
num-rational = "=0.2.4"
num-traits = "=0.2.11"

# `num-traits`の自動実装
num-derive = "=0.3.0"

# NumPyの`ndarray`のような多次元配列
ndarray = "=0.13.0"

# 線形代数
nalgebra = "=0.20.0"

# (線形)代数の抽象化
alga = "=0.9.3"

# libmのRust実装
libm = "=0.2.1"

# 乱数
rand = { version = "=0.7.3", features = ["small_rng"] }
getrandom = "=0.1.14"
rand_chacha = "=0.2.2"
rand_core = "=0.5.1"
rand_hc = "=0.2.0"
rand_pcg = "=0.2.1"

# 乱数の分布の追加
rand_distr = "=0.2.2"

# グラフ
petgraph = "=0.5.0"

# 挿入順を保持するhash table
indexmap = "=1.3.2"

# 正規表現
regex = "=1.3.6"

# staticアイテムの遅延初期化
lazy_static = "=1.4.0"

# `NotNan<f64>`, `OrderedFloat<f64>`
ordered-float = "=1.0.2"

# ASCII文字列
ascii = "=1.0.0"

# permutation
permutohedron = "=0.2.4"

# スライスの拡張
superslice = "=1.0.0"

# イテレータの拡張
itertools = "=0.9.0"

# イテレータの拡張（一次元累積和と浮動小数点数の等差数列）
itertools-num = "=0.1.3"

# `BTreeMap`, `BTreeSet`, `HashMap`, `HashSet`のリテラル用マクロ
maplit = "=1.0.2"

# 即席enum `Either<L, R>`
either = "=1.5.3"

# `BTreeMap`, `BTreeSet`, `HashMap`, `HashSet`, `Vec`の永続データ構造版
im-rc = "=14.3.0"

# 可変長bit set
fixedbitset = "=0.2.0"

# 可変長bit set
bitset-fixed = "=0.1.0"

# 競技プログラミングの入出力サポートその1
proconio = { version = "=0.3.6", features = ["derive"] }

# 競技プログラミングの入出力サポートその2
text_io = "=0.1.8"

# 競技プログラミングの入出力サポートその3
whiteread = "=0.5.0"

# 高速なハッシュ関数
rustc-hash = "=1.1.0"

# ある長さまでは要素を「直に」持つ可変長配列
smallvec = "=1.2.0"
```

refarance:
https://github.com/rust-lang-ja/atcoder-rust-resources/wiki/2020-Update