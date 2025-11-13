---
title: 弹出层图片放大缩小ImageMoveable
description: '实现描述：点击页面相关dom出现弹出框进行相关图片缩放移动等细节查看'
slug: image-modal-moveable

taxonomies:
  categories: ['frontend', 'article']
  tags: ['starzoom', '图片操作']
---

```javascript
/**
 * 弹出层图片放大缩小 ImageMoveable(selector)
 * ========================================
 *
 * 实现描述：点击页面相关dom出现弹出框进行相关图片缩放移动等细节查看
 * dependency:
 *    [jQuery](http://jquery.com/)
 *    [underscore](http://underscorejs.org/)
 *    [Backbone.js](http://backbonejs.org/)
 *
 * author: MeanOfWind
 * QQ: 316841740
 * created time: 2014.6.12
 *
 *
 * dom标签属性：
 *    tagName: any
 *    id: any (not required)
 *    class: any 如果页面有多个图片处理传递class 程序会遍历所有的对象单独实例化
 *    src: any (not required)
 *    data-config: 配置选项 参数格式：为JSON格式 不加花括号 键名不加引号 键值如为字符串加单引号
 *            可选参数：
 *            width:  弹出框及图片宽 default 0
 *            height: 弹出框及图片高 default 0
 *            scale:  图片缩放比例   default 1/8
 *            src:    图片url 如果是img标签存在src属性可不用指定
 *--------------------------------------------
  *
  * 函数调用示例:
  *
  *  (function($){
  *    $(function(){
  *      ImageMoveable( ".img_moveable" );
  *    })
  *  })(jQuery)
  *
  *
  * Tips:
  * 定点缩放 主要算法实现:
  * 缩放后图片位置(top)＝图片的当前位置(top) [+ 或 -] （鼠标按下时距图片左上角距离(disX)＊缩放因子(scale)）
  *
  */
(function($, _, Backbone){
  //弹出框样式
  var css = '.clear{clear:both;height:0;display:block}.imdia{z-index:1000;position:absolute;top:0;left:0}.imdia .dh{cursor:move;height:30px}.imdia .dh .dtit{margin:0;padding:0;display:block;float:left}.imdia .dh .btn{display:block;float:left;cursor:pointer;height:20px;line-height:20px;padding:0 10px;border-radius:3px;font-family:"微软雅黑";font-size:14px;color:#fff;background-color:#0082cb;margin:5px 5px 0}.imdia .dh .btn.zoom{background-color:#ccccd1;opacity:.5}.imdia .dh .btn.active{background-color:#0082cb;opacity:1}.imdia .dh .close{float:right}.imdia .dcon{background-color:#fff;width:100px;height:50px;position:relative;overflow:hidden;border-shadow:10px 10px 5px #888}.imdia .dcon img{display:block;position:absolute;top:0;left:0}#imdiabg{display:block;width:100%;height:100%;background-color:#000;opacity:.7;position:fixed;z-index:900;top:0;left:0}';
  var gnum = 0; //保存当前实例序号 自加用于生成唯一ID
  var diaClassName = "imdia";// 弹出框class
  var bgid = diaClassName + "bg"; //弹出框背景层ID
  var zoomIn = true;//big 放大缩小控制
  /**
   * 弹出框model
   * @type {*}
   */
  var DialogModel = Backbone.Model.extend({
    defaults:{
      id: "dia_",
      width:0,
      height:0,
      diaheight: 0,
      marginLeft:0,
      marginTop:0,
      top:0,
      left:0
    },
    initialize: function(){
      var app = this.get("app");
      //根据当前appid 设置相关参数
      this.set({id: this.get("id")+app.id});
      this.set({diaheight: (this.get("height") + 30)});
      this.setPosition();
    },
    setPosition: function(){
      var scrolltop = $(window).scrollTop();
      var winh = $(window).height();
      var top = winh&gt;this.get("height")? ((winh-this.get("height"))/2) : scrolltop;
      var scrollleft = $(window).scrollLeft();
      var winw = $(window).width();
      var left = winw&gt;this.get("width")? (winw-this.get("width"))/2 : scrollleft;
      this.set({top: top + scrolltop});
      this.set({left: left + scrollleft});
    }
  });
  /**
   * 图片model
   * @type {*}
   */
  var ImageModel = Backbone.Model.extend({
    defaults: {
      id: "img_",
      width:0,
      height:0,
      top:0,
      left:0,
      scale: 1/8, //
      src: ''
    },
    initialize: function(){
      var app = this.get("app");
      //根据当前appid 设置相关参数
      this.set({id: this.get("id")+app.id});
      this.set({scale: eval(this.get("scale"))});
    }
  });
  /**
   * dialog view
   * @type {*}
   */
  var DialogView = Backbone.View.extend({
    tagName: "div",
    className: diaClassName,
    mouseDown: false, //鼠标是否按下
    mouseMove: false,
    disX: 0, //鼠标按下位置和dialog left距离
    disY: 0,
    //弹出框html模板
    template: '&lt;div class="dh"&gt;\
                  &lt;h4 class="dtit"&gt;&lt;/h4&gt;\
                  &lt;div class="btn zoom zoom_in active"&gt;放大&lt;/div&gt;\
                  &lt;div class="btn zoom zoom_out"&gt;缩小&lt;/div&gt;\
                  &lt;div class="btn close"&gt;关闭&lt;/div&gt;&lt;div class="clear"&gt;&lt;/div&gt;\
                &lt;/div&gt;\
                &lt;div class="dcon"&gt;&lt;/div&gt;\
                &lt;div class="db"&gt;&lt;/div&gt;&lt;div class="clear"&gt;&lt;/div&gt;',
    initialize: function(){
      _.bindAll(this, "setPosition","dh_mousemove");
      this.listenTo(this.model,"change",this.setPosition); //监听model　change
      $("body").mousemove(this.dh_mousemove); //绑定鼠标move事件
    },
    events: {
      "click .close": "diaClose",
      "mousedown .dh": "dh_mousedown",
      "mouseup .dh": "dh_mouseup",
      "click .zoom_in": "zoom_in",
      "click .zoom_out": "zoom_out"
      // "mousemove .dh": "dh_mousemove"
    },
    render: function(){
      this.$el.attr("id",this.model.get("id"));
      this.$el.html(this.template);
      this.$el.find(".dcon").append(this.model.get("app").imageView.el);
      this.$el.css({ //dia 样式
        display:"none",
        left: this.model.get("left")+ "px",
        top: this.model.get("top") + "px",
        width:this.model.get("width") + "px",
        height: this.model.get("diaheight") + "px"
      });//内容样式
      this.$el.find(".dcon").css({
        width:this.model.get("width") + "px",
        height: this.model.get("height") + "px"
      });
      //console.log(this)
      return this;
    },
    //关闭
    diaClose: function(){
      this.hide();
    },
    show: function(){
      $("#"+bgid).css("display","block");
      this.$el.show();
    },
    hide: function(){
      $("#"+bgid).css("display","none");
      this.$el.hide();
    },
    setPosition: function(){
      this.$el.css({
        top: this.model.get("top") + "px",
        left: this.model.get("left") + "px"
      });
    },
    //处理托动
    dh_mousedown: function(e){
      this.mouseDown = true;
      this.disX = Math.abs(e.pageX - this.model.get("left"));
      this.disY = Math.abs(e.pageY - this.model.get("top"));
      //console.log(this.disX);
      //console.log(this.mouseDownX)
    },
    dh_mouseup: function(){
      this.mouseDown = false;
      this.mouseMove = false;
    },
    dh_mousemove: function(e){
      this.mouseMove = true;
      //console.log(3)
      if(this.mouseDown){
        var x = e.pageX; //当前鼠标位置
        var y = e.pageY;
        this.model.set({top: y-this.disY});
        this.model.set({left: x-this.disX});
        //console.log(x-this.mouseDownX)
      }
    },
    //放大
    zoom_in: function(e){
      zoomIn = true;// 全局变量
      //改变button class切换对应颜色
      this.$el.find(".zoom").removeClass("active");
      $(e.target).addClass("active");
    },
    //缩小
    zoom_out: function(e){
      zoomIn = false;
      this.$el.find(".zoom").removeClass("active");
      $(e.target).addClass("active");
    }
  });


  /**
   * 图片view类
   * @type {*}
   */
  var ImageView = Backbone.View.extend({
    tagName:"img",
    disX:0,   //鼠标按下时的点 离 图片左上角距离
    disY:0,
    dconX: 0,  //图片外层div
    dconY: 0,
    mouseDown: false, //鼠标没有按下
    mouseMove: false, //鼠标没有移动
    initialize: function(){
      _.bindAll(this, "img_mouse_move","setPosition");
      this.$el.attr({//设置当前image  dom id及src值
        id: this.model.get("id"),
        src: this.model.get('src')
      });
      this.$el.css({
        width: this.model.get("width") + "px",
        height: this.model.get("height") + "px"
      });
      //监听鼠标移动事件
      $(document).mousemove(this.img_mouse_move);
      this.listenTo(this.model,"change",this.setPosition); //监听model　change
    },
    render: function(){
      return this;
    },
    events: {
      "mousedown": "imgmousedown",//鼠标按下
      "mouseup": "imgmouseup"     //鼠标弹起
    },
    //鼠标按下具体处理
    imgmousedown: function(e){
      this.mouseDown = true;
      var $con = this.model.get("app").dialogView.$el.find(".dcon");
      //dcon 左上角相对屏幕
      this.dconX = $con.offset().left;
      this.dconY = $con.offset().top;
      //图片相对屏幕
      var imgY = this.$el.offset().top;
      var imgX = this.$el.offset().left;
      //按下时鼠标位置
      var mouseX = e.pageX;
      var mouseY = e.pageY;
      // 计算鼠标距离图片左上角距离
      this.disX = Math.abs(mouseX - imgX);
      this.disY = Math.abs(mouseY - imgY);
      //console.log(disx);
      //console.log(this.$el.offset())
    },
    /**
     * 鼠标弹起事件具体处理
     * 如果鼠标按下并未移动则进行图片缩放操作
     */
    imgmouseup: function(){
      //console.log(2)
      if(this.mouseDown &amp;&amp; !this.mouseMove){
        var w = this.model.get("width");//图片当前尺寸
        var h = this.model.get("height");
        var scale = this.model.get("scale");  //绽放比例
        var width, height,top=this.model.get("top"),//图片当前位置
            left=this.model.get("left");
        //定点缩放 主要算法实现
        //缩放后图片位置(top) ＝  图片的当前位置(top) [+ 或 -] （鼠标按下时距图片左上角距离(disX)＊缩放因子(scale)）
        if(zoomIn){ //放大
          width = w+w*scale;
          height = h+h*scale;
          top = top - this.disY*scale;
          left = left - this.disX*scale;
        }else{
          width = w-w*scale;
          height = h-h*scale;
          top = top + this.disY*scale;
          left = left + this.disX*scale;
        }
        //重设图片model数据
        this.model.set({
          "width":width,
          "height": height,
          top:top,
          left: left
        });
      }
      this.mouseDown = false;
      this.mouseMove = false;
    },
    //鼠标移动具体处理
    img_mouse_move: function(e){
      var top,left;
      if(this.mouseDown){
        this.mouseMove = true;
        //或取图片相对于容器div.dcon 的top left
        //e.pageX: 事件触发时鼠标相对于document的坐标
        //算法： 图片位置＝鼠标坐标－容器坐标－鼠标在当前图片坐标系的位置
        top = e.pageY - this.dconY - this.disY;
        left = e.pageX - this.dconX - this.disX;
        //console.log(left)
        this.model.set({top: top});
        this.model.set({left: left});
      }
    },
    //根据model值重设当前图片位置及尺寸
    setPosition: function(){
      this.$el.css({
        top: this.model.get("top") + "px",
        left: this.model.get("left") + "px",
        width: this.model.get("width") + "px",
        height: this.model.get("height") + "px"
      });
    }
  });

  /**
   * 主控制类 实例化相关资源
   * @type {*}
   */
  var App = Backbone.View.extend({
    initialize: function(options){
      _.bindAll(this,"winScroll");
      this.id = "g_img_idx_" + gnum++; //设置当前操作对象单一ＩＤ
      options.data.app = this;   //当前引用到对应实例
      //Model实例
      this.dialogModel = new DialogModel(options.data);  //弹出框model
      this.imageModel = new ImageModel(options.data);  //图片model
      //View 实例化
      this.imageView = new ImageView({model:this.imageModel});//图片view
      this.dialogView = new DialogView({model:this.dialogModel});
      //console.log(options);
      //console.log(this.imageView.render().el);
      //console.log(this.el)
      $(window).scroll(this.winScroll);
      this.render();
    },
    render: function(){
      $("body").append(this.dialogView.render().el);
    },
    events:{
      "click": "elClick"
    },
    //当前对象点击
    elClick: function(){
      this.dialogView.show();
    },
    //预留window sroll事件
    winScroll: function(){
      this.dialogModel.setPosition();
    }
  });

  //工具函数
  // parse data-config attr string to json
  function attrToJson(data_config){
    var arr = data_config.split(","); //split to array
    for(var i = 0; i&lt;arr add return add andd parse to json param selector constructor imagemoveable="function(selector){" return foreach every selector the starter function var var data="attrToJson(data_config);" var src="src;" new bg mask var&gt;&lt;/div&gt;";
    var $css = "&lt;style&gt;"+ css +"&lt;/style&gt;";
    $("body").append(bg).append($css);//添加弹出层背景 及对应CSS样式
  }
})(jQuery, _, Backbone);
```
