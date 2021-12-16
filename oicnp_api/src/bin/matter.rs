use gray_matter::Matter;
use gray_matter::engine::YAML;


const DATA: &'static str = "---\r\nlayout: post\r\ntitle: '2. 如何创建 Magento 2 控制器'\r\ntags: php magento2\r\nexcerpt: '控制器(Controller)在模块开发和 PHP MVC 框架中都很重要。主要功能是接收请求、处理逻辑并显示页面'\r\n---\r\n\r\n> [Magento 2 开发内容目录](/2020/02/02/0.magento-menu.html)\r\n\r\n> 控制器(Controller)在模块开发和 PHP MVC 框架中都很重要。主要功能是接收请求、处理逻辑并显示页面。\r\n\r\nMagento 2 模块的控制器目录包含有一个或多个动作类(Action Class)文件，动作类有一个 `execute` 方法。控制器主要有两种 *前端控制器* 和 *后台控制器*， 它们工作流都一样，后台控制器有一些小区别，多一个检测权限的方法。\r\n\r\n## 控制器如何工作的？\r\n\r\n控制器接收来自终端（浏览器或控制台）的请求";

fn main() {
    let matter = Matter::<YAML>::new();
    let res = matter.parse(DATA);
    let data = res.data.as_ref().unwrap();
    let layout = &data["layout"].as_string().unwrap();
    let title = &data["title"].as_string().unwrap();
    let tags = &data["tags"].as_string().unwrap();
    let excerpt = &data["excerpt"].as_string().unwrap();
    let content = res.content;
    println!("layout:{}, title:{}, tags:{}, excerpt:{}, content:{}", layout, title, tags, excerpt, content);
}