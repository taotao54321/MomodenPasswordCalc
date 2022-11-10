use momoden_password::{Password, PasswordChar};

use crate::query::{Pattern, PatternChar};
use crate::util::VecExt as _;

pub(crate) fn generate_passwords(pattern: &Pattern, count_max: usize) -> Vec<Password> {
    let mut solver = Solver::new(pattern, count_max);
    solver.dfs();

    solver.passwords
}

#[derive(Debug)]
struct Solver<'a> {
    pattern: &'a Pattern,
    count_max: usize,
    passwords: Vec<Password>,
    chars: Vec<PasswordChar>,
}

impl<'a> Solver<'a> {
    fn new(pattern: &'a Pattern, count_max: usize) -> Self {
        Self {
            pattern,
            count_max,
            passwords: Vec::with_capacity(count_max),
            chars: Vec::with_capacity(Password::MAX_LEN),
        }
    }

    fn dfs(&mut self) {
        // 規定数のパスワードが生成されたら打ち切る。
        if self.passwords.len() >= self.count_max {
            return;
        }

        let pos = self.chars.len();

        // 全ての文字が決まったら有効かどうかチェックして戻る。
        if pos == self.pattern.len() {
            let password = unsafe { Password::new_unchecked(&self.chars) };
            if password.is_valid() {
                self.passwords.push(password);
            }
            return;
        }

        // 枝刈り: 2 文字目が無効なら直ちに却下。
        if pos == 2 && Password::is_invalid_second_char(unsafe { *self.chars.get_unchecked(1) }) {
            return;
        }

        match self.pattern[pos] {
            PatternChar::Password(pc) => {
                self.chars.push(pc);
                self.dfs();
                unsafe { self.chars.pop_unchecked() }
            }
            PatternChar::Any => {
                for pc in PasswordChar::all() {
                    self.chars.push(pc);
                    self.dfs();
                    unsafe { self.chars.pop_unchecked() }
                }
            }
        }
    }
}
