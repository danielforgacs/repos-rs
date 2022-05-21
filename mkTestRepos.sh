here=$PWD
td=$(mktemp -d --suffix "__repos_test")
export DEVDIR_ORIGINAL=$DEVDIR
export DEVDIR=$td
echo $td

mkdir $td/no_commit && cd $td/no_commit && \
    git init

mkdir $td/just_init && cd $td/just_init && \
    git init && \
    git commit --allow-empty -m 'Init.'

mkdir $td/new_file && cd $td/new_file && \
    git init && \
    touch untracked_new_file

mkdir $td/new_file_staged && cd $td/new_file_staged && \
    git init && \
    git commit --allow-empty -m 'Init.' && \
    echo "nothing." > staged_new_file && \
    git add staged_new_file

mkdir $td/modified_files && cd $td/modified_files && \
    git init && \
    git commit --allow-empty -m 'Init.' && \
    echo "nothing." > some_file && \
    git add some_file && \
    git commit -m '1st change' && \
    echo "1st update." >> some_file

mkdir $td/many_branches && cd $td/many_branches && \
    git init && \
    git commit --allow-empty -m 'Init.' && \
    git branch dev && \
    git branch release && \
    git branch hotfix && \
    git branch feature && \

mkdir $td/many_branches_w_branch_checked_out && cd $td/many_branches_w_branch_checked_out && \
    git init && \
    git commit --allow-empty -m 'Init.' && \
    git checkout -b dev && \
    git checkout -b release && \
    git checkout -b hotfix && \
    git checkout -b feature && \

mkdir $td/branches_on_branch && cd $td/branches_on_branch && \
    git init && \
    git commit --allow-empty -m 'Init.' && \
    git checkout -b release && \
    git checkout -b dev && \
    git checkout -b hotfix && \
    git checkout -b zzz && \
    git checkout -b sort && \
    git checkout -b qqq && \
    git checkout -b lll && \
    git checkout -b mmm && \
    git checkout -b aaa && \

mkdir $td/grouped_branches && cd $td/grouped_branches && \
    git init && \
    git commit --allow-empty -m 'Init.' && \
    git checkout -b dev/hotfix/bug1 && \
    git checkout -b dev/feature/new1 && \
    git checkout -b dev/dev/feature1 && \
    git checkout -b dev/dev/feature2 && \
    git checkout -b dev/release/release1 && \
    git checkout -b dev/release/release2

mkdir $td/changes_on_a_branch && cd $td/changes_on_a_branch && \
    git init && \
    git commit --allow-empty -m 'Init.' && \
    git checkout -b dev && \
    git checkout -b release && \
    git checkout -b feature && \
    touch some_file

mkdir $td/changes_on_a_branch_b && cd $td/changes_on_a_branch_b && \
    git init && \
    git commit --allow-empty -m 'Init.' && \
    git checkout -b dev && \
    echo "0" >> $td/changes_on_a_branch_b/some_file && \
    git add $td/changes_on_a_branch_b/some_file && git commit -am 'somfile' && \
    echo "11" >> $td/changes_on_a_branch_b/some_file

cd $here
