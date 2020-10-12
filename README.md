#Personal History Search

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

# Pin current page

Add a bookmarklet to pin the current page you are looking at. If the url has not been index yet it will import it and pin it.

```
javascript: (function () {fetch("http://localhost:7172/attributes?field=pinned&value=1&url="+document.location).then(data=> data.json()).then(result=> alert("pinned: "+document.location));}());
```

Using:
tantivy for search
Yew for the front end
Actix for the server
