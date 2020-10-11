#Personal History Search

A search over your browser history.

This is a process that is stored and processed locally.
No data is sent remotely.
No authenticated data is processed (only public urls)
Automatic processors (firefox: Read the sqlite db)
Provide a home page
Provide search bar widget

![search page](https://raw.githubusercontent.com/sbeckeriv/personal_search/master/example.png)

# Pin current page

Add a bookmarklet to pin the current page you are looking at. If the url has not been index yet it will import it and pin it.

```
javascript: (function () {i fetch("http://localhost:7172/attributes?field=pinned&value=1&url="+document.location).then(data=> data.json()).then(result=> console.log(result));}());
```

Using:
tantivy for search
Yew for the front end
Actix for the server
