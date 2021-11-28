# add-show-users.rs

simple rust code for add and display users to/from list

## Requirements
* [Rust](https://www.rust-lang.org/tools/install)

## Installation
```bash
$ git clone https://github.com/TheAwiteb/add-show-users.rs
$ cd add-show-users.rs
```

## Running
```bash
# Path is add-show-users.rs
$ cargo run
```

## Output

```
'q' for quit.
'a' for add new user.
's' for show users.
>>: a

👱 Username: A
📧 Email: A@A.com

'q' for quit.
'a' for add new user.
's' for show users.
>>: a

👱 Username: B
📧 Email: B@B.com

'q' for quit.
'a' for add new user.
's' for show users.
>>: s

===🆔=== User id 1 ===🆔===

👱 Username: A
📧 Email: A@A.com

===🆔=== User id 2 ===🆔===

👱 Username: B
📧 Email: B@B.com

'q' for quit.
'a' for add new user.
's' for show users.
>>: q

```