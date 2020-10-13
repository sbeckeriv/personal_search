# Personal History Search

A search over your browser history.

This is a process that is stored and processed locally.
No data is sent remotely.
No authenticated data is processed (only public urls)
Automatic processors (firefox: Read the sqlite db)
Provide a home page
Provide search bar widget

![search page](https://raw.githubusercontent.com/sbeckeriv/personal_search/master/example.png)

# Status

Alpha level. You need to install start jobs and cron jobs manually. You might need to run some commands to clean things up. The UI works but it is rough.

`cargo run --release --bin server --features server`

open http://localhost:7172/index.html and configure domains to ignore and then turn on the indexer.

manually run the indexer

`cargo run --bin firefox_sync --features="sync" --release`
`cargo run --bin chrome_sync --features="sync" --release`

the first run should index your last 1000 history if you turned on the index in the site config.

cron the indexer..

# Pin current page

Add a bookmarklet to pin the current page you are looking at. If the url has not been index yet it will import it and pin it. This will depend on the sites cors configuration.

```
javascript: (function () {fetch("http://localhost:7172/attributes?field=pinned&value=1&url="+document.location).then(data=> data.json()).then(result=> alert("pinned: "+document.location));}());
```

# Open search

Open search is supported. In firefox I added it as a search engine with a keyword. I can type "ps postgres" and it will go to http://localhost:7172/index.html?q=postgres

# Development

use `PS_INDEX_DIRECTORY=test/private_search` to test

Test files

places.sqlite is a 30 url file to test different states. chrome History is about the same. I use firefox more.

`cargo run --bin firefox_sync --features="sync" -- --db test/places.sqlite --backfill`

`cargo run --bin chrome_sync --features="sync" -- --db test/History --backfill`

index a single url
`cargo run --bin personal_search -- --import_url https://docs.rs/tantivy/0.13.1/tantivy/schema/struct.FieldValue.html`

query test
`cargo run --bin personal_search -- --query music`

more options under help
`cargo run --bin personal_search -- --help`

Yew:

under the search folder `yarn build` and refresh the page. It uses parcel. rust files are under crate folder. In there run cargo build.

# Using

tantivy for search
Yew for the front end
Actix for the server
Actix for the server
