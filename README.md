[![Build](https://github.com/John15321/proc-opt/actions/workflows/build.yml/badge.svg)](https://github.com/John15321/proc-opt/actions/workflows/build.yml)
[![Code check](https://github.com/John15321/proc-opt/actions/workflows/rust.yml/badge.svg)](https://github.com/John15321/proc-opt/actions/workflows/rust.yml)
[![Docs](https://github.com/John15321/proc-opt/actions/workflows/docs.yml/badge.svg)](https://github.com/John15321/proc-opt/actions/workflows/docs.yml)
[![Package](https://github.com/John15321/proc-opt/actions/workflows/package.yml/badge.svg)](https://github.com/John15321/proc-opt/actions/workflows/package.yml)
[![Tests](https://github.com/John15321/proc-opt/actions/workflows/tests.yml/badge.svg)](https://github.com/John15321/proc-opt/actions/workflows/tests.yml)

# Process Optimization

Process optimization library.

## Features

---

Algorithms (TODO):

* Carlier
* Schrage
* Genetic Algorithm
* Simulated Annealing
* Particle Swarm
* Tabu Search

## Development and branching strategy

---

Specfic branch names:

* Main: "`main`"
* Develop: "`branch_develop`"
* Feature: "`ISSUENUMBER_short_lowercase_description`" e.g. "`6_make_better_readme`"

![Branching strategy: https://www.atlassian.com/git/tutorials/comparing-workflows/gitflow-workflow](img/feature_branches.svg)


## Useful tips

* For running test in a more visually appealing way use the
    `cargo-nextest` crate.
* For drawing the module tree of your crate use `cargo-modules` crate
* For manipulating the Cargo.toml in cmd use `cargo-edit` crate
* For cheking if your deps have security vulnerabilities use `cargo-audit` crate
* Tox like task definition using `cargo-make`
* For smootinh the release process use `cargo-release`

## Credits

---

This package was created with Cookiecutter, and the
`John15321/cookiecutter-krabby-patty` project template.

Cookiecutter: <https://github.com/audreyr/cookiecutter>

`John15321/cookiecutter-krabby-patty`: <https://github.com/John15321/cookiecutter-krabby-patty>
