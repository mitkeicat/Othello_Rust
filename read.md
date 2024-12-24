# オセロのプログラム

## 概要
オセロのプログラムです。Rust による実装になります。ゲームアルゴリズムの研究用を考えています。ボードの表現にビットボードを使用しています。

## 使い方
プログラムを実行すると対戦モードの入力になります。対戦モードは以下の通りです：

1. 黒（先手）
    - 人間は黒、AIが白の対戦です。
2. 白（後手）
    - AIが黒、人間が白の対戦です。
3. PC vs PC
    - 設定したAI同士で対戦します。
4. Human vs Human
    - 黒と白の双方を人間で対戦します。
5. Mult
    - 設定されたAI同士で50回対戦します。アルゴリズムの強さを測定するためのモードです。
6. Test
    - 規定された局面から対戦をスタートするデバッグ用のモードです。

## AIの設定
AIの設定は以下の3つから選択します：

1. alpha_beta
    - アルファベータ法によって着手を決定します。読みの深さは8手先まで読みます。残り手数が14手から完全読みをします。
2. MCTS (Monte Carlo Tree Search)
    - MCTSによって着手を決定します。シミュレーション回数は5000回としています。木を展開するしきい値は一回としています。
3. primitive_montecarlo
    - 原始的モンテカルロ法によって着手を決定します。MCTSのようなツリーサーチを行わず、ロールアウトのみによって探索します。

## Othello Program

### Overview
This is an Othello program. It is implemented in Rust. It is intended for research into game algorithms. Bitboard is used to represent the board.

### How to Use
When you run the program, it will prompt you to select a game mode. The game modes are as follows:

1. Black (first move)
    - The human plays as black and the AI plays as white.
2. White (second move)
    - The AI plays as black and the human plays as white.
3. PC vs PC
    - The AI you set will play against each other.
4. Human vs Human
    - Both black and white will be played by humans.
5. Mult
    - The AI you set will play against each other 50 times. This is a mode for measuring the strength of the algorithm.
6. Test
    - This is a debugging mode that starts the game from a specified position.

### AI Settings
You can choose from the following three options for AI settings:

1. alpha_beta
    - Determines the move using the alpha-beta method. The depth of the read is up to 8 moves ahead. A complete read is performed when there are 14 moves remaining.
2. MCTS (Monte Carlo Tree Search)
    - Determines the move using MCTS. The number of simulations is set to 5000. The threshold for expanding the tree is set to one time.
3. primitive_montecarlo
    - Determines the move using the primitive Monte Carlo method. Does not perform a tree search like MCTS, but searches only by rollout.

