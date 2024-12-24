# �I�Z���̃v���O����

## �T�v
�I�Z���̃v���O�����ł��BRust �ɂ������ɂȂ�܂��B�Q�[���A���S���Y���̌����p���l���Ă��܂��B�{�[�h�̕\���Ƀr�b�g�{�[�h���g�p���Ă��܂��B

## �g����
�v���O���������s����Ƒΐ탂�[�h�̓��͂ɂȂ�܂��B�ΐ탂�[�h�͈ȉ��̒ʂ�ł��F

1. ���i���j
    - �l�Ԃ͍��AAI�����̑ΐ�ł��B
2. ���i���j
    - AI�����A�l�Ԃ����̑ΐ�ł��B
3. PC vs PC
    - �ݒ肵��AI���m�őΐ킵�܂��B
4. Human vs Human
    - ���Ɣ��̑o����l�Ԃőΐ킵�܂��B
5. Mult
    - �ݒ肳�ꂽAI���m��50��ΐ킵�܂��B�A���S���Y���̋����𑪒肷�邽�߂̃��[�h�ł��B
6. Test
    - �K�肳�ꂽ�ǖʂ���ΐ���X�^�[�g����f�o�b�O�p�̃��[�h�ł��B

## AI�̐ݒ�
AI�̐ݒ�͈ȉ���3����I�����܂��F

1. alpha_beta
    - �A���t�@�x�[�^�@�ɂ���Ē�������肵�܂��B�ǂ݂̐[����8���܂œǂ݂܂��B�c��萔��14�肩�犮�S�ǂ݂����܂��B
2. MCTS (Monte Carlo Tree Search)
    - MCTS�ɂ���Ē�������肵�܂��B�V�~�����[�V�����񐔂�5000��Ƃ��Ă��܂��B�؂�W�J���邵�����l�͈��Ƃ��Ă��܂��B
3. primitive_montecarlo
    - ���n�I�����e�J�����@�ɂ���Ē�������肵�܂��BMCTS�̂悤�ȃc���[�T�[�`���s�킸�A���[���A�E�g�݂̂ɂ���ĒT�����܂��B

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

