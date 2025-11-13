---
title: 'ubuntu fcitx输入法安装'
description: '搜狗输入法是基于fcitx框架下的，所以我们得安装fcitx才行'
slug: ubuntu-input-method-fcitx

taxonomies:
  categories: ['server', 'article']
  tags: ['ubuntu', 'fcitx', '输入法']
---

目前搜狗输入法是基于 fcitx 框架下的，所以我们得安装 fcitx 才行

首要得卸载 Ubuntu 默认的 ibus 输入法：

    sudo apt-get remove ibus

然后添加 fcitx 的 nightlyPPA：
在终端输入：

    sudo add-apt-repository ppa:fcitx-team/nightly
    sudo apt-get update

安装 fcitx 以及搜狗输入法，并设置 fcitx 为默认。

    sudo apt-get install fcitx fcitx-config-gtk fcitx-sunpinyin fcitx-googlepinyin fcitx-module-cloudpinyin fcitx-sogoupinyin
    sudo apt-get install fcitx-table-all
    sudo apt-get install im-switch
    im-switch -s fcitx -z default

接下来安装搜狗输入法皮肤，下载下面的软件并安装
http://pan.baidu.com/share/link?shareid=437349&uk=3188176680

重启。。。。。
