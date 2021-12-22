use std::io::Write;
use std::path::PathBuf;
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod repostatus;
mod repo;

const REPO_NAME_WIDTH: usize = 20;
const REPO_STATUS_WIDTH: usize = 9;
const BARNCH_NAME_WIDTH: usize = 12;

struct Tui {
    column: u16,
    column_id: u16,
    current_column_id: u16,
    row_column_counts: Vec<u16>,
    row: u16,
    current_row: u16,
    row_count: usize,
}

impl Tui {
    fn new() -> Self {
        Self {
            column: 0,
            column_id: 0,
            current_column_id: 0,
            row_column_counts: Vec::new(),
            row: 0,
            current_row: 0,
            row_count: 0,
        }
    }

    fn reset(&mut self) {
        self.row = 0;
        self.column = 0;
    }

    fn row(&self) -> u16 {
        // Plus 1 to skip the header line.
        self.row + 1
    }

    fn finished_row(&mut self) {
        self.column = 0;
        self.column_id = 0;
        self.row += 1;
    }

    fn go_up(&mut self) {
        if self.current_row > 0 {
            self.current_row -= 1;
        }
        self.validate_current_column()
    }

    fn go_down(&mut self) {
        if self.current_row < self.row_count as u16 - 1 {
            self.current_row += 1;
        }
        self.validate_current_column()
    }

    fn go_right(&mut self) {
        self.current_column_id += 1;
        self.validate_current_column()
    }

    fn go_left(&mut self) {
        if self.current_column_id > 0 {
            self.current_column_id -= 1;
        }
    }

    fn validate_current_column(&mut self) {
        if self.current_column_id > self.row_column_counts[self.current_row as usize] - 1 {
            self.current_column_id = self.row_column_counts[self.current_row as usize] - 1
        }
    }

    fn column(&mut self) -> u16 {
        match self.column_id {
            0 => {}
            1 => self.column += REPO_NAME_WIDTH as u16 + 1,
            _ => self.column += REPO_STATUS_WIDTH as u16 + 1,
        };
        self.column_id += 1;
        self.column
    }

    fn adjust_column_width(&mut self, width: u16) {
        self.column -= 10;
        self.column += width + 1;
    }

    fn is_current_cell(&self) -> bool {
        self.column_id == self.current_column_id + 1 && self.row == self.current_row
    }
}

/// Zero based termion goto.
fn goto(x: u16, y: u16) -> termion::cursor::Goto {
    termion::cursor::Goto(x + 1, y + 1)
}

fn main() {
    let dev_dir = get_dev_dir();
    let repo_paths = find_repo_dirs(dev_dir);
    let repos: Vec<repo::Repo> = repo_paths
        .iter()
        .map(|path| repo::Repo::new(path.to_path_buf()))
        .collect();
    tui(repos);
}

fn get_dev_dir() -> PathBuf {
    match std::env::var("DEVDIR") {
        Ok(path) => PathBuf::from(path),
        Err(_) => std::env::current_dir().unwrap(),
    }
}

fn find_repo_dirs(root: PathBuf) -> Vec<PathBuf> {
    let mut repos: Vec<PathBuf> = Vec::new();

    if let Ok(read_dir) = root.read_dir() {
        for dir in read_dir {
            if dir.as_ref().expect("msg").path().join(".git").is_dir() {
                repos.push(dir.unwrap().path().to_path_buf())
            }
        }
    }

    repos.sort_by_key(|x| x.to_str().unwrap().to_lowercase());
    repos
}

fn tui(mut repos: Vec<repo::Repo>) {
    let bg_current_cell = color::Bg(color::Rgb(75, 30, 15));
    let bg_reset = color::Bg(color::Reset);

    let fg_master_ok = color::Fg(color::Rgb(0, 175, 0));
    let fg_master_not_ok = color::Fg(color::Rgb(255, 180, 0));
    let fg_not_master_ok = color::Fg(color::Rgb(0, 200, 255));
    let fg_not_master_not_ok = color::Fg(color::Rgb(225, 0, 0));

    let fg_active_branch = color::Fg(color::Rgb(35, 200, 35));
    let fg_inactive_branch = color::Fg(color::Rgb(90, 90, 90));

    let fg_info = color::Fg(color::Rgb(75, 75, 75));

    let fg_reset = color::Fg(color::Reset);

    let stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut stdout = termion::screen::AlternateScreen::from(stdout);
    let mut keep_running = true;
    let mut tui = Tui::new();
    let repo_count = repos.len();

    let header = format!(
        "{}{}{:>re$} |{:^st$}| Branches ------->",
        goto(0, 0),
        fg_info,
        "<------- Repo",
        "stat",
        re = REPO_NAME_WIDTH,
        st = REPO_STATUS_WIDTH - 2,
    );
    let footer = format!(
        "{}U: untracked, D: deleted, d: deleted staged, S: staged{}M: modified, N: new file, n: new file 2",
        goto(1, repos.len() as u16+1),
        goto(1, repos.len() as u16+2),
    );

    while keep_running {
        write!(stdout, "{}", termion::clear::All).unwrap();
        write!(stdout, "{}", header).unwrap();
        write!(stdout, "{}", footer).unwrap();
        tui.reset();
        tui.row_count = repo_count;

        for repo in &repos {
            tui.row_column_counts.push(repo.branches.len() as u16 + 2);

            write!(stdout, "{}", goto(tui.column(), tui.row())).unwrap();
            match repo.get_repo_state() {
                repo::RepoState::MasterOk => write!(stdout, "{}", fg_master_ok).unwrap(),
                repo::RepoState::MasterNotOk => write!(stdout, "{}", fg_master_not_ok).unwrap(),
                repo::RepoState::NotMasterOK => write!(stdout, "{}", fg_not_master_ok).unwrap(),
                repo::RepoState::NotMasterNotOK => write!(stdout, "{}", fg_not_master_not_ok).unwrap(),
            }
            {
                if tui.is_current_cell() {
                    write!(stdout, "{}", bg_current_cell).unwrap();
                }
                write!(stdout, "{}", repo.name).unwrap();
            }

            write!(stdout, "{}", bg_reset).unwrap();
            write!(stdout, "{}", goto(tui.column(), tui.row())).unwrap();

            {
                if tui.is_current_cell() {
                    write!(stdout, "{}", bg_current_cell).unwrap();
                }
                write!(stdout, "[{}]", repo.status.to_string()).unwrap();
            }

            write!(stdout, "{}", fg_reset).unwrap();
            write!(stdout, "{}", bg_reset).unwrap();

            for branch in &repo.branches {
                write!(stdout, "{}", goto(tui.column(), tui.row())).unwrap();

                {
                    if tui.is_current_cell() {
                        write!(stdout, "{}", bg_current_cell).unwrap();
                    }
                    if branch == repo.current_branch.as_str() {
                        write!(stdout, "{}", fg_active_branch).unwrap();
                    } else {
                        write!(stdout, "{}", fg_inactive_branch).unwrap();
                    }
                    if tui.column > 100 {
                        write!(stdout, "...").unwrap();
                        write!(stdout, "{}{}", bg_reset, fg_reset).unwrap();
                        break;
                    } else {
                        write!(stdout, "{}", branch).unwrap();
                    }
                    write!(stdout, "{}{}", bg_reset, fg_reset).unwrap();
                }

                tui.adjust_column_width(branch.len() as u16);
            }

            tui.finished_row();
        }

        let branch_index = match tui.current_column_id {
            0 | 1 | 2 => 0_usize,
            _ => tui.current_column_id as usize - 2,
        };
        write!(
            stdout,
            "{}{} [{:<w$}] < {}",
            goto(0, repos.len() as u16 + 3),
            repos[tui.current_row as usize].name,
            repos[tui.current_row as usize].current_branch,
            repos[tui.current_row as usize].branches[branch_index],
            w = BARNCH_NAME_WIDTH,
        )
        .unwrap();

        stdout.flush().unwrap();

        for c in std::io::stdin().keys() {
            match c.unwrap() {
                Key::Char('q') => {
                    keep_running = false;
                    break;
                }
                Key::Right | Key::Char('l') => {
                    tui.go_right();
                    break;
                }
                Key::Left | Key::Char('h') => {
                    tui.go_left();
                    break;
                }
                Key::Up | Key::Char('k') => {
                    tui.go_up();
                    break;
                }
                Key::Down | Key::Char('j') => {
                    tui.go_down();
                    break;
                }
                Key::Char('\n') => {
                    match tui.current_column_id {
                        0 => {}
                        1 => {
                            repos[tui.current_row as usize].clear_stat();
                            break;
                        }
                        _ => {
                            let branch = repos[tui.current_row as usize].branches
                                [tui.current_column_id as usize - 2]
                                .to_owned();
                            repos[tui.current_row as usize].checkout_branch(branch);
                        }
                    }
                    break;
                }
                _ => {}
            }
        }
    }
    writeln!(stdout, "{}", goto(0, repos.len() as u16 + 3)).unwrap();
}
