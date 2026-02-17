# Contributing to AI Guardian
=====================================

Thank you for considering contributing to AI Guardian, an autonomous AI assistant for web security and performance optimization. This document outlines the steps and guidelines to follow when contributing to the project.

## Setup
--------

To start contributing, follow these setup steps:

1. **Fork the repository**: Create a fork of the AI Guardian repository to your GitHub account.
2. **Clone the repository**: Clone the forked repository to your local machine using `git clone https://github.com/your-username/ai-guardian.git`.
3. **Install dependencies**: Install the required dependencies for the project, including Rust, TensorFlow, and nginx.
4. **Configure your environment**: Configure your environment variables to point to the installed dependencies.

## Branching
------------

When creating a new branch, use the following naming conventions:

* **feat/**: New features or enhancements
* **fix/**: Bug fixes
* **docs/**: Documentation changes

## Commits
---------

AI Guardian follows the Conventional Commits specification for commit messages. When committing changes, use the following format:

`type(scope): brief description`

* **type**: One of `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`
* **scope**: Optional scope of the commit (e.g. `nginx`, `tensorflow`, `rust`)
* **brief description**: Brief description of the changes made

Example: `feat(nginx): add support for nginx 1.20`

## Pull Request Checklist
------------------------

Before submitting a pull request, ensure that:

* [ ] You have followed the branching and commit guidelines
* [ ] Your code is formatted according to the project's code style guidelines (see below)
* [ ] You have run the tests and ensured they pass (see below)
* [ ] Your changes are backwards compatible with previous versions of the project

## Code Style
-------------

AI Guardian uses the following code styles:

* **Rust**: Follow the Rust style guidelines, using `rustfmt` to format code
* **TensorFlow**: Follow the TensorFlow style guidelines, using `clang-format` to format code
* **nginx**: Follow the nginx style guidelines, using `nginx-style-guide` to format configuration files

## Running Tests
----------------

To run the tests, use the following command:

cargo test

This will run the unit tests and integration tests for the project.

## Reporting Bugs
----------------

When reporting bugs, please include the following information:

* **Description of the bug**: A clear and concise description of the bug
* **Steps to reproduce**: Step-by-step instructions to reproduce the bug
* **Expected behavior**: The expected behavior of the code
* **Actual behavior**: The actual behavior of the code

You can report bugs by opening an issue on the AI Guardian repository.