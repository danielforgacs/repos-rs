use crate::prelude::*;

const NO_BRANCH: &str = "(no branch)";

pub struct Repo {
    pub git_repo: Repository,
    name: String,
    current_branch: String,
    branches: Vec<String>,
    status: Status,
}

impl Repo {
    pub fn new(path: &PathBuf) -> ReposResult<Self> {
        let repo = Repository::open(path)?;
        let status = read_status(&repo);
        let current_branch = read_current_branch(&repo);
        let mut branches = read_branches(&repo);
        if branches.is_empty() {
            branches = vec![current_branch.clone()];
        }
        let name = repo
            .path()
            .components()
            .nth_back(1)
            .map(|f| f.as_os_str())
            .unwrap()
            .to_owned()
            .into_string()
            .unwrap();

        Ok(Self {
            git_repo: repo,
            name,
            current_branch,
            branches,
            status,
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn current_branch(&self) -> &str {
        self.current_branch.as_str()
    }

    pub fn sort_branches(&mut self) {
        self.branches.sort();
    }

    pub fn set_current_branch_as_first(&mut self) {
        let mut branches = vec![self.current_branch.to_string()];
        if !self.branches().is_empty() {
            branches.extend(self.branches.iter().cloned().filter(|b| b != &self.current_branch))
        }
        self.branches = branches;
    }

    pub fn branches(&self) -> &Vec<String> {
        &self.branches
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn is_on_master(&self) -> bool {
        self.current_branch == "master"
    }

    pub fn checkout_branch(&self, branch: String) -> ReposResult<()> {
        if branch != NO_BRANCH {
            std::process::Command::new("git")
                .arg("checkout")
                .arg(branch)
                .current_dir(&self.git_repo.path().parent().unwrap())
                .output()
                .expect("Could not checkout repos.");
        };
        Ok(())
    }
}

fn read_current_branch(repo: &Repository) -> String {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
            None
        }
        Err(_error) => return String::from("n/a"),
    };
    let head = head.as_ref().and_then(|h| h.shorthand());
    head.unwrap_or("(no branch)").to_string()
}

fn read_branches(repo: &Repository) -> Vec<String> {
    repo
        .branches(None)
        .unwrap()
        .map(|f| f.unwrap())
        .filter(|f| f.1 == BranchType::Local)
        .map(|f| f.0)
        .map(|f| f.name().unwrap().unwrap().to_string())
        .collect::<Vec<String>>()
}

pub fn read_status(repo: &Repository) -> Status {
    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);
    status_options.include_ignored(INCLUDE_IGNORED);
    let mut stats = repo
        .statuses(Some(&mut status_options))
        .unwrap()
        .iter()
        .map(|f| f.status())
        .collect::<Vec<_>>();
    stats.sort_unstable();
    stats.dedup();
    Status::new().set_from_vec(stats)
}
