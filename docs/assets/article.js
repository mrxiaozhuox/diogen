marked.setOptions({
    langPrefix:'language-',
})

document.getElementById("content").innerHTML = marked.parse(
  document.getElementById("meta-content").innerText
);

hljs.highlightAll();