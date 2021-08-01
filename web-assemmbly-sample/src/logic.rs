/**
 * マンデルブロ集合を計算するためのプログラムを実装したファイル
 */

/**
 * セルの発散判定を行い、判定までのイテレーション回数を返す関数
 */
fn get_n_diverged(x0: f64, y0: f64, max_iter: usize) -> u8 {
    // 複素数z_nの実部をxn
    let mut xn = 0.0;
    // 複素数z_nの虚部をyn
    let mut yn = 0.0;

    // イテレーション回数を計算
    for i in 1..max_iter {
        let x_next = xn * xn - yn * yn + x0;
        let y_next = 2.0 * xn * yn + y0;
        // 代入する。
        xn = x_next;
        yn = y_next;
        // 実部の2乗と虚部の2乗の合計が4.0を超えた場合
        if yn * yn + xn * xn > 4.0 {
            // 戻り値
            return 1 as u8;
        }
    }
    max_iter as u8;
}