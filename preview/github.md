--- 
layout: preview
title: GitHub 
category: preview
---

GitHub allows developers to create, store, manage, and share their code. 
It uses Git to provide distributed version control and GitHub itself provides 
access control, bug tracking, software feature requests, task management, 
continuous integration, and wikis for every project.

---

# Version Control System
## Theory

- You can identify every enhancement with a commit.
- More developers can contribute to the repository.
- Git provides a nice interface to manage discrepancies.
- You use branch for different features, to avoid conflicts during development.
- You solve conflicts on merges between branches.

---

# Issue Tracking System
## Theory

- Every repository has the page `Issues`
- `Issues` are defined by:
    - Title
    - Description: in markdown
    - Assignee: who will complete the issue
    - Labels: e.g. Fix, Enhacement, Documentation
    - Projects: we will come on this later
    - Milestone: we will come on this later

---

# Projects

- Each account, let it be a User or an Organization can have projects. 
- Projects are not repository dependent. 
- Broadly speaking, Projects are used to manage issues:
    - Assign issues
    - Build road-map with issues as steps
    - Check the status of issues seamlessly

---

# Milestone

- Define a clear stage of the project. E.g. 
    - Requirements' Analysis
    - MVP
    - v2.0
- Milestone group multiple issues
- Milestone are visible on the projects

---

# Discussions

- Every GitHub repository make available discussions

---

# GitHub Organization

- Multiple users work on the same repository
- Issues defined for every repository
- Issues to assign to any user in the same organization
- Project to group issues from different repositories

---

# GitHub Actions

- GitHub Actions are programs executed remotely every time an event occurs.
- Usually events are 
    - Push to some branch
    - Issue creation
- Actions can be
    - Code formatting
    - Static check on the code
    - Running tests
- Actions can also be used to create a new release  
  E.g. every time a push to the main branch occurs

---

# Version Control System
## Practice

- `git add <files>`: add files to the next commit
- `git commit -m "<descritpion>"`: create a new commit
- `git push`: upload commits to the remote repository
- `git pull`: download commits from the remote repository
- `git branch <branch-name>`: create new branch
- `git switch <branch-name>`: change branch
- `git merge <branch-name>`: merge `<branch-name>` into current branch

