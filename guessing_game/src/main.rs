use std::io;
use std::cmp::Ordering;
// Rngトレイトは乱数生成器が実装すべきメソッドを定義
use rand::Rng;

fn main() {

    // println! は画面に文字列を表示するマクロ
    println!("Guess the number!"); // 数を当ててごらん
    // 乱数生成して 開始〜終了
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);    //秘密の数字は次の通り: {}



    // match式は複数のアーム（腕）で構成されます。
    // 各アームはマッチさせるパターン
    // matchに与えられた値がそのアームのパターンにマッチしたときに実行されるコードで構成。
    loop {
        println!("Please input your guess."); // ほら、予想を入力してね

        // let を使用して変数を宣言
        // Rust は変数を可変（mutable）にするには、`mut`をつける
        // let apples = 5;      // immutable
        // let mut bananas = 5; // mutable
        // 可変変数を作成し、その変数は現時点では新しい空のStringのインスタンスに束縛されている
        let mut guess = String::new();

        // `stdin関数`はターミナルの標準入力へのハンドルを表す型である std::io::Stdin のインスタンスを返す
        // read_lineメソッドは、ユーザが標準入力に入力したものを文字列に追加すること
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");     // 行の読み込みに失敗しました

        // もしparseメソッドが文字列から数値への変換に成功したなら、
        // 結果の数値を保持するOk値を返す。 このOk値は最初のアームのパターンにマッチ。
        // match式はparseメソッドが生成してOk値に格納したnumの値を返します。
        // その数値は私たちが望んだように、これから作成する新しいguess変数に収まります。

        // もしparseメソッドが文字列から数値への変換に失敗したなら、エラーに関する詳細な情報を含むErr値を返します。
        // このErr値は最初のmatchアームのOk(num)パターンにはマッチしませんが、2番目のアームのErr(_)パターンにはマッチします。
        // アンダースコアの_はすべての値を受け付けます。 この例ではすべてのErr値に対して、その中にどんな情報があってもマッチさせたいと言っているのです。
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);     // 次のように予想しました: {}
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            // ループを抜けることはプログラミングを抜けるということ
            Ordering::Equal => {
                println!("You win!");
                break
            }
        }
    }
}
