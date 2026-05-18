use macroquad::prelude::*;
use macroquad::ui::root_ui;

enum GameState {
    Betting,
    Rolling {
        player_choice: &'static str,
        roll_timer: f32,
        die1: i32,
        die2: i32,
    },
    Result {
        player_choice: &'static str,
        die1: i32,
        die2: i32,
    },
}

#[macroquad::main("Rust版 半丁ゲーム")]
async fn main() {
    // --- 日本語フォントの読み込み ---
    // Windowsのシステムフォント（メイリオ）を読み込みます
    // ※ Macの場合は "C:\\Windows\\Fonts\\msyh.ttc" を "/System/Library/Fonts/ヒラギノ角ゴ ProN W3.otf" などに変更してください
    let font_path = "C:\\Windows\\Fonts\\msyh.ttc";
    
    let font = load_ttf_font(font_path)
        .await
        .unwrap_or_else(|_| {
            panic!("フォントファイルの読み込みに失敗しました。パスを確認してください: {}", font_path)
        });

    let mut money = 1000;
    let bet_amount = 100;
    let mut state = GameState::Betting;

    loop {
        clear_background(DARKGREEN);

        // テキスト描画パラメータの設定（日本語フォントを指定）
        let draw_text_jp = |text: &str, x: f32, y: f32, size: f32, color: Color| {
            draw_text_ex(
                text,
                x,
                y,
                TextParams {
                    font: Some(&font),
                    font_size: size as u16,
                    color,
                    ..Default::default()
                },
            );
        };

        // --- 共通情報の描画 ---
        draw_text_jp(&format!("所持金: {} 両", money), 20.0, 50.0, 30.0, WHITE);
        draw_text_jp(&format!("賭け金: {} 両", bet_amount), 20.0, 90.0, 20.0, LIGHTGRAY);

        // --- 状態ごとの処理 ---
        match state {
            GameState::Betting => {
                draw_text_jp("丁（偶数:Even）か 半（奇数:Odd）か選べ", 200.0, 200.0, 35.0, WHITE);

                if root_ui().button(Vec2::new(250.0, 300.0), "Even") {
                    state = GameState::Rolling {
                        player_choice: "丁",
                        roll_timer: 0.0,
                        die1: 1,
                        die2: 1,
                    };
                }

                if root_ui().button(Vec2::new(410.0, 300.0), "Odd") {
                    state = GameState::Rolling {
                        player_choice: "半",
                        roll_timer: 0.0,
                        die1: 1,
                        die2: 1,
                    };
                }
            }

            GameState::Rolling { ref mut player_choice, ref mut roll_timer, ref mut die1, ref mut die2 } => {
                *die1 = rand::gen_range(1, 7);
                *die2 = rand::gen_range(1, 7);

                // ※ 🎲 マークが化ける場合は、単純に "サイコロ: 3 と 5" のようなテキストにすると安全です
                draw_text_jp(&format!(" dice1: {}   dice2: {}", die1, die2), 260.0, 250.0, 40.0, WHITE);

                *roll_timer += get_frame_time();

                if *roll_timer > 1.5 {
                    state = GameState::Result {
                        player_choice: *player_choice,
                        die1: *die1,
                        die2: *die2,
                    };
                }
            }

            GameState::Result { player_choice, die1, die2 } => {
                let sum = die1 + die2;
                let result = if sum % 2 == 0 { "丁" } else { "半" };
                let is_win = player_choice == result;

                draw_text_jp(&format!("出目: {} + {} = {} ({})", die1, die2, sum, result), 220.0, 200.0, 30.0, WHITE);

                if is_win {
                    draw_text_jp("的中！", 350.0, 280.0, 45.0, YELLOW);
                } else {
                    draw_text_jp("残念...", 350.0, 280.0, 45.0, ORANGE);
                }

                if root_ui().button(Vec2::new(330.0, 400.0), "next") {
                    if is_win {
                        money += bet_amount;
                    } else {
                        money -= bet_amount;
                    }
                    state = GameState::Betting;
                }
            }
        }

        next_frame().await
    }
}