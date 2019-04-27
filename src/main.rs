fn main() {
    println!("
    半径 {:.1}、 円周率 {:.3}、 面積 {:.3}",
    // 半径
    3.2,
    // 円周率
    std::f64::consts::PI,
    // 半径 * 円周率
    3.2f64.powi(2) * std::f64::consts::PI,
    );
}
