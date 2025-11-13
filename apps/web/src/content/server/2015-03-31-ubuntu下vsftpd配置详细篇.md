---
title: 'ubuntu下vsftpd配置详细篇'
description: '本人配置的ftp服务器的设计要求如下匿名用户可登录浏览，但不能下载设置4个不同等级的用户使用此ftp服务器(虚拟用户)，分别如下用户名:nan306 路径/home/vsftpd 管理用户，可对ftp服务器的所有'
slug: ubuntu-vsftpd

taxonomies:
  categories: ['server', 'article']
  tags: ['ubuntu', 'vsftpd']
---

### 1、 本人配置的 ftp 服务器的设计要求如下：

（1）、匿名用户可登录浏览，但不能下载
（2）、设置 4 个不同等级的用户使用此 ftp 服务器(虚拟用户)，分别如下
用户名:nan306 路径/home/vsftpd 管理用户，可对 ftp 服务器的所有文件进行任何操作
用户名:down 路径/home/vsftpd/down 下载用户，只可下载此目录下的文件
用户名:upload 路径/home/vstfpd/upload 上传用户，在此目录下可上传下载删除等操作
用户名:wsn 路径/home/vsftpd/wsn 实验室文件管理目录，此用户目录为本实验室内部资料，其下挂在 down 虚拟路径

### 2、vsftpd 的安装

    $ sudo apt-get install vsftpd

安装完后不用做任何配置既可用匿名方式进行访问,默认的 ftp 文件夹为/srv/ftp

### 3、vsftpd 的开始、关闭和重启

    $sudo /etc/init.d/vsftpd start #开始
    $sudo /etc/init.d/vsftpd stop #关闭
    $sudo /etc/init.d/vsftpd restart #重启

关于 vsftpd 的开机自启动，当 vsftpd 装好后是默认开机自启动的，如果不需要可关闭，关闭方法很多，网上自己看吧,我用一条命令

    $sudo mv /etc/rc2.d/S20vsftpd /etc/rc2.d/K20vsftpd

具体请参考开机自启动文章 http://hi.baidu.com/jidaxiaobeibei/blog/item/e16309446cc0b237879473d6.html

### 3、vsftpd 的配置

这一块网上信息是铺天盖地，关于配置参数我这里就不一一列举，初次接触的朋友参考http://os.51cto.com/art/201003/189123.htm这个网址，高级点的配置参考http://www.517sou.net/blogview.asp?logID=515#commmark_241。如果没接触国建议先看看第一个网址里面的内容，简单明了。
关于配置这块初次配置 ftp 的朋友我的建议是别一下把所有配置全看完就照着做，这样都配完了 ftp 出了问题都不知道哪步错的，一步步来，配一步测试一步。

我的配置过程如下：
vsftpd 的配置很简单，就是打开/etc/vsftpd.conf 文件，

    $sudo vi /etc/vsftpd.conf ＃我这里用ssh登录服务器配置的，所以用vi，本机的话可以直接 sudo gedit /etc/vsftpd.conf

然后对参数进行修改，修改完咧就重启服务器

    $sudo /etc/init.d/vsftpd restart #重启就可以实现配置功能。

对本人配置的服务器中 vsftpd.conf 的参数使用如下：

    listen=YES #启用独立vsftpd服务器
    #listen_ipv6=YES 不需要，注释掉
    anonymous_enable=YES #本服务器需要匿名访问
    local_enable=YES #要用虚拟用户，需要本地访问的（关于本地用户和虚拟用户不要迷茫，稍后解释）
    write_enable=YES #需要本地用户对文件进行修改和删除
    #local_umask=022 FTP上传文件权限 ，默认是077，本服务器每个虚拟用户都有上传权限设置，总的就留空注释掉了
    #anon_upload_enable=YES 是否允许匿名用户上传，不需要匿名上传注释掉
    #anon_mkdir_write_enable=YES 是否允许匿名用户的写和创建目录的权限，不要匿名管理，注释掉
    dirmessage_enable=YES 当切换目录时，是否显示该目录下message隐藏文件的内容，这个用来显示登录信息的 设为YES
    message_file=Welcome 显示的欢迎信息，在ftp目录下建Welcome的文件，里面写上登录信息即可，一般人常用.message隐藏文件
    xferlog_enable=YES 是否激活上传和下载的日志，需要
    connect_from_port_20=YES 是否启动FTP数据端口20的连接请求 需要
    chown_uploads=YES 是否改变上传文件的所有者，我这里需要改变上传文件所有者
    chown_username=virtual 改变上传文件的所有着为virtual，这个virtual用户一会要新建的，用来实现虚拟用户登录
    xferlog_file=/var/log/vsftpd.log 上传/下载日志文件所默认的路径
    xferlog_std_format=YES 是否使用标准的ftpd xferlog日志格式
    idle_session_timeout=600 是否将在用户会话空闲10分钟后被中断
    data_connection_timeout=120 是否将在数据连接空闲2分钟后被中断
    #nopriv_user=ftpsecure 是否运行vsftpd需要的非特殊系统用户默认nobody 不需要
    #async_abor_enable=YES 是否是否允许运行特殊的FTP命令async 不要
    ascii_upload_enable=YES 是否启用上传的ascii传输方式 需要
    ascii_download_enable=YES 是否启用下载的ascii传输方式 需要
    ftpd_banner=Welcome to blah FTP service. 用户连接服务器后显示信息，显示信息可以随便写
    #deny_email_enable=YES 是否允许某些匿名用户使用邮件地址（默认的）
    max_clients=10 #FTP服务器最大承载用户,本人设为10
    max_per_ip=5 #限制每个IP的进程
    local_max_rate=256000 #最大传输速率(b/s)
    #hide_ids=YES #是否隐藏文件的所有者和组信息，不隐藏
    idle_session_timeout= 3000 #空闲（发呆）用户会话的超时时间，若是超出这时间没有数据的传送或是指令的输入，则会强迫断线。单位为秒，默认值为300。
    下面是用来虚拟用户登录的
    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    guest_enable=YES 使用虚拟用户
    guest_username=virtual 将虚拟用户等同本地用户 virtual
    user_config_dir=/etc/vsftpd/vsftpd_user_conf 虚拟用户配置文件夹
    pam_service_name=vsftpd.vu 虚拟用户加密设置
    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

其他未列出来的视情况自己斟酌，如不明白清保持原样。

关于配置文件修改一定注意一下几点：
1、配置前最好把配置文件备份一下， sudo cp /etc/vsftpd.conf /etc/vsftpd.conf.back，以备配错了恢复
2、所有配置语句后面不要有空行，我自己的配置很严格连空格都没有，把所有没必要的注释尤其是汉语注释全删除，如果配置语句后面或前面有空行或空格会报错
3、为了防止出现第 2 点中的错误，请尽量无比自己一条条改写配置语句，不要从网上拷贝，很容易多空格，多回车
4、不清楚自己是否需要的请保持配置文件原样。
配置完了请重启 ftp,看是否报错，是否可以连接，如报错或不能连接请自己检查配置文件，如果还不能启动，报错，很有可能是配置文件有空格或回车，检查并修改。如果实在还不行，恢复配置文件，重新一条条验证配置。
4、使用虚拟用户登录（文本法）
此实现有很多方法，主要的是文本法和数据库法，其实大同小异，本文使用文本法。
此处涉及到两个概念，一个是本地用户，一个是虚拟用户。
本地用户就是 linux 实实在在的一个用户，如 root，你的登录用户等等都是本地用户。
虚拟用户不是 linux 上的用户，只是自己起的虚拟用户，用来使用 ftp 的，虚拟用户必须关联到一个本地用户上。
虚拟用户的实现可以参考官方http://wiki.ubuntu.org.cn/Vsftpd%E8%99%9A%E6%8B%9F%E7%94%A8%E6%88%B7%E8%...，但是里面有错误，请注意甄别。官方的多错，没天理呀，我就是因为这个错误搞了好久，错误很简单，就是其中“为虚拟用户创建本地系统用户 ”那块本地系统用户建立错了，应该是 virtual,既改为：
sudo useradd virtual -d /home/vsftpd -s /bin/false
sudo chown virtual:virutal /home/vsftpd
当然这个用户名字你也可以改，但是此处的名字必须和配置文件 vsftpd.conf 中 guest_username=virtual 项的参数相同。
………………………………………………………………………………………………
下面是我的配置过程,结合官方的那个网址看啊，大部分相同，略有不同：

首先先新建文件夹
sudo mkdir /home/vsftpd
cd /home/vsftpd
sudo mkdir down upload wsn

### （1）、创建虚拟用户数据库

新建 loguser.txt 文件，
$sudo vi /home/loguser.txt

里面输入虚拟用户名和密码，格式如下

nan306
mima1
wsn
mima2
upload
mima3
down
mima4

注意不要多空格和空行，其中 nan306,upload 等为虚拟用户名，另外两行为相应密码
生成数据库
先装一个软件

    $sudo apt-get install db4.7-util

新建一个文件夹，放置配置文件

    sudo mkdir /etc/vsftpd

然后执行

    $sudo db4.7_load -T -t hash -f /home/loguser.txt /etc/vsftpd/vsftpd_login.db

最后设置一下数据库文件的访问权限

    $sudo chmod 600 /etc/vsftpd/vsftpd_login.db

配置 PAM 文件
新建/etc/pam.d/vsftpd.vu

    $sudo vi /etc/pam.d/vsftpd.vu

输入内容如下：

    auth required /lib/security/pam_userdb.so db=/etc/vsftpd_login
    account required /lib/security/pam_userdb.so db=/etc/vsftpd_login

- 我们上一步建立的数据库 vsftpd_login 在此处被使用
- 我们建立的虚拟用户将采用 PAM 进行验证，这是通过/etc/vsftpd.conf 文件中的 语句 pam_service_name=vsftpd.vu 来启用的，稍后你将发现。

### (2)、为虚拟用户建立本地用户

新建一个系统用户 vsftpd，用户家目录为/home/vsftpd, 用户登录终端设为/bin/false(即使之不能登录系统)
udo useradd virtual -d /home/vsftpd -s /bin/false
sudo chown virtual:virutal /home/vsftpd
现在为止，我们的 3 个用户都可以工作了，可是它们的根目录现在都是/home/vsftpd，权限也都一样。 那么怎么才能完成我们预定的目标呢？

### （3）新建 etc/vsftpd_user_conf 文件夹

在上面的配置中，有这么一行 r_config_dir=/etc/vsftpd/vsftpd_user_conf
现在，我们要把各个用户的配置文件放到目录/etc/vsftpd/vsftpd_user_conf 中

    sudo mkdir /etc/vsftpd/vsftpd_user_conf
    cd /etc/vsftpd/vsftpd_user_conf
    sudo touch nan306 wsn upload down

每个文件既为一个配置文件，如 nan306 是个系统管理用户，里面配置如下：
打开 nan306

    sudo vi /etc/vsftpd/vsftpd_user_conf/nan306

里面添加

    write_enable=YES
    anon_world_readable_only=NO
    anon_upload_enable=YES
    anon_mkdir_write_enable=YES
    anon_other_write_enable=YES
    local_root=/home/vsftpd

这里要注意不能有空格，不然登录的时候会提示出错。下同
wsn 用了管理 wsn 文件夹，配置如下：
打开 wsn

    sudo vi /etc/vsftpd/vsftpd_user_conf/wsn

里面添加

    write_enable=YES
    anon_world_readable_only=NO
    anon_upload_enable=YES
    anon_mkdir_write_enable=YES
    anon_other_write_enable=YES
    local_root=/home/vsftpd/wsn

wsn 目录下面还要挂载 down 文件夹的虚拟路径
在 wsn 目录下新建目录 down
sudo mkdir /home/vsftpd/wsn/down
然后系统启动是将/home/vsftpd/down 挂载到/home/vsftpd/wsn/down
改写/etc/rc.local 实现开机启动
sudo vi /etc/rc.local
添加 mount –bind /home/vsftpd/down /home/vsftpd/wsn/down
upload 目录中

    write_enable=YES
    anon_world_readable_only=NO
    anon_upload_enable=YES
    anon_mkdir_write_enable=YES
    anon_other_write_enable=YES
    local_root=/home/vsftpd/upload

    down目录中
    local_root=/home/vsftpd/upload

至此配置完了。重启 vsftpd 我们就可以看到效果了^\_^
/etc/init.d/vsftpd restart
500 OOPS: cannot change directory:
linux 打开 ftp 遇到的问题 500 OOPS: cannot change directory:
google 好多都是 执行这个就 OK setsebool ftpd_disable_trans 1 service vsftpd restart
但是执行的时候遇到这个问题 Could not change active booleans: Invalid boolean
搜了好久终于解决

    setsebool -P ftp_home_dir=1

参考文献：
最初级简单的请看下面两个
http://os.51cto.com/art/201003/189123.htm
http://os.51cto.com/art/200901/106622.htm
高级点的看下面两个
http://www.linuxsir.org/main/?q=node/152#8.4 这个很好的，里面有很多实用设置
http://www.517sou.net/blogview.asp?logID=515#commmark_241 这个很全很全，非常全，非常推进
虚拟用户官方配置，有错误，记得更改

转载地址： http://blog.chinaunix.net/uid-526789-id-3773984.html
