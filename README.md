[![Build Status](https://travis-ci.com/neosam/rusty-blog.svg?branch=master)](https://travis-ci.com/neosam/rusty-blog)

# rusty-blog
A blog software which works with files only written in rust

## Pre-Alpha
This software is basically just something I write to do some web development
in Rust.  Maybe this will never really be stable at all - there is plenty of
weblog software out there anyway.

## What it does
It tries to be a weblog software and it tries to be something in the middle
between a statically hosted blog like Jekyll but doesn't actually generate
static html files.  Instead it uses the files in the background but creates
the html files dynamically.  By this, no database is required and updates
could be prepared with git or by switching the working directories with
softlinks.  But since it's dynamic, it is possible to support user comments.

## What it doesn't support
At the moment when I've written this README, the whole code base is in main.rs
only and this file contains less thatn 240 lines of code.  So there is a lot of
stuff missing.

### It's not optimized at all
The current code is brutally unoptimized.  The template engine for example
gets initialized on every request.  The generated content could be cached which
is not done.

### No RSS
There is no news feed integrated yet.

### No comments
Use comments are not yet implemented.

### Will I ever update it?
I do hobby projects since I was about 12 years and I never ever really finished
one of them.  So there is a high chance, I will not have an actual good version
of this project.  This is also not my goal.


