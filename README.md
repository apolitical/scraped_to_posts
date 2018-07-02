Scraper Articles to WP Posts
============================

[![GitHub release](https://img.shields.io/github/release/apolitical/scraped_to_posts.svg)](https://github.com/apolitical/scraped_to_posts/releases)
[![GitHub license](https://img.shields.io/github/license/apolitical/scraped_to_posts.svg)](https://github.com/apolitical/scraped_to_posts/blob/master/LICENSE)

A simple command line utility for streaming articles scraped from other websites into stubs on our
feed. These appear with a short summary and a link to the original article.


Usage
-----

To be used with [content-classifier](https://github.com/apolitical/content-classifier) and
[scrappy-doo](https://github.com/apolitical/scrappy-doo/)

Each program is designed to stream from one to the next by outputing json documents one line at a
time.

```bash
$ pipenv run scrape {scraper_name} -t jsonlines -o - 2> {error_file} \ # Get data from other sites
  | python classify_scraped_content.py \                               # Pipe to classifier
  | ./scaped_to_posts                                                  # Upload to database
```

You will need `DATABASE_URL` in your environment, or you can use a `.env` file.

Contributing
------------

If you want to help, that's brilliant! Have a look at our [Contributing Guide](CONTRIBUTING.md). We also adhere to a
[Code of Conduct](CODE_OF_CONDUCT.md), so please check that out, it includes details on who to contact if you have any
concerns.
