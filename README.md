# Achievements [![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/achievements.svg
[crates.io]: https://crates.io/crates/achievements

**Achievements is a command line tool to track and celebrate the passage of time.**

**NOTE**: Personal/experimental project.

Install using Cargo:

```Bash
$ cargo install achievements
```

Then run by
```Bash
$ achievements

Moon landing: 19984 days 💎💎💎💎💎
Berlin Wall Fall: 12567 days 💎💎💎
```

**IMPORTANT**: The way the number of days/months/etc is calculated
is _very simple_ and **not** accurate. A day is ~86400 seconds.
A month is ~30 days, a year is ~365 days etc...this means the reported
intervals are only a rough indication and they can be wrong.

For example:
- the accurate number of days since the Moon landing should
  be 19985 days but the tool reports 19984 days (1 day off)
- the accurate number of days since the Berlin Wall fall should
  be 12568 days but the tool reports 12567 days (1 day off)
