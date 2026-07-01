# Dioxus 0.3 升级到 0.7 实战说明

本文基于本项目这次真实升级过程整理，目标不是讲 Dioxus 官方所有变化，而是讲:

- 这次从 `dioxus 0.3` 升级到 `0.7.9` 时，实际遇到了哪些问题
- 为什么会报错
- 应该怎么改
- 哪些地方最容易卡住

如果你后面还要升级类似项目，这份文档可以当成“排错路线图”。

## 1. 这次升级的总体结论

这次升级不是“只改版本号”就能完成的。

真正需要一起处理的，至少有 4 层：

- 依赖版本本身
- 后端框架 API 变化
- 前端 Dioxus 组件和状态管理写法变化
- 一些以前能编译、升级后就会暴露出来的类型和生命周期问题

简单理解：

- `0.3` 时很多写法“比较宽松”
- `0.7` 时类型检查、事件回调、信号状态、渲染节点这些地方都更明确了
- 所以升级时要把“模糊写法”改成“更清晰、更严格的写法”

## 2. 这次实际升级到的主要版本

本次项目最终落到的稳定版本大致如下：

- `dioxus = 0.7.9`
- `dioxus-router = 0.7.9`
- `dioxus-web = 0.7.9`
- `axum = 0.8.9`
- `diesel = 2.3.10`
- `diesel_migrations = 2.3.2`
- `reqwest = 0.13.4`
- `time = 0.3.52`
- `uuid = 1.23.4`
- `web-sys = 0.3.103`
- `wasm-bindgen = 0.2.126`

一个很重要的经验：

- 升级时尽量统一版本，不要某些 crate 还停在旧版，某些 crate 已经升到新版
- 特别是 `uuid`、`reqwest`、`wasm-bindgen`、`web-sys`、`diesel` 这种被很多模块共享的依赖

否则你会反复遇到：

- trait 对不上
- feature 对不上
- lockfile 一直抖动
- A 模块能编，B 模块不能编

## 3. 升级顺序建议

这次实践证明，最稳的顺序是：

1. 先统一依赖版本
2. 先修共享层和后端
3. 再修前端
4. 最后再做 workspace 级别检查

原因很简单：

- 前端会依赖 `shared/*`
- 后端也会依赖 `shared/*`
- 如果共享层没稳定，前后端会一起报错，很难看清真正问题

建议的检查顺序：

1. `cargo check -p uchat_domain -p uchat_endpoint`
2. `cargo check -p uchat_query`
3. `cargo check -p uchat_server`
4. `cargo check -p frontend`
5. `cargo check`

## 4. 前端最重要的变化：状态管理从 Fermi 转向 Signal/Context

这次前端升级里，一个核心改动是：

- 旧的全局状态思路不再继续沿用 Fermi 路线
- 改成了 Dioxus 0.7 更自然的 `Signal + provide_context`

项目中的实际做法在 [frontend/src/app.rs](/abs/path/E:/dioxus-x/frontend/src/app.rs:53)：

```rust
let toaster = use_signal(Toaster::default);
let post_manager = use_signal(PostManager::default);
let local_profile = use_signal(LocalProfile::default);
let sidebar = use_signal(SidebarManager::default);

provide_context(toaster);
provide_context(post_manager);
provide_context(local_profile);
provide_context(sidebar);
```

通俗理解：

- 以前像是“全局仓库”
- 现在更像“我在应用入口放进去，子组件按需拿出来”

这样做的好处：

- 代码更贴近 Dioxus 0.7 的习惯
- 组件之间的数据来源更清晰
- 类型也更容易推断

## 5. Signal 的一个大坑：很多地方需要 `mut`

这是这次升级里最容易让人烦躁的问题之一。

### 现象

代码以前看起来没问题，升级后突然报：

```rust
cannot borrow `toaster` as mutable
cannot borrow `post_manager` as mutable
cannot borrow `message` as mutable
```

### 原因

在 Dioxus 0.7 里，`Signal` 的一些写操作需要可变借用语义更明确。

比如你要：

- `write()`
- `set(...)`
- `with_mut(...)`

那变量本身往往要声明成 `mut`。

### 例子

错误写法：

```rust
let toaster = toaster.clone();
async move {
    toaster.write().error("failed", duration);
}
```

正确写法：

```rust
let mut toaster = toaster.clone();
async move {
    toaster.write().error("failed", duration);
}
```

项目里这一类修改出现在：

- [frontend/src/app.rs](/abs/path/E:/dioxus-x/frontend/src/app.rs:19)
- [frontend/src/page/home.rs](/abs/path/E:/dioxus-x/frontend/src/page/home.rs:18)
- [frontend/src/page/view_profile.rs](/abs/path/E:/dioxus-x/frontend/src/page/view_profile.rs:20)
- [frontend/src/elements/post/quick_respond.rs](/abs/path/E:/dioxus-x/frontend/src/elements/post/quick_respond.rs:39)

一个好记的经验：

- 只要你看到 `Signal`
- 又要在闭包、异步块里调用 `set/write/with_mut`
- 优先检查是不是少了 `mut`

## 6. 事件回调类型更严格了

这次升级里，`AppbarImgButton` 是一个很典型的例子。

实际修改见 [frontend/src/elements/app_bar.rs](/abs/path/E:/dioxus-x/frontend/src/elements/app_bar.rs:8)。

### 为什么这里容易出错

旧代码里常见这种写法：

```rust
click_handler: move |_| navigator.go_back()
```

升级后会出现一类错误：

```rust
the trait bound `Callback<...>: SuperFrom<{closure...}>` is not satisfied
```

### 原因

不是 `navigator.go_back()` 不能用，而是：

- 组件 props 定义的回调签名
- 实际传入的闭包参数
- 闭包返回值

这三者不完全匹配了。

### 这次项目的解决方式

把按钮组件的点击回调从“带事件参数”改成“无参回调”：

```rust
click_handler: EventHandler<()>,
```

组件内部统一转发：

```rust
onclick: move |_| {
    click_handler.call(());
},
```

然后调用方统一改成：

```rust
click_handler: move || {
    navigator.go_back();
},
```

### 为什么这样更稳

因为大多数业务按钮其实根本不需要鼠标事件对象，只是想“点了以后执行动作”。

把事件参数拿掉后：

- 调用方更简单
- 类型更稳定
- 不容易再被 `MouseEvent` / `EventHandler<T>` 卡住

## 7. 列表渲染写法要更明确

这也是 Dioxus 0.7 升级里非常常见的问题。

### 现象

你可能会看到类似错误：

```rust
Vec<Result<VNode, RenderError>>: IntoDynNode<FromNodeIterator> is not satisfied
```

### 原因

以前有些地方把 `Vec<Element>` 或 `Vec<rsx! {...}>` 直接塞进 `rsx!`，升级后不一定还能自动推断。

### 错误示意

```rust
let choices = vec![...];

rsx! {
    ol {
        {choices}
    }
}
```

### 更稳的写法

```rust
rsx! {
    ol {
        for choice in choices { {choice} }
    }
}
```

项目里这一类修复出现在：

- [frontend/src/page/home.rs](/abs/path/E:/dioxus-x/frontend/src/page/home.rs:36)
- [frontend/src/page/new_post/poll.rs](/abs/path/E:/dioxus-x/frontend/src/page/new_post/poll.rs:131)
- [frontend/src/page/trending.rs](/abs/path/E:/dioxus-x/frontend/src/page/trending.rs:48)

一句话总结：

- 如果是一个列表
- 不要赌框架会不会自动展开
- 直接 `for item in items { {item} }`

最稳

## 8. `'static` 生命周期问题会比以前更容易暴露

### 典型场景

文件上传、异步点击、循环里生成按钮，这几类地方最容易中招。

### 常见报错

```rust
closure may outlive the current function
temporary value dropped while borrowed
```

### 例子 1：文件上传事件

升级后这类写法容易报错：

```rust
oninput: |_| {
    async move {
        ...
    }
}
```

更稳的写法是：

```rust
oninput: move |_| {
    async move {
        ...
    }
}
```

也就是把外层闭包也改成 `move`，明确把需要的值拿进去。

项目位置：

- [frontend/src/page/new_post/image.rs](/abs/path/E:/dioxus-x/frontend/src/page/new_post/image.rs:67)
- [frontend/src/page/edit_profile.rs](/abs/path/E:/dioxus-x/frontend/src/page/edit_profile.rs:24)

### 例子 2：循环中绑定点击事件

如果你直接在迭代里用临时引用，升级后更容易出生命周期问题。

错误思路：

```rust
onclick: move |_| vote_onclick(post_id, choice.id)
```

如果 `choice` 是迭代中的借用对象，最好先把需要的值单独拷出来：

```rust
let choice_id = choice.id;
let choice_description = choice.description.clone();
```

然后再在闭包里用 `choice_id`。

项目位置：

- [frontend/src/elements/post/content.rs](/abs/path/E:/dioxus-x/frontend/src/elements/post/content.rs:96)

## 9. 表单的 `prevent_default` 老写法要改

这次升级后，`prevent_default: "onsubmit"` 还能编译，但会报警告，提示它已经不推荐这样用了。

### 旧写法

```rust
form {
    prevent_default: "onsubmit",
    onsubmit: form_onsubmit,
}
```

### 推荐写法

```rust
form {
    onsubmit: move |evt| {
        evt.prevent_default();
        form_onsubmit(evt);
    },
}
```

### 为什么这样更好理解

旧写法像“给标签挂一个声明式属性”。

新写法更直接：

- 事件来了
- 先阻止默认提交
- 再执行我们自己的业务逻辑

项目里这一类修复出现在：

- [frontend/src/page/login.rs](/abs/path/E:/dioxus-x/frontend/src/page/login.rs:164)
- [frontend/src/page/register.rs](/abs/path/E:/dioxus-x/frontend/src/page/register.rs:143)
- [frontend/src/page/new_post/chat.rs](/abs/path/E:/dioxus-x/frontend/src/page/new_post/chat.rs:176)
- [frontend/src/page/new_post/image.rs](/abs/path/E:/dioxus-x/frontend/src/page/new_post/image.rs:208)
- [frontend/src/page/new_post/poll.rs](/abs/path/E:/dioxus-x/frontend/src/page/new_post/poll.rs:283)
- [frontend/src/page/edit_profile.rs](/abs/path/E:/dioxus-x/frontend/src/page/edit_profile.rs:365)
- [frontend/src/elements/post/quick_respond.rs](/abs/path/E:/dioxus-x/frontend/src/elements/post/quick_respond.rs:81)

补充一个更简单的经验：

- 如果按钮本来就不是提交表单用的
- 直接加 `type="button"`
- 通常比手动阻止默认行为更简单

## 10. 后端升级不是只有 Dioxus，Axum 0.8 也有关键变化

虽然用户感知更强的是前端，但这次后端也有几个很关键的升级点。

### 10.1 Axum 启动方式变了

旧项目常见写法是：

```rust
axum::Server::bind(...)
```

这次升级后改成了：

```rust
let listener = tokio::net::TcpListener::bind(args.bind).await?;
axum::serve(listener, router.into_make_service()).await
```

项目位置：

- [backend/server/src/bin/api.rs](/abs/path/E:/dioxus-x/backend/server/src/bin/api.rs:66)

通俗理解：

- 以前像“Axum 自己顺便帮你管理监听”
- 现在更像“你先把 listener 准备好，再交给 Axum 去服务”

这样做更贴近当前 Axum 的 API 设计。

### 10.2 提取器和 trait 适配更严格

升级到较新的 Axum 后，一些自定义 extractor、handler trait bound 会更挑剔。

这次项目里为了兼容当前写法，引入了：

- `async-trait = "0.1.89"`

这不是“新框架一定需要 async-trait”，而是当前项目的抽象方式继续保留时，它能帮助过渡。

## 11. Diesel 和连接池是后端升级最容易爆炸的地方

这次后端真正最难的地方，不是 Axum，而是 Diesel 生态配套。

### 现象

理论上你会以为：

- 升级 `diesel`
- 升级 `bb8-diesel`
- 改几个 import

就结束了。

实际不是。

### 这次项目的真实情况

原来的连接池方案和当前稳定依赖组合并不顺滑，最后采取了一个更稳定也更容易维护的做法：

- 不再强依赖复杂的异步池适配层
- 改成保存数据库 URL
- 每次需要时建立 `PgConnection`

项目位置：

- [backend/query/src/util.rs](/abs/path/E:/dioxus-x/backend/query/src/util.rs:1)

关键结构大概变成：

```rust
pub struct AsyncConnectionPool {
    connection_url: Arc<str>,
}
```

获取连接时：

```rust
connect(self.connection_url.as_ref())
```

### 为什么这样做

因为升级的目标首先是：

- 先恢复可编译
- 先回到稳定 crates.io 版本
- 先减少不确定性

如果为了保留旧的“异步池抽象”而引入更多不稳定适配，升级成本会迅速变高。

这个策略很适合老项目升级：

- 先让系统站起来
- 再考虑后续是不是要引入更复杂的池化优化

## 12. 密码学相关依赖要特别注意版本配套

这次后端还有一个很典型的问题：

- `argon2`
- `password-hash`
- `rsa`
- `rand`

这些库不是单独看版本号就行，它们之间常常有“配套关系”。

### 这次项目里的关键处理

- `argon2 = 0.5.3`
- `password-hash = 0.5.0`
- `rsa = 0.9.10`
- `rand = 0.8.6`

这里尤其要注意 `rand`。

如果你把某些库升到了新版，而 `rsa` 或其他库还在旧的 `rand_core` 体系里，就容易出现：

- trait 不匹配
- RNG 类型不兼容
- `CryptoRngCore` / `RngCore` 对不上

通俗理解：

- 不是“都叫随机数库”就能混用
- 同一条生态链里，最好尽量用兼容的一组版本

## 13. `just check` 不一定只检查你当前目录

这是这次验证阶段一个很容易误判的点。

看起来你在：

- `frontend/` 下执行 `just check`
- `backend/` 下执行 `just check`

像是只会检查当前目录。

但本项目实际不是这样。

根目录 [justfile](/abs/path/E:/dioxus-x/justfile:1) 的 `check` recipe 是：

```just
check:
    cargo check -p frontend --target wasm32-unknown-unknown
    cargo check --workspace --exclude frontend
```

所以不管你在 `frontend` 还是 `backend` 子目录运行 `just check`，最终都会跑到 workspace 级别。

这次就是因此暴露了：

- `tools/project-init` 里的 `Spinner` 可变借用错误

也就是说：

- “前端检查失败”
- 不一定真是 frontend 自己的问题
- “后端检查失败”
- 也不一定真是 backend 自己的问题

先看 recipe 真正执行了什么，非常重要。

## 14. 这次升级里最值得记住的排错思路

如果以后还要升级类似项目，建议按这个思路走。

### 14.1 不要一上来就全量修

先缩小范围：

- 先 shared
- 再 backend/query
- 再 backend/server
- 最后 frontend

### 14.2 优先修“高频同类错误”

比如这次前端里最先值得统一修的是：

- `Signal` 缺 `mut`
- 事件回调签名不匹配
- 列表渲染写法
- `prevent_default` 老写法

一类问题修完，往往能一起消掉十几个错误。

### 14.3 遇到 trait 错误先怀疑“签名不一致”

尤其是这类报错：

```rust
SuperFrom
SuperInto
Callback<...>
EventHandler<...>
```

通常不要先怀疑框架坏了。

优先检查：

- props 定义的参数类型
- 调用处闭包参数
- 闭包返回值

这三者有没有统一。

### 14.4 遇到生命周期报错先把临时值拆出来

尤其在：

- `map(...)`
- `iter(...)`
- `onclick: move |_| ...`
- `async move`

这些组合场景中。

经验公式：

- 先把 `id`、`String`、`clone()` 的值单独存到局部变量
- 再把这个局部变量捕获进闭包

通常会立刻好很多。

## 15. 这次升级后，项目最终达成的状态

本次升级最终已经验证通过的内容包括：

- `cargo check` 全工作区通过
- `cargo check -p frontend` 通过
- `cargo check -p uchat_server` 通过
- `cargo check -p uchat_query` 通过
- `cargo check -p uchat_crypto` 通过
- `backend` 目录执行 `just check` 通过
- `frontend` 目录执行 `just check` 通过

说明这次升级已经不只是“版本号改完了”，而是：

- 依赖层对齐完成
- 业务代码适配完成
- workspace 检查链路恢复完成

## 16. 给以后升级者的建议

如果你未来还要从 Dioxus 老版本继续升级，最建议记住下面几条：

- 不要把升级理解成“改 Cargo.toml”
- 先接受“业务代码一定要跟着改”这个事实
- 优先把状态管理、事件回调、列表渲染这三类前端问题统一处理
- 后端先盯住 Diesel、连接池、Axum 启动方式
- 遇到大面积报错时，不要一个个零碎修，先找“同类根因”
- 子目录执行的 `just` 命令，不一定只影响子目录，先看 recipe

最后用一句最通俗的话总结这次升级：

> 从 Dioxus 0.3 升级到 0.7，最大的变化不是“语法变了多少”，而是“框架希望你把状态、事件、渲染边界写得更明确”。只要顺着这个方向改，报错会越来越少，代码也会越来越稳。
