// This is what jspm link tinymce would generate
// importmap.js injection script
(map => {
  document.head.appendChild(Object.assign(document.createElement("script"), {
    type: "importmap",
    innerHTML: JSON.stringify({
      imports: map.imports,
      scopes: map.scopes
    })
  }));
})
({
  "imports": {
    "tinymce": "https://ga.jspm.io/npm:tinymce@7.7.0/tinymce.min.js",
    "tinymce/": "https://ga.jspm.io/npm:tinymce@7.7.0/"
  },
  "scopes": {}
});
