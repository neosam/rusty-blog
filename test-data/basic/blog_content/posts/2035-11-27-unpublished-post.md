---
title: New blog
date: 2035-11-27T09:00:00+02:00
author: neosam
---

Once again, I try to start to run a new blog.  But now, I write my own blog
software.  Well I know this is nonsense since there is already a lot of much
better weblog software out there and what I write doesn't add any value.

So, why am I doing this?  I try to get some more experience with Rust in
web development.  For a basic blogging software, I just needed 236 lines of
code.  And currently it provides these features:

* Threaded web server thanks to actix-web.
* Templating system to create custom themes thanks to tinytemplate.
* Static file share to host css files.
* Display posts which are stored as markdown files thanks to the markdown
  library.
* Display a list of posts.
* And loggging! (= env_logger)

A lot of very important features are still missing.  For example RSS feeds and
comments.  The implementation I made was not very smart and for example the
template engine gets initialized on every page request.  Since I don't expect
many visitors on this page, this is good enough for now but this needs to be
fixed.  Hopefully I can make an update in 2019.

The code is available at [https://github.com/neosam/rusty-blog](https://github.com/neosam/rusty-blog "Github")
 
