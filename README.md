个人学习宏的仓库，完成`procedural macros workshop`的题目。

- [ ] Builder
- [ ] CustomDebug
- [ ] seq!
- [ ] sorted
- [ ] bitfield

一些图片资料放在了`assets`文件夹。


# 环境准备
```shell
cargo install cargo-expand
```

# 一些hint
- 在和`README.md`同级的`main.rs`中粘贴进测试代码(tests中的)，用`cargo expand`就可以看到展开后的代码。
- 派生宏是将返回的`TokenStream`追加到被展开的代码之后。原来的代码不会被修改。