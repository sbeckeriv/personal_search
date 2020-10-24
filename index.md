## Private search
Often when looking for a site I visited before I need to remember a bit of the title or the url. The contents of the page are not searchable from omnibars but the contents are what I remember the most. Think google for just pages you have already seen. Also its all local to your computer. 

![Image of the search](https://raw.githubusercontent.com/sbeckeriv/personal_search/master/images/search.png)

### Process 

As you visit sites in your browser it records the history of the url, the time and other metadata. Using these files the indexer reads thems and re-requests the site again. The reason to request the site again is to keep private authed data out of the index. Honestly gmail does a fine job of search I dont need another search for my email*.

The html is downloaded, stored and processed in to a search index[1]. The server loads up the index and finds your results. Since we have the original file there is also an offline** view of the file when it was indexed.


### Install

At the moment prebuild binaries are under releases [https://github.com/sbeckeriv/personal_search/releases](https://github.com/sbeckeriv/personal_search/releases) . You need to figure out how to get them to run (cron for sync commands) or start at startup (for the server). 

Once the service is running visit [http://localhost:7172/](http://localhost:7172/)  click the settings icon. Add domains you do not want to index and turn on indexing.

Run the indexer of your choice. The first run should backfill your last 1000 pages. After that it will only index new pages.

The index and configuration files are stored at HOME/.config/private_search

##### Pin current page

Add a bookmarklet to pin the current page you are looking at. If the url has not been index yet it will import it and pin it. This will depend on the sites cors configuration.

```
javascript: (function () {fetch("http://localhost:7172/attributes?field=pinned&value=1&url="+document.location).then(data=> data.json()).then(result=> alert("pinned: "+document.location));}());
```

### Features

* Linux, Mac and Windows builds
* Chrome indexer
* Brave indexer
* Firefox indexer
* Offline Viewin'
* Full-text search, Natural query language (e.g. (michael AND jackson) OR "king of pop"), Phrase queries search Phrase queries search (e.g. "michael jackson") [1]
* hierarchical tags "/tag/base" "/tag/base/1" "/tag/base/2"
* keywords - /tag are user created facets /keywords come directly from the html keywords
* Pinning/Staring
* Hide url
* Index ingore list
* bookmarklet to pin a page your are viewing ***
* opensearch - Allows for custom search engine from the omnibar. I can type "ps postgres" and go to the postgres results.

Settings:
![Image of the settings](https://raw.githubusercontent.com/sbeckeriv/personal_search/master/images/settings.png)
Tag search
![Image of the settings](https://raw.githubusercontent.com/sbeckeriv/personal_search/master/images/tags.png)
Offline view with link to live site
![Image of the settings](https://raw.githubusercontent.com/sbeckeriv/personal_search/master/images/offline.png)



### Details

The server is build with Actix [2] and the front end is build with Yew. Its fast. Loading the search index (25k urls), searching, and parsing the results to json takes 14ms. Since it is all local there is no network lag issues. I dont know how to mesure Yew but the results feel instant as you type [add video]. 

25000 urls is taking up about 475mb. Since the main data is stored in the index there is no need for a backup.  

Don't like the program? Your data is yours. You can export all the content to brotli compressed json files. 

### Upgrading

No story yet. 


[1]  [https://github.com/tantivy-search/tantivy](https://github.com/tantivy-search/tantivy)

[2] [https://github.com/actix/actix-web](https://github.com/actix/actix-web)

[3] [https://yew.rs/docs/en/](https://yew.rs/docs/en/)

*With a custom indexer you can index *anything* you like, Auth pages, emails.. videos..

** images are not stored and will load if connected to the internet. 

*** depends on cors of the site but works well enough. Could also be used to only index pinned sites if you dont use the indexers.
