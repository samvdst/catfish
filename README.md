# catfish 🥸

**Because sometimes files pretend to be something they’re not.**  
_(No matter the name or location, catfish will find out if they’re the same.)_

> I needed this functionality today and threw this tool together.
> I decided to share it here, in case someone else finds it useful!

## Why “catfish”?

- **cat** is a Unix tool.
- **fish** … well, we’re fishing for the truth in your file system.
- A catfish is a sneaky creature, just like files that might be identical under different names or locations.
- catfish 🥸 unmask duplicates for what they really are.

## What does it do?

`catfish` recursively scans two folders—let’s call them “left” and “right”—and hashes every file (using SHA256).

- If a file in the **right** folder has the same content (hash) as **any** file in the **left**
  folder, it won’t be listed.
- We **don’t** check the file’s location in the left folder. Any matching hash anywhere in “left” is
  enough to exclude it.
- We **don’t** check for duplicates within the left folder itself—if “left” has duplicates, that’s
  not our concern.
- We can optionally ignore duplicates in the **right** folder, so that only the **first** occurrence
  of any given hash in “right” is shown.

**Backstory**:

Some time ago, I switched from cloud provider X to cloud provider Y. I had both drives fully synced
locally (a full copy, not a "lite" sync), so I copied all my files from X to Y and then turned off
sync for X. But I forgot to delete the local X folder, and ended up adding new files to it by
mistake. When I went to delete it, a simple path comparison with Y wasn't enough because I'd moved
and renamed files in Y, which would have caused a lot of false positives. What I really needed was
to find out which files in X didn't exist anywhere in Y - so I could copy them over if necessary,
and then safely delete X without losing anything important.

## Installation

1. **Ensure you have [Rust](https://www.rust-lang.org/) and Cargo installed.**
2. Run:
   ```bash
   cargo install catfish
   ```
   or clone this repo and:
   ```bash
   git clone https://github.com/samvdst/catfish.git
   cd catfish
   cargo install --path .
   ```
3. That’s it! You can now run `catfish` from anywhere.

## Usage

```bash
catfish [OPTIONS] <LEFT_FOLDER> <RIGHT_FOLDER>
```

- **`-i, --ignore-duplicates`**: if there are multiple files with the same hash in `RIGHT_FOLDER`, only list the first occurrence.

## Example

Suppose we have two folders: foo (left) and bar (right). In bar, we have a file that appears twice with identical content.

```bash
catfish foo bar
Files in "bar" but not in "foo":
f2d30353acf140ed51b1343368255c1201a7ee898acd60b25e207ff75555e12c bar/example.txt
f2d30353acf140ed51b1343368255c1201a7ee898acd60b25e207ff75555e12c bar/example_dupe.txt
af89f7d49b0c8ded732a9a2b3aff738cd1a3c1cd0d3635742adfee47faa31cba bar/another_file.txt
```

If we then run:

```bash
catfish foo bar --ignore-duplicates
Files in "bar" but not in "foo":
f2d30353acf140ed51b1343368255c1201a7ee898acd60b25e207ff75555e12c bar/example.txt
af89f7d49b0c8ded732a9a2b3aff738cd1a3c1cd0d3635742adfee47faa31cba bar/another_file.txt
```

## Contributing

Ideas, improvements, and pull requests are always welcome.
But please note: I can’t guarantee that I’ll have much time to work on this.
So if you open a PR, thanks in advance for your patience!
