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
    max_iter as u8
}

/**
 * 各セルについての色情報を格納する関数
 */
pub fn generate_mandelbrot_set(
    canvas_w: usize, 
    canvas_h: usize, 
    x_min: f64, 
    x_max: f64, 
    y_min: f64, 
    y_max: f64,
    max_iter: usize) -> Vec<u8> {
        let canvas_w_f64 = canvas_w as f64;
        let canvas_h_f64 = canvas_h as f64;
        // 色情報を格納する配列を用意する。
        let mut data = vec![];
        // 色情報を配列に格納する。
        for i in 0..canvas_h {
            // iを宣言する。
            let i_f64 = i as f64;
            let y = y_min + (y_max - y_min) * i_f64 / canvas_h_f64;
            // 横情報毎に格納する。
            for j in 0..canvas_w {
                let x = x_min + (x_max - x_min) * (j as f64) / canvas_w_f64;
                // イテレーション回数を取得する。
                let iter_index = get_n_diverged(x, y, max_iter);
                // 8色に塗り分けます
                let v = iter_index % 8 * 32;
                // R
                data.push(v);
                // G
                data.push(v);
                // B
                data.push(v);
                // A
                data.push(255);
            }
        }
        data
}

/**
 * 各種テストを追記する。
 */
#[cfg(test)]
mod tests {
    // 必要なモジュールを読み込む
    use super::*;

    /**
     * get_n_diverged()関数用のテスト
     */
    #[test]
    fn test_get_n_diverged() {
        // イテレーション関数
        let max_iter = 10;
        // チェック
        assert_eq!(get_n_diverged(1.0, 0.0, max_iter), 3);
        assert_eq!(get_n_diverged(0.0, 0.0, max_iter), max_iter as u8);
        assert_eq!(get_n_diverged(0.0, 1.0, max_iter), max_iter as u8);
    }

    /**
     * generate_mandelbrot_set関数用のテスト
     */
    #[test]
    fn test_generate_mandelbrot_set() {
        // 各種変数を用意する。
        let canvas_w = 2;
        let canvas_h = 2;
        let x_min = -1.0;
        let x_max = 1.0;
        let y_min = -1.0;
        let y_max = 1.0;
        let max_iter = 8;
        // [(-1, -1), (-1, 0), (0, -1), (0, 0)]について値を確認する。
        assert_eq!(
            generate_mandelbrot_set(canvas_w, canvas_h, x_min, x_max, y_min, y_max, max_iter),
            vec![96, 96, 96, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255]
        );
    }
}