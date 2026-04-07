#### 修复tailwind css样式未加载
修改tailwind.config.js中content为
```
content: [
    "./frontend/index.html",
    "./frontend/src/**/*.{html,rs,scss}"
],
```

windows中修改Trunk.win.toml为
```
  "npx --yes @tailwindcss/cli -i frontend/src/tailwind.css -o $TRUNK_STAGING_DIR/tailwind.css -m -c frontend/tailwind.config.js",
```
注意修改` $TRUNK_STAGING_DIR/tailwind.css`斜杠

在tailwind css v4中只需导入
```
@import "tailwindcss";
```
一行即可